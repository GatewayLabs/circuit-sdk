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
        assert!(N <= 8, "Int<N> can only support up to 8 bits for i8");

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
        assert!(N <= 16, "Int<N> can only support up to 16 bits for i16");

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
        assert!(N <= 32, "Int<N> can only support up to 32 bits for i32");

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
        assert!(N <= 64, "Int<N> can only support up to 64 bits for i64");

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
        assert!(N <= 128, "Int<N> can only support up to 128 bits for i128");

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
        assert!(N <= 8, "Int<N> can only be converted to i8 if N <= 8");

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
        assert!(N <= 16, "Int<N> can only be converted to i16 if N <= 16");

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
        assert!(N <= 32, "Int<N> can only be converted to i32 if N <= 32");

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
        assert!(N <= 64, "Int<N> can only be converted to i64 if N <= 64");

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
        assert!(N <= 128, "Int<N> can only be converted to i128 if N <= 128");

        let mut value: i128 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}
