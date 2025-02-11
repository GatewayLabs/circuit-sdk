use serde::{Deserialize, Serialize};

use crate::int::GarbledInt;
use ruint::Uint;
use std::fmt::Display;
use std::marker::PhantomData;

pub type GarbledBoolean = GarbledUint<1>;
pub type GarbledBit = GarbledUint<1>;
pub type GarbledUint2 = GarbledUint<2>;
pub type GarbledUint4 = GarbledUint<4>;
pub type GarbledUint8 = GarbledUint<8>;
pub type GarbledUint16 = GarbledUint<16>;
pub type GarbledUint32 = GarbledUint<32>;
pub type GarbledUint64 = GarbledUint<64>;
pub type GarbledUint128 = GarbledUint<128>;
pub type GarbledUint160 = GarbledUint<160>;
pub type GarbledUint256 = GarbledUint<256>;
pub type GarbledUint512 = GarbledUint<512>;
pub type GarbledUint1024 = GarbledUint<1024>;

// Define a new type Uint<N>
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GarbledUint<const N: usize> {
    pub bits: Vec<bool>,              // Store the bits of the unsigned integer
    _phantom: PhantomData<[bool; N]>, // PhantomData to ensure the N bit size
}

impl<const N: usize> GarbledUint<N> {
    pub fn zero() -> Self {
        GarbledUint::new(vec![false; N])
    }

    pub fn one() -> Self {
        GarbledUint::new(vec![true; N])
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }
}

impl<const N: usize> Display for GarbledUint<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u128::from(self.clone()))
    }
}

// Implement Uint<N>
impl<const N: usize> GarbledUint<N> {
    // Constructor for GarbledUint<N> from a boolean vector
    pub fn new(bits: Vec<bool>) -> Self {
        //assert_eq!(bits.len(), N, "The number of bits must be {}", N);
        GarbledUint {
            bits,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<GarbledInt<N>> for GarbledUint<N> {
    fn from(uint: GarbledInt<N>) -> Self {
        // Directly copy the bits from the unsigned Uint<N> to the signed GarbledInt<N>
        GarbledUint {
            bits: uint.bits,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<&GarbledInt<N>> for GarbledUint<N> {
    fn from(int: &GarbledInt<N>) -> Self {
        GarbledUint {
            bits: int.bits.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<bool> for GarbledUint<N> {
    fn from(value: bool) -> Self {
        GarbledUint::new(vec![value])
    }
}

impl<const N: usize> From<u8> for GarbledUint<N> {
    fn from(value: u8) -> Self {
        assert!(
            N <= 8,
            "GarbledUint<N> can only support up to 8 bits for u8"
        );

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }
}

impl<const N: usize> From<u16> for GarbledUint<N> {
    fn from(value: u16) -> Self {
        assert!(
            N <= 16,
            "GarbledUint<N> can only support up to 16 bits for u16"
        );

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }
}

impl<const N: usize> From<u32> for GarbledUint<N> {
    fn from(value: u32) -> Self {
        assert!(
            N <= 32,
            "GarbledUint<N> can only support up to 32 bits for u32"
        );

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }
}

impl<const N: usize> From<u64> for GarbledUint<N> {
    fn from(value: u64) -> Self {
        assert!(
            N <= 64,
            "GarbledUint<N> can only support up to 64 bits for u64"
        );

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }
}

impl<const N: usize> From<u128> for GarbledUint<N> {
    fn from(value: u128) -> Self {
        assert!(
            N <= 128,
            "GarbledUint<N> can only support up to 128 bits for u128"
        );

        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }
}

impl<const N: usize> From<GarbledUint<N>> for bool {
    fn from(guint: GarbledUint<N>) -> Self {
        guint.bits[0]
    }
}

impl<const N: usize> From<GarbledUint<N>> for u8 {
    fn from(guint: GarbledUint<N>) -> Self {
        assert!(
            N <= 8,
            "GarbledUint<N> can only be converted to u8 if N <= 8"
        );

        let mut value: u8 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledUint<N>> for u16 {
    fn from(guint: GarbledUint<N>) -> Self {
        assert!(
            N <= 16,
            "GarbledUint<N> can only be converted to u16 if N <= 16"
        );

        let mut value: u16 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledUint<N>> for u32 {
    fn from(guint: GarbledUint<N>) -> Self {
        assert!(
            N <= 32,
            "GarbledUint<N> can only be converted to u32 if N <= 32"
        );

        let mut value: u32 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledUint<N>> for u64 {
    fn from(guint: GarbledUint<N>) -> Self {
        assert!(
            N <= 64,
            "GarbledUint<N> can only be converted to u64 if N <= 64"
        );

        let mut value: u64 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledUint<N>> for u128 {
    fn from(guint: GarbledUint<N>) -> Self {
        assert!(
            N <= 128,
            "Uint<N> can only be converted to u128 if N <= 128"
        );

        let mut value: u128 = 0;
        for (i, &bit) in guint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

// Implement ruint::Uint support for GarbledUint<N>
impl<const BITS: usize, const LIMBS: usize> Into<Uint<BITS, LIMBS>> for GarbledUint<BITS> {
    fn into(self) -> Uint<BITS, LIMBS> {
        let mut value = Uint::<BITS, LIMBS>::ZERO;
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value.set_bit(i, bit);
            }
        }

        value
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for GarbledUint<BITS> {
    fn from(uint: Uint<BITS, LIMBS>) -> Self {
        let mut bits = Vec::with_capacity(BITS);
        for i in 0..BITS {
            bits.push(uint.bit(i));
        }

        GarbledUint::new(bits)
    }
}

#[cfg(test)]
mod tests {
    use ruint::aliases::{U128, U8};

    use crate::uint::GarbledUint;

    #[test]
    fn test_from_ruint_to_garbleduint() {
        let value = U8::from(255);
        let uint = GarbledUint::from(value.clone());
        assert_eq!(U8::from(uint), value);
    }

    #[test]
    fn test_from_garbleduint_8_to_ruint() {
        let guint = GarbledUint::<8>::from(255u8);
        let uint8 = U8::from(guint.clone());
        assert_eq!(GarbledUint::from(uint8), guint);
    }

    #[test]
    fn test_from_garbleduint_128_to_ruint() {
        let guint = GarbledUint::<128>::from(255u128);
        let uint128 = U128::from(guint.clone());
        assert_eq!(GarbledUint::from(uint128), guint);
    }

    #[test]
    fn test_garbled_uint_from_bool() {
        let uint = GarbledUint::<1>::from(true);
        assert_eq!(bool::from(uint), true);
    }
}
