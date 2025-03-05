use crate::{SszbEncode, BYTES_PER_LENGTH_OFFSET};
use alloy_primitives::{Address, Bloom, FixedBytes, U128, U256};
use bytes::buf::BufMut;
use ethereum_types::{H160, H256, H32};
use milhouse::{List as PersistentList, Value, Vector as PersistentVector};
use paste::paste;
use ssz_types::{BitList, BitVector, FixedVector, VariableList};
use std::sync::Arc;
use typenum::Unsigned;

macro_rules! uint_sszb_encode {
    ($type: ident, $bit_size: expr) => {
        impl SszbEncode for $type {
            fn is_ssz_static() -> bool {
                true
            }

            fn ssz_fixed_len() -> usize {
                $bit_size / 8
            }

            fn sszb_bytes_len(&self) -> usize {
                $bit_size / 8
            }

            fn ssz_max_len() -> usize {
                $bit_size / 8
            }

            fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
                self.ssz_write(buf);
            }

            fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

            fn ssz_write(&self, buf: &mut impl BufMut) {
                paste! { buf.[<put_ $type _le>](*self) }
            }
        }
    };
}

impl SszbEncode for u8 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        1
    }

    fn sszb_bytes_len(&self) -> usize {
        1
    }

    fn ssz_max_len() -> usize {
        1
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_u8(*self);
    }
}

uint_sszb_encode!(u16, 16);
uint_sszb_encode!(u32, 32);
uint_sszb_encode!(u64, 64);
uint_sszb_encode!(u128, 128);

impl SszbEncode for bool {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        1
    }

    fn ssz_max_len() -> usize {
        1
    }

    fn sszb_bytes_len(&self) -> usize {
        1
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(&(*self as u8).to_le_bytes());
    }
}

impl<const N: usize> SszbEncode for [u8; N] {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        N
    }

    fn ssz_max_len() -> usize {
        N
    }

    fn sszb_bytes_len(&self) -> usize {
        N
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(self.as_slice());
    }
}

impl SszbEncode for Address {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        20
    }

    fn ssz_max_len() -> usize {
        20
    }

    fn sszb_bytes_len(&self) -> usize {
        20
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(self.as_slice());
    }
}

impl<const N: usize> SszbEncode for FixedBytes<N> {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        N
    }

    fn ssz_max_len() -> usize {
        N
    }

    fn sszb_bytes_len(&self) -> usize {
        N
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.0)
    }
}

impl SszbEncode for Bloom {
    fn is_ssz_static() -> bool {
        true
    }

    fn sszb_bytes_len(&self) -> usize {
        256
    }

    fn ssz_fixed_len() -> usize {
        256
    }

    fn ssz_max_len() -> usize {
        256
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.0 .0)
    }
}

impl SszbEncode for U256 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        32
    }

    fn sszb_bytes_len(&self) -> usize {
        32
    }

    fn ssz_max_len() -> usize {
        32
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(self.as_le_slice());
    }
}

impl SszbEncode for U128 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        16
    }

    fn sszb_bytes_len(&self) -> usize {
        16
    }

    fn ssz_max_len() -> usize {
        16
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(self.as_le_slice());
    }
}

impl SszbEncode for H32 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        4
    }

    fn sszb_bytes_len(&self) -> usize {
        4
    }

    fn ssz_max_len() -> usize {
        4
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(self.as_bytes());
    }
}

impl SszbEncode for H160 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        20
    }

    fn sszb_bytes_len(&self) -> usize {
        20
    }

    fn ssz_max_len() -> usize {
        20
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(self.as_bytes());
    }
}

impl SszbEncode for H256 {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        32
    }

    fn sszb_bytes_len(&self) -> usize {
        32
    }

    fn ssz_max_len() -> usize {
        32
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(self.as_bytes());
    }
}

impl<N: Unsigned + Clone> SszbEncode for BitVector<N> {
    fn is_ssz_static() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        std::cmp::max(1, (N::to_usize() + 7) / 8)
    }

    fn sszb_bytes_len(&self) -> usize {
        self.as_slice().len()
    }

    fn ssz_max_len() -> usize {
        std::cmp::max(1, (N::to_usize() + 7) / 8)
    }

    fn ssz_write_fixed(&self, _offset: &mut usize, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write_variable(&self, _buf: &mut impl BufMut) {}

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.clone().into_bytes());
    }
}

impl<N: Unsigned + Clone> SszbEncode for BitList<N> {
    fn is_ssz_static() -> bool {
        false
    }

    fn ssz_fixed_len() -> usize {
        BYTES_PER_LENGTH_OFFSET
    }

    fn sszb_bytes_len(&self) -> usize {
        self.clone().into_bytes().len()
    }

    fn ssz_max_len() -> usize {
        std::cmp::max(1, (N::to_usize() + 7) / 8)
    }

    fn ssz_write_fixed(&self, offset: &mut usize, buf: &mut impl BufMut) {
        // usize can be u32 or u64 depending on the system
        // so we restrict offset to the first BYTES_PER_LENGTH_OFFSET bytes
        buf.put_slice(&offset.to_le_bytes()[0..BYTES_PER_LENGTH_OFFSET]);
        *offset += self.sszb_bytes_len();
    }

    fn ssz_write_variable(&self, buf: &mut impl BufMut) {
        self.ssz_write(buf);
    }

    fn ssz_write(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.clone().into_bytes());
    }
}

impl<T: SszbEncode> SszbEncode for Arc<T> {
    fn is_ssz_static() -> bool {
        T::is_ssz_static()
    }

    fn ssz_fixed_len() -> usize {
        T::ssz_fixed_len()
    }

    fn ssz_max_len() -> usize {
        T::ssz_max_len()
    }

    fn sszb_bytes_len(&self) -> usize {
        self.as_ref().sszb_bytes_len()
    }

    fn ssz_write_fixed(&self, offset: &mut usize, buf: &mut impl BufMut) {
        self.as_ref().ssz_write_fixed(offset, buf);
    }

    fn ssz_write_variable(&self, buf: &mut impl BufMut) {
        self.as_ref().ssz_write_variable(buf);
    }

    fn ssz_write(&self, buf: &mut impl BufMut) {
        self.as_ref().ssz_write(buf);
    }
}

impl<T: SszbEncode + Value, N: Unsigned> SszbEncode for PersistentList<T, N> {
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

impl<T: SszbEncode + Value, N: Unsigned> SszbEncode for PersistentVector<T, N> {
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

impl<T: SszbEncode, N: Unsigned> SszbEncode for VariableList<T, N> {
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

impl<T: SszbEncode, N: Unsigned> SszbEncode for FixedVector<T, N> {
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
