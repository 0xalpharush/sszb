use crate::{
    read_offset_from_slice, sanitize_offset, ssz_decode_variable_length_items, DecodeError,
    SszbDecode, SszbEncode, TryFromIter, BYTES_PER_LENGTH_OFFSET,
};
use bytes::buf::{Buf, BufMut};
use ghilhouse::{Error as GhilhouseError, List, Value, Vector};
use itertools::process_results;
use typenum::Unsigned;

impl<T, N> TryFromIter<T> for List<T, N>
where
    T: Value + SszbDecode,
    N: Unsigned,
{
    type Error = GhilhouseError;

    fn try_from_iter(iter: impl Iterator<Item = T>) -> Result<Self, Self::Error> {
        List::try_from_iter(iter)
    }
}

impl<T, N> TryFromIter<T> for Vector<T, N>
where
    T: Value + SszbDecode,
    N: Unsigned,
{
    type Error = GhilhouseError;

    fn try_from_iter(iter: impl Iterator<Item = T>) -> Result<Self, Self::Error> {
        Vector::try_from_iter(iter)
    }
}

impl<T: SszbEncode + Value, N: Unsigned> SszbEncode for List<T, N> {
    fn is_ssz_static() -> bool {
        false
    }

    fn ssz_fixed_len() -> usize {
        BYTES_PER_LENGTH_OFFSET
    }

    fn ssz_max_len() -> usize {
        T::ssz_max_len() * N::to_usize()
    }

    fn sszb_bytes_len(&self) -> usize {
        if <T as SszbEncode>::is_ssz_static() {
            <T as SszbEncode>::ssz_fixed_len() * self.len()
        } else {
            let mut len = self.iter().map(|item| SszbEncode::sszb_bytes_len(item)).sum();
            len += BYTES_PER_LENGTH_OFFSET * self.len();
            len
        }
    }

    fn ssz_write_fixed(&self, offset: &mut usize, buf: &mut impl BufMut) {
        buf.put_slice(&offset.to_le_bytes()[0..BYTES_PER_LENGTH_OFFSET]);
        *offset += self.sszb_bytes_len();
    }

    fn ssz_write_variable(&self, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write(&self, buf: &mut impl BufMut) {
        if T::is_ssz_static() {
            for item in self {
                item.ssz_write(buf);
            }
        } else {
            let offset = &mut (self.len() * BYTES_PER_LENGTH_OFFSET);
            for item in self {
                item.ssz_write_fixed(offset, buf);
            }
            for item in self {
                item.ssz_write(buf);
            }
        }
    }
}

impl<T: SszbDecode + Value, N: Unsigned> SszbDecode for List<T, N> {
    fn is_ssz_static() -> bool {
        false
    }

    fn ssz_fixed_len() -> usize {
        BYTES_PER_LENGTH_OFFSET
    }

    fn ssz_max_len() -> usize {
        if T::is_ssz_static() {
            <T as SszbDecode>::ssz_fixed_len() * N::to_usize()
        } else {
            let mut len = T::ssz_max_len() * N::to_usize();
            len += BYTES_PER_LENGTH_OFFSET * N::to_usize();
            len
        }
    }

    fn ssz_read(
        _fixed_bytes: &mut impl Buf,
        variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let max_len = N::to_usize();

        // Lists are always stored in the dynamic section at the end
        // So we only check if the variable bytes are empty
        if !variable_bytes.has_remaining() {
            Ok(Self::empty())
        } else if T::is_ssz_static() {
            let num_items = variable_bytes
                .remaining()
                .checked_div(<T as SszbDecode>::ssz_fixed_len())
                .ok_or(DecodeError::ZeroLengthItem)?;

            if num_items > max_len {
                return Err(DecodeError::BytesInvalid(format!(
                    "List of {} items exceeds maximum of {}",
                    num_items, max_len
                )));
            }

            // let bytes = variable_bytes.copy_to_bytes(num_items * <T as SszbDecode>::ssz_fixed_len());

            process_results(
                variable_bytes
                    .chunk()
                    .chunks_exact(<T as SszbDecode>::ssz_fixed_len())
                    .map(|chunk| <T as SszbDecode>::from_ssz_bytes(chunk)),
                |iter| List::try_from_iter(iter),
            )?
            .map_err(|e| DecodeError::BytesInvalid(format!("Error processing results: {:?}", e)))
        } else {
            // we move over variable_bytes to var_offsets (of type Bytes) since it has more methods for us to work with
            // let mut var_offsets = variable_bytes.copy_to_bytes(variable_bytes.remaining());
            let var_offsets = variable_bytes.chunk();

            let first_offset = read_offset_from_slice(&var_offsets[0..BYTES_PER_LENGTH_OFFSET])?;
            sanitize_offset(
                first_offset,
                None,
                var_offsets[BYTES_PER_LENGTH_OFFSET..].len(),
                Some(first_offset),
            )?;
            if first_offset % BYTES_PER_LENGTH_OFFSET != 0 || first_offset < BYTES_PER_LENGTH_OFFSET
            {
                return Err(DecodeError::InvalidListFixedBytesLen(first_offset));
            }

            // get how many items are in the list by reading the offset (only way to deduce in variable lists)
            let num_items = first_offset / BYTES_PER_LENGTH_OFFSET;

            // if length exceeds expected max_len then revert
            if num_items > max_len {
                return Err(DecodeError::BytesInvalid(format!(
                    "Variable length list of {} items exceeds maximum of {:?}",
                    num_items, max_len
                )));
            }

            // var_offsets now only contains the offsets, and var_items contains the list items (bytes)
            // let mut var_items = var_offsets.split_off(num_items * BYTES_PER_LENGTH_OFFSET);
            let mut var_items = &var_offsets[(num_items * BYTES_PER_LENGTH_OFFSET)..];
            ssz_decode_variable_length_items(
                &var_offsets[..(num_items * BYTES_PER_LENGTH_OFFSET)],
                &mut var_items,
            )
        }
    }
}

impl<T: SszbEncode + Value, N: Unsigned> SszbEncode for Vector<T, N> {
    fn is_ssz_static() -> bool {
        T::is_ssz_static()
    }

    fn ssz_fixed_len() -> usize {
        if <T as SszbEncode>::is_ssz_static() {
            <T as SszbEncode>::ssz_fixed_len() * N::to_usize()
        } else {
            BYTES_PER_LENGTH_OFFSET
        }
    }

    fn ssz_max_len() -> usize {
        T::ssz_max_len() * N::to_usize()
    }

    fn sszb_bytes_len(&self) -> usize {
        if <T as SszbEncode>::is_ssz_static() {
            <T as SszbEncode>::ssz_fixed_len() * N::to_usize()
        } else {
            let mut len = self.iter().map(|item| SszbEncode::sszb_bytes_len(item)).sum();
            len += BYTES_PER_LENGTH_OFFSET * N::to_usize();
            len
        }
    }

    fn ssz_write_fixed(&self, offset: &mut usize, buf: &mut impl BufMut) {
        if T::is_ssz_static() {
            self.ssz_write(buf);
        } else {
            buf.put_slice(&offset.to_le_bytes()[0..BYTES_PER_LENGTH_OFFSET]);
            *offset += self.sszb_bytes_len();
        }
    }

    fn ssz_write_variable(&self, buf: &mut impl BufMut) {
        if !T::is_ssz_static() {
            self.ssz_write(buf);
        }
    }

    fn ssz_write(&self, buf: &mut impl BufMut) {
        if T::is_ssz_static() {
            for item in self {
                item.ssz_write(buf);
            }
        } else {
            let offset = &mut (self.len() * BYTES_PER_LENGTH_OFFSET);
            for item in self {
                item.ssz_write_fixed(offset, buf);
            }
            for item in self {
                item.ssz_write(buf);
            }
        }
    }
}

impl<T: SszbDecode + Value, N: Unsigned> SszbDecode for Vector<T, N> {
    fn is_ssz_static() -> bool {
        T::is_ssz_static()
    }

    fn ssz_fixed_len() -> usize {
        if T::is_ssz_static() {
            <T as SszbDecode>::ssz_fixed_len() * N::to_usize()
        } else {
            BYTES_PER_LENGTH_OFFSET
        }
    }

    fn ssz_max_len() -> usize {
        if T::is_ssz_static() {
            <T as SszbDecode>::ssz_fixed_len() * N::to_usize()
        } else {
            let mut len = T::ssz_max_len() * N::to_usize();
            len += BYTES_PER_LENGTH_OFFSET * N::to_usize();
            len
        }
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = N::to_usize();

        // Vectors are either static, in which case the data is in the fixed bytes section
        // or it's dynamic and the data is in variable bytes.
        // The vector is empty if both sections are empty.
        if !(fixed_bytes.has_remaining() || variable_bytes.has_remaining()) {
            Ok(Self::try_from(List::empty()).map_err(|e| {
                DecodeError::BytesInvalid(format!("Error decoding empty vector: {:?}", e))
            })?)
        } else if T::is_ssz_static() {
            // T is static, so data resides in fixed_bytes
            if fixed_bytes.remaining() < len * <T as SszbDecode>::ssz_fixed_len() {
                return Err(DecodeError::BytesInvalid(format!(
                    "Vector of {} items not equal to length {}",
                    fixed_bytes
                        .remaining()
                        .checked_div(<T as SszbDecode>::ssz_fixed_len())
                        .unwrap(),
                    len
                )));
            }

            // create slice of length `len * T::ssz_fixed_len`
            // let bytes = fixed_bytes.copy_to_bytes(len * <T as SszbDecode>::ssz_fixed_len());
            let bytes = &fixed_bytes.chunk()[..(len * <T as SszbDecode>::ssz_fixed_len())];

            let res = process_results(
                bytes
                    .chunks_exact(<T as SszbDecode>::ssz_fixed_len())
                    .map(|chunk| <T as SszbDecode>::from_ssz_bytes(chunk)),
                |iter| Vector::try_from_iter(iter),
            )?
            .map_err(|e| DecodeError::BytesInvalid(format!("Error processing results: {:?}", e)));

            fixed_bytes.advance(len * <T as SszbDecode>::ssz_fixed_len());
            res
        } else {
            // T is not static so data resides in variable_bytes
            // let mut var_offsets = variable_bytes.copy_to_bytes(variable_bytes.remaining());
            // let mut var_items = var_offsets.split_off(len * BYTES_PER_LENGTH_OFFSET);
            // ssz_decode_variable_length_items(var_offsets, &mut var_items)

            let var_offsets = &variable_bytes.chunk()[..(len * BYTES_PER_LENGTH_OFFSET)];
            let mut var_items = &variable_bytes.chunk()[(len * BYTES_PER_LENGTH_OFFSET)..];
            ssz_decode_variable_length_items(var_offsets, &mut var_items)
        }
    }
}
