use ruint::Uint;
use serde::{Deserialize, Serialize};

use crate::uint::GarbledUint;
use std::convert::From;
use std::fmt::Display;
use std::marker::PhantomData;

pub type GarbledInt1 = GarbledInt<1>;
pub type GarbledInt2 = GarbledInt<2>;
pub type GarbledInt4 = GarbledInt<4>;
pub type GarbledInt8 = GarbledInt<8>;
pub type GarbledInt16 = GarbledInt<16>;
pub type GarbledInt32 = GarbledInt<32>;
pub type GarbledInt64 = GarbledInt<64>;
pub type GarbledInt128 = GarbledInt<128>;
pub type GarbledInt160 = GarbledInt<160>;
pub type GarbledInt256 = GarbledInt<256>;
pub type GarbledInt512 = GarbledInt<512>;
pub type GarbledInt1024 = GarbledInt<1024>;

// Define a new type GarbledInt<N>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarbledInt<const N: usize> {
    pub bits: Vec<bool>, // Store the bits of the signed integer (in two's complement form)
    _phantom: PhantomData<[bool; N]>, // PhantomData to ensure the N bit size
}

impl<const N: usize> Display for GarbledInt<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Convert the bits to a signed integer
        match N {
            8 => write!(f, "{}", i8::from(self.clone())),
            16 => write!(f, "{}", i16::from(self.clone())),
            32 => write!(f, "{}", i32::from(self.clone())),
            64 => write!(f, "{}", i64::from(self.clone())),
            128 => write!(f, "{}", i128::from(self.clone())),
            160..=1024 => write!(f, "GarbledInt<{}>", N),
            _ => panic!("Unsupported bit size for GarbledInt"),
        }
    }
}

// Implement GarbledInt<N>
impl<const N: usize> GarbledInt<N> {
    // Constructor for GarbledInt<N> from a boolean vector
    pub fn new(bits: Vec<bool>) -> Self {
        assert_eq!(bits.len(), N, "The number of bits must be {}", N);
        GarbledInt {
            bits,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<GarbledUint<N>> for GarbledInt<N> {
    fn from(uint: GarbledUint<N>) -> Self {
        // Directly copy the bits from the unsigned Uint<N> to the signed GarbledInt<N>
        GarbledInt {
            bits: uint.bits,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<&GarbledUint<N>> for GarbledInt<N> {
    fn from(uint: &GarbledUint<N>) -> Self {
        // Directly copy the bits from the unsigned Uint<N> to the signed GarbledInt<N>
        GarbledInt {
            bits: uint.bits.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<i8> for GarbledInt<N> {
    fn from(value: i8) -> Self {
        assert!(N <= 8, "GarbledInt<N> can only support up to 8 bits for i8");

        // Convert i8 to bits, least-significant bit first (two's complement)
        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<i16> for GarbledInt<N> {
    fn from(value: i16) -> Self {
        assert!(N <= 16, "GarbledInt<N> can only support up to 16 bits for i16");

        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<i32> for GarbledInt<N> {
    fn from(value: i32) -> Self {
        assert!(N <= 32, "GarbledInt<N> can only support up to 32 bits for i32");

        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<i64> for GarbledInt<N> {
    fn from(value: i64) -> Self {
        assert!(N <= 64, "GarbledInt<N> can only support up to 64 bits for i64");

        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<i128> for GarbledInt<N> {
    fn from(value: i128) -> Self {
        assert!(N <= 128, "GarbledInt<N> can only support up to 128 bits for i128");

        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<GarbledInt<N>> for i8 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 8, "GarbledInt<N> can only be converted to i8 if N <= 8");

        let mut value: i8 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledInt<N>> for i16 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 16, "GarbledInt<N> can only be converted to i16 if N <= 16");

        let mut value: i16 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledInt<N>> for i32 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 32, "GarbledInt<N> can only be converted to i32 if N <= 32");

        let mut value: i32 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledInt<N>> for i64 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 64, "GarbledInt<N> can only be converted to i64 if N <= 64");

        let mut value: i64 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledInt<N>> for i128 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 128, "GarbledInt<N> can only be converted to i128 if N <= 128");

        let mut value: i128 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

// Implement ruint::Uint support for GarbledInt<N>
impl<const BITS: usize, const LIMBS: usize> TryFrom<GarbledInt<BITS>> for Uint<BITS, LIMBS> {
    type Error = ruint::ToUintError<Uint<BITS, LIMBS>>;

    fn try_from(guint: GarbledInt<BITS>) -> Result<Self, Self::Error> {
        let mut value: [u8; BITS] = [0; BITS];
        let is_negative = guint.bits[BITS - 1]; // check MSB for 2's complement; e.g. if neg

        for (i, &bit) in guint.bits.iter().enumerate() {
            if is_negative {
                value[i] = if bit { 0 } else { 1 };
            } else {
                value[i] = if bit { 1 } else { 0 };
            }
        }

        if is_negative {
            Ok(Uint::<BITS, LIMBS>::from_le_bytes(value).overflowing_add(Uint::<BITS, LIMBS>::from(1)).0)
        } else {
            Ok(Uint::<BITS, LIMBS>::from_le_bytes(value))
        }
    }
}

impl<const BITS: usize, const LIMBS: usize> From<Uint<BITS, LIMBS>> for GarbledInt<BITS> {
    fn from(uint: Uint<BITS, LIMBS>) -> Self {
        let mut bits = Vec::with_capacity(BITS);
        let is_negative = uint.bit(BITS - 1);

        let mut value = uint;
        if is_negative {
            // Convert from positive value to two's complement
            value = !value + Uint::<BITS, LIMBS>::from(1);
        }

        for i in 0..BITS {
            bits.push(value.bit(i));
        }

        GarbledInt::new(bits)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ruint::aliases::{U128, U8};

    #[test]
    fn test_from_ruint_to_gabledint() {
        let value = U8::from(5);
        let int = GarbledInt::from(value);
        assert_eq!(int, GarbledInt::<8>::from(5_i8));
    }

    #[test]
    pub fn from_garbledint_8_to_ruint () {
        let gint = GarbledInt::<8>::from(5_i8);
        let ruint = U8::try_from(gint).unwrap();
        assert_eq!(ruint, ruint::Uint::from(5_i8));
    }

    #[test]
    pub fn from_garbledint_128_to_ruint () {
        let gint = GarbledInt::<128>::from(255_i128);
        let ruint = U128::try_from(gint).unwrap();
        assert_eq!(ruint, ruint::Uint::from(255_i128));
    }

    #[test]
    pub fn from_negative_garbledint_8_to_ruint () {
        let gint = GarbledInt::<8>::from(-5_i8);
        let ruint = U8::try_from(gint.clone()).unwrap();        

        assert_eq!(U8::try_from(-5_i8).unwrap(), ruint);
    }

    #[test]
    pub fn from_negative_garbledint_128_to_ruint () {        
        let gint = GarbledInt::<128>::from(-255_i128);
        let ruint = U128::try_from(gint.clone()).unwrap();
        assert_eq!(GarbledInt::from(ruint), gint);
    }


}