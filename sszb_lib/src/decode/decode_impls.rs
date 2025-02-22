use crate::{
    read_offset_from_buf, read_offset_from_slice, sanitize_offset, DecodeError, SszDecode,
    BYTES_PER_LENGTH_OFFSET,
};
use alloy_primitives::{Address, Bloom, FixedBytes, U128, U256};
use bytes::buf::Buf;
use ethereum_types::{H160, H256, H32};
use first_err::FirstErr;
use itertools::Itertools as _;
use milhouse::{Error as MilhouseError, List as PersistentList, Value, Vector as PersistentVector};
use paste::paste;
use smallvec::ToSmallVec;
use ssz_types::{BitList, BitVector, FixedVector, VariableList};
use typenum::Unsigned;

macro_rules! uint_ssz_decode {
    ($type: ident, $bit_size: expr) => {
        impl SszDecode for $type {
            fn is_ssz_static() -> bool {
                true
            }

            fn ssz_fixed_len() -> usize {
                $bit_size / 8
            }

            fn ssz_max_len() -> usize {
                $bit_size / 8
            }

            fn ssz_read(
                fixed_bytes: &mut impl Buf,
                _variable_bytes: &mut impl Buf,
            ) -> Result<Self, DecodeError> {
                let len = fixed_bytes.remaining();
                let expected = <Self as SszDecode>::ssz_fixed_len();

                if len < expected {
                    Err(DecodeError::InvalidByteLength { len, expected })
                } else {
                    Ok(paste! { fixed_bytes.[<get_ $type _le>]() })
                }
            }
        }
    };
}

impl SszDecode for u8 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        1
    }

    fn ssz_max_len() -> usize {
        1
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(fixed_bytes.get_u8())
        }
    }
}

uint_ssz_decode!(u16, 16);
uint_ssz_decode!(u32, 32);
uint_ssz_decode!(u64, 64);
uint_ssz_decode!(u128, 128);

impl SszDecode for bool {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        1
    }

    fn ssz_max_len() -> usize {
        1
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            match fixed_bytes.get_u8() {
                0 => Ok(false),
                1 => Ok(true),
                num => Err(DecodeError::BytesInvalid(format!(
                    "Out-of-range for boolean: {}",
                    num
                ))),
            }
        }
    }
}

impl SszDecode for Address {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        20
    }

    fn ssz_max_len() -> usize {
        20
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        let mut bytes: [u8; 20] = [0u8; 20];
        fixed_bytes.copy_to_slice(&mut bytes[..]);

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(Self::from(bytes))
        }
    }
}

impl<const N: usize> SszDecode for FixedBytes<N> {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        N
    }

    fn ssz_max_len() -> usize {
        N
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        let mut bytes: [u8; N] = [0u8; N];
        fixed_bytes.copy_to_slice(&mut bytes[..]);

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(Self(bytes))
        }
    }
}

impl SszDecode for Bloom {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        256
    }

    fn ssz_max_len() -> usize {
        256
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        let mut bytes: [u8; 256] = [0u8; 256];
        fixed_bytes.copy_to_slice(&mut bytes[..]);

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(Self::from_slice(&bytes))
        }
    }
}

impl SszDecode for U256 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        32
    }
    fn ssz_max_len() -> usize {
        32
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        let mut bytes: [u8; 32] = [0u8; 32];
        fixed_bytes.copy_to_slice(&mut bytes[..]);

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(Self::from_le_slice(&bytes))
        }
    }
}

impl SszDecode for U128 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        16
    }
    fn ssz_max_len() -> usize {
        16
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        let mut bytes: [u8; 16] = [0u8; 16];
        fixed_bytes.copy_to_slice(&mut bytes[..]);

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(Self::from_le_slice(&bytes))
        }
    }
}

impl SszDecode for H32 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        4
    }
    fn ssz_max_len() -> usize {
        4
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        let mut bytes: [u8; 4] = [0u8; 4];
        fixed_bytes.copy_to_slice(&mut bytes[..]);

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(Self::from_slice(&bytes))
        }
    }
}

impl SszDecode for H160 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        20
    }
    fn ssz_max_len() -> usize {
        20
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        let mut bytes: [u8; 20] = [0u8; 20];
        fixed_bytes.copy_to_slice(&mut bytes[..]);

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(Self::from_slice(&bytes))
        }
    }
}

impl SszDecode for H256 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        32
    }

    fn ssz_max_len() -> usize {
        32
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        let mut bytes: [u8; 32] = [0u8; 32];
        fixed_bytes.copy_to_slice(&mut bytes[..]);

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            Ok(Self::from_slice(&bytes))
        }
    }
}

impl<N: Unsigned + Clone> SszDecode for BitVector<N> {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        std::cmp::max(1, (N::to_usize() + 7) / 8)
    }

    fn ssz_max_len() -> usize {
        std::cmp::max(1, (N::to_usize() + 7) / 8)
    }

    fn ssz_read(
        fixed_bytes: &mut impl Buf,
        _variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let len = fixed_bytes.remaining();
        let expected = <Self as SszDecode>::ssz_fixed_len();

        if len < expected {
            Err(DecodeError::InvalidByteLength { len, expected })
        } else {
            let bytes = fixed_bytes.copy_to_bytes(expected);

            Self::from_bytes(bytes.to_smallvec()).map_err(|e| {
                DecodeError::BytesInvalid(format!("BitVector failed to decode: {:?}", e))
            })
        }
    }
}

impl<N: Unsigned + Clone> SszDecode for BitList<N> {
    fn is_ssz_static() -> bool {
        false
    }

    fn ssz_fixed_len() -> usize {
        BYTES_PER_LENGTH_OFFSET
    }

    fn ssz_max_len() -> usize {
        BYTES_PER_LENGTH_OFFSET + std::cmp::max(1, (N::to_usize() + 7) / 8)
    }

    fn ssz_read(
        _fixed_bytes: &mut impl Buf,
        variable_bytes: &mut impl Buf,
    ) -> Result<Self, DecodeError> {
        let bytes = variable_bytes.copy_to_bytes(variable_bytes.remaining());

        Self::from_bytes(bytes.to_smallvec())
            .map_err(|e| DecodeError::BytesInvalid(format!("BitVector failed to decode: {:?}", e)))
    }
}

impl<T: SszDecode + Value + Default, N: Unsigned> SszDecode for PersistentList<T, N> {
    fn is_ssz_static() -> bool {
        false
    }

    fn ssz_fixed_len() -> usize {
        BYTES_PER_LENGTH_OFFSET
    }

    fn ssz_max_len() -> usize {
        if T::is_ssz_static() {
            <T as SszDecode>::ssz_fixed_len() * N::to_usize()
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
                .checked_div(<T as SszDecode>::ssz_fixed_len())
                .ok_or(DecodeError::ZeroLengthItem)?;

            if num_items > max_len {
                return Err(DecodeError::BytesInvalid(format!(
                    "List of {} items exceeds maximum of {}",
                    num_items, max_len
                )));
            }

            let bytes = variable_bytes.copy_to_bytes(num_items * <T as SszDecode>::ssz_fixed_len());

            // first_err_or_else returns the first error in the iterator or returns Ok(List<T>)
            bytes
                .chunks(<T as SszDecode>::ssz_fixed_len())
                .map(|chunk| <T as SszDecode>::from_ssz_bytes(chunk))
                .first_err_or_else(|iter| match PersistentList::try_from_iter(iter) {
                    Ok(list) => Ok(list),
                    Err(e) => Err(DecodeError::BytesInvalid(format!(
                        "Error processing results: {:?}",
                        e
                    ))),
                })?
        } else {
            // we move over variable_bytes to var_offsets (of type Bytes) since it has more methods for us to work with
            let mut var_offsets = variable_bytes.copy_to_bytes(variable_bytes.remaining());

            let first_offset =
                read_offset_from_buf(&mut var_offsets.slice(0..BYTES_PER_LENGTH_OFFSET))?;
            sanitize_offset(
                first_offset,
                None,
                var_offsets.slice(BYTES_PER_LENGTH_OFFSET..).len(),
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
            let mut var_items = var_offsets.split_off(num_items * BYTES_PER_LENGTH_OFFSET);
            ssz_decode_variable_length_items(var_offsets, &mut var_items)
        }
    }
}

impl<T: SszDecode + Value, N: Unsigned> SszDecode for PersistentVector<T, N> {
    fn is_ssz_static() -> bool {
        T::is_ssz_static()
    }

    fn ssz_fixed_len() -> usize {
        if T::is_ssz_static() {
            <T as SszDecode>::ssz_fixed_len() * N::to_usize()
        } else {
            BYTES_PER_LENGTH_OFFSET
        }
    }

    fn ssz_max_len() -> usize {
        if T::is_ssz_static() {
            <T as SszDecode>::ssz_fixed_len() * N::to_usize()
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
            Ok(Self::try_from(PersistentList::empty()).map_err(|e| {
                DecodeError::BytesInvalid(format!("Error decoding empty vector: {:?}", e))
            })?)
        } else if T::is_ssz_static() {
            // T is static, so data resides in fixed_bytes
            if fixed_bytes.remaining() < len * <T as SszDecode>::ssz_fixed_len() {
                return Err(DecodeError::BytesInvalid(format!(
                    "Vector of {} items not equal to length {}",
                    fixed_bytes
                        .remaining()
                        .checked_div(<T as SszDecode>::ssz_fixed_len())
                        .unwrap(),
                    len
                )));
            }

            // create slice of length `len * T::ssz_fixed_len`
            let bytes = fixed_bytes.copy_to_bytes(len * <T as SszDecode>::ssz_fixed_len());

            bytes
                .chunks(<T as SszDecode>::ssz_fixed_len())
                .map(|chunk| <T as SszDecode>::from_ssz_bytes(chunk))
                .first_err_or_else(|iter| match PersistentVector::try_from_iter(iter) {
                    Ok(list) => Ok(list),
                    Err(e) => Err(DecodeError::BytesInvalid(format!(
                        "Error processing results: {:?}",
                        e
                    ))),
                })?
        } else {
            // T is not static so data resides in variable_bytes
            let mut var_offsets = variable_bytes.copy_to_bytes(variable_bytes.remaining());
            let mut var_items = var_offsets.split_off(len * BYTES_PER_LENGTH_OFFSET);
            ssz_decode_variable_length_items(var_offsets, &mut var_items)
        }
    }
}

impl<T: SszDecode, N: Unsigned> SszDecode for VariableList<T, N> {
    fn is_ssz_static() -> bool {
        false
    }

    fn ssz_fixed_len() -> usize {
        BYTES_PER_LENGTH_OFFSET
    }

    fn ssz_max_len() -> usize {
        if T::is_ssz_static() {
            <T as SszDecode>::ssz_fixed_len() * N::to_usize()
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

        if !variable_bytes.has_remaining() {
            Ok(Self::empty())
        } else if T::is_ssz_static() {
            let num_items = variable_bytes
                .remaining()
                .checked_div(<T as SszDecode>::ssz_fixed_len())
                .ok_or(DecodeError::ZeroLengthItem)?;

            if num_items > max_len {
                return Err(DecodeError::BytesInvalid(format!(
                    "List of {} items exceeds maximum of {}",
                    num_items, max_len
                )));
            }

            let bytes = variable_bytes.copy_to_bytes(num_items * <T as SszDecode>::ssz_fixed_len());

            bytes
                .chunks(<T as SszDecode>::ssz_fixed_len())
                .map(|chunk| <T as SszDecode>::from_ssz_bytes(chunk))
                .first_err_or_else(|iter| match VariableList::try_from_iter(iter) {
                    Ok(list) => Ok(list),
                    Err(e) => Err(DecodeError::BytesInvalid(format!(
                        "Error processing results: {:?}",
                        e
                    ))),
                })?
        } else {
            let mut var_offsets = variable_bytes.copy_to_bytes(variable_bytes.remaining());

            let first_offset =
                read_offset_from_buf(&mut var_offsets.slice(0..BYTES_PER_LENGTH_OFFSET))?;
            sanitize_offset(
                first_offset,
                None,
                var_offsets.slice(BYTES_PER_LENGTH_OFFSET..).len(),
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

            let mut var_items = var_offsets.split_off(num_items * BYTES_PER_LENGTH_OFFSET);
            ssz_decode_variable_length_items(var_offsets, &mut var_items)
        }
    }
}

impl<T: SszDecode, N: Unsigned> SszDecode for FixedVector<T, N> {
    fn is_ssz_static() -> bool {
        T::is_ssz_static()
    }

    fn ssz_fixed_len() -> usize {
        if T::is_ssz_static() {
            <T as SszDecode>::ssz_fixed_len() * N::to_usize()
        } else {
            BYTES_PER_LENGTH_OFFSET
        }
    }

    fn ssz_max_len() -> usize {
        if T::is_ssz_static() {
            <T as SszDecode>::ssz_fixed_len() * N::to_usize()
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

        if !(fixed_bytes.has_remaining() || variable_bytes.has_remaining()) {
            Ok(Self::new(Vec::new()).map_err(|e| {
                DecodeError::BytesInvalid(format!("Error deocoding empty vector: {:?}", e))
            })?)
        } else if T::is_ssz_static() {
            if fixed_bytes.remaining() < len * <T as SszDecode>::ssz_fixed_len() {
                return Err(DecodeError::BytesInvalid(format!(
                    "Vector of {} items not equal to length {}",
                    fixed_bytes
                        .remaining()
                        .checked_div(<T as SszDecode>::ssz_fixed_len())
                        .unwrap(),
                    len
                )));
            }

            // create slice of length `len * T::ssz_fixed_len`
            let bytes = fixed_bytes.copy_to_bytes(len * <T as SszDecode>::ssz_fixed_len());

            bytes
                .chunks(<T as SszDecode>::ssz_fixed_len())
                .map(|chunk| <T as SszDecode>::from_ssz_bytes(chunk))
                .first_err_or_else(|iter| match FixedVector::try_from_iter(iter) {
                    Ok(list) => Ok(list),
                    Err(e) => Err(DecodeError::BytesInvalid(format!(
                        "Error processing results: {:?}",
                        e
                    ))),
                })?
        } else {
            let mut var_offsets = variable_bytes.copy_to_bytes(variable_bytes.remaining());
            let mut var_items = var_offsets.split_off(len * BYTES_PER_LENGTH_OFFSET);
            ssz_decode_variable_length_items(var_offsets, &mut var_items)
        }
    }
}

pub trait TryFromIter<T>: Sized {
    type Error: std::fmt::Debug;

    fn try_from_iter(iter: impl Iterator<Item = T>) -> Result<Self, Self::Error>;
}

impl<T, N> TryFromIter<T> for PersistentList<T, N>
where
    T: Value + SszDecode,
    N: Unsigned,
{
    type Error = MilhouseError;

    fn try_from_iter(iter: impl Iterator<Item = T>) -> Result<Self, Self::Error> {
        PersistentList::try_from_iter(iter)
    }
}

impl<T, N> TryFromIter<T> for VariableList<T, N>
where
    T: SszDecode,
    N: Unsigned,
{
    type Error = DecodeError;

    fn try_from_iter(iter: impl Iterator<Item = T>) -> Result<Self, Self::Error> {
        match VariableList::new(iter.collect()) {
            Ok(list) => Ok(list),
            _ => Err(DecodeError::BytesInvalid(format!(
                "Error processing results"
            ))),
        }
    }
}

impl<T, N> TryFromIter<T> for PersistentVector<T, N>
where
    T: Value + SszDecode,
    N: Unsigned,
{
    type Error = MilhouseError;

    fn try_from_iter(iter: impl Iterator<Item = T>) -> Result<Self, Self::Error> {
        PersistentVector::try_from_iter(iter)
    }
}

impl<T, N> TryFromIter<T> for FixedVector<T, N>
where
    T: SszDecode,
    N: Unsigned,
{
    type Error = DecodeError;

    fn try_from_iter(iter: impl Iterator<Item = T>) -> Result<Self, Self::Error> {
        match FixedVector::new(iter.collect()) {
            Ok(list) => Ok(list),
            _ => Err(DecodeError::BytesInvalid(format!(
                "Error processing results"
            ))),
        }
    }
}

pub fn ssz_decode_variable_length_items<T: SszDecode, L: TryFromIter<T>>(
    var_offsets: bytes::Bytes,
    var_items: &mut impl Buf,
) -> Result<L, DecodeError> {
    if var_offsets.is_empty() && !var_items.has_remaining() {
        return L::try_from_iter(std::iter::empty()).map_err(|e| {
            DecodeError::BytesInvalid(format!("Error trying to collect empty items: {:?}", e))
        });
    }

    // h/t the grandine team for this iter def:
    // https://github.com/grandinetech/grandine/blob/develop/ssz/src/shared.rs#L174
    // This iterator splits offset sections into offset sized chunks (of length BYTES_PER_LENGTH_OFFSET),
    // reads them (maps to Result<usize, Error>), slides over them in windows of size 2,
    // and calculates the length between offsets before decoding an item of that length (the last map).
    // The .chain call is so we don't forget an offset at the end since it stops iterating
    // when the window hits the the last chunk.
    var_offsets
        .chunks_exact(BYTES_PER_LENGTH_OFFSET)
        .map(read_offset_from_slice)
        .chain(core::iter::once(Ok(
            var_offsets.len() + var_items.remaining()
        )))
        .tuple_windows()
        .map(move |(start_result, end_result)| {
            let start = start_result?;
            let end = end_result?;
            let len = end - start;
            let bytes = var_items.copy_to_bytes(len);
            <T as SszDecode>::from_ssz_bytes(bytes.chunk())
        })
        .first_err_or_else(|iter| match L::try_from_iter(iter) {
            Ok(list) => Ok(list),
            Err(e) => Err(DecodeError::BytesInvalid(format!(
                "Error processing results: {:?}",
                e
            ))),
        })?
}
