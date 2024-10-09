use crate::uint::GarbledUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::convert::From;
use std::marker::PhantomData;
use tandem::states::{Contributor, Evaluator};
use tandem::Circuit;

pub type GarbledInt1 = GarbledInt<1>;
pub type GarbledInt2 = GarbledInt<2>;
pub type GarbledInt4 = GarbledInt<4>;
pub type GarbledInt8 = GarbledInt<8>;
pub type GarbledInt16 = GarbledInt<16>;
pub type GarbledInt32 = GarbledInt<32>;
pub type GarbledInt64 = GarbledInt<64>;
pub type GarbledInt128 = GarbledInt<128>;

impl<const N: usize> From<GarbledUint<N>> for GarbledInt<N> {
    fn from(uint: GarbledUint<N>) -> Self {
        // Directly copy the bits from the unsigned Uint<N> to the signed Int<N>
        GarbledInt {
            bits: uint.bits,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<&GarbledUint<N>> for GarbledInt<N> {
    fn from(uint: &GarbledUint<N>) -> Self {
        // Directly copy the bits from the unsigned Uint<N> to the signed Int<N>
        GarbledInt {
            bits: uint.bits.clone(),
            _phantom: PhantomData,
        }
    }
}

// Define a new type GarbledInt<N>
#[derive(Debug, Clone)]
pub struct GarbledInt<const N: usize> {
    pub(crate) bits: Vec<bool>, // Store the bits of the signed integer (in two's complement)
    _phantom: PhantomData<[bool; N]>, // PhantomData to ensure the N bit size
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

    // Create a GarbledInt<N> from an i8 value
    pub fn from_i8(value: i8) -> Self {
        assert!(N <= 8, "Int<N> can only support up to 8 bits for from_i8");

        // Convert i8 to bits, least-significant bit first (little-endian, two's complement)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledInt::new(bits)
    }

    // Convert a GarbledInt<N> to an i8 value
    pub fn to_i8(&self) -> i8 {
        assert!(N <= 8, "Int<N> can only be converted to i8 if N <= 8");

        let mut value: i8 = 0;

        // Iterate through the bits and reconstruct the i8 value (two's complement)
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }

    // Create a GarbledInt<N> from an i16 value
    pub fn from_i16(value: i16) -> Self {
        assert!(
            N <= 16,
            "Int<N> can only support up to 16 bits for from_i16"
        );

        // Convert i16 to bits, least-significant bit first (little-endian, two's complement)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledInt::new(bits)
    }

    // Convert a GarbledInt<N> to an i16 value
    // Convert a GarbledInt<N> to an i16 value
    pub fn to_i16(&self) -> i16 {
        assert!(N <= 16, "Int<N> can only be converted to i16 if N <= 16");

        let mut value: i16 = 0;

        // Iterate through the bits and reconstruct the i16 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }

    // Create a GarbledInt<N> from an i32 value
    pub fn from_i32(value: i32) -> Self {
        assert!(
            N <= 32,
            "Int<N> can only support up to 32 bits for from_i32"
        );

        // Convert i32 to bits, least-significant bit first (little-endian, two's complement)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledInt::new(bits)
    }

    // Convert a GarbledInt<N> to an i32 value
    pub fn to_i32(&self) -> i32 {
        assert!(N <= 32, "Int<N> can only be converted to i32 if N <= 32");

        let mut value: i32 = 0;

        // Iterate through the bits and reconstruct the i32 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }

    // Create a GarbledInt<N> from an i64 value
    pub fn from_i64(value: i64) -> Self {
        assert!(
            N <= 64,
            "Int<N> can only support up to 64 bits for from_i64"
        );

        // Convert i64 to bits, least-significant bit first (little-endian, two's complement)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledInt::new(bits)
    }

    // Convert a GarbledInt<N> to an i64 value
    pub fn to_i64(&self) -> i64 {
        assert!(N <= 64, "Int<N> can only be converted to i64 if N <= 64");

        let mut value: i64 = 0;

        // Iterate through the bits and reconstruct the i64 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }

    pub fn from_i128(value: i128) -> Self {
        assert!(
            N <= 128,
            "Int<N> can only support up to 128 bits for from_i128"
        );

        // Convert i128 to bits, least-significant bit first (little-endian, two's complement)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledInt::new(bits)
    }

    // Convert a GarbledInt<N> to an i128 value
    pub fn to_i128(&self) -> i128 {
        assert!(N <= 128, "Int<N> can only be converted to i128 if N <= 128");

        let mut value: i128 = 0;

        // Iterate through the bits and reconstruct the i128 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }

    /// Simulates the local execution of the circuit using a 2 Party MPC protocol.
    ///
    /// The Multi-Party Computation is performed using the full cryptographic protocol exposed by the
    /// [`Contributor`] and [`Evaluator`]. The messages between contributor and evaluator are exchanged
    /// using local message queues. This function thus simulates an MPC execution on a local machine
    /// under ideal network conditions, without any latency or bandwidth restrictions.
    pub fn simulate(
        &self,
        circuit: &Circuit,
        input_contributor: &[bool],
        input_evaluator: &[bool],
    ) -> anyhow::Result<Vec<bool>> {
        let mut eval = Evaluator::new(
            circuit.clone(),
            input_evaluator,
            ChaCha20Rng::from_entropy(),
        )?;
        let (mut contrib, mut msg_for_eval) =
            Contributor::new(circuit, input_contributor, ChaCha20Rng::from_entropy())?;

        tracing::debug!("contributor ciphertext: {:?}", hex::encode(&msg_for_eval));

        assert_eq!(contrib.steps(), eval.steps());

        for _ in 0..eval.steps() {
            let (next_state, msg_for_contrib) = eval.run(&msg_for_eval)?;
            eval = next_state;

            let (next_state, reply) = contrib.run(&msg_for_contrib)?;
            contrib = next_state;

            msg_for_eval = reply;
        }
        Ok(eval.output(&msg_for_eval)?)
    }
}

// Test conversions
#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::{GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8};

    #[test]
    fn test_from_i8() {
        let a = GarbledInt8::from_i8(-86); // Two's complement binary for -86 is 10101010
        assert_eq!(a.to_i8(), -86);
    }

    #[test]
    fn test_from_i16() {
        let a = GarbledInt16::from_i16(-21845); // Two's complement binary for -21845 is 1010101010101011
        assert_eq!(a.to_i16(), -21845);
    }

    #[test]
    fn test_from_i32() {
        let a = GarbledInt32::from_i32(-1431655765); // Two's complement binary for -1431655765 is 10101010101010101010101010101011
        assert_eq!(a.to_i32(), -1431655765);
    }

    #[test]
    fn test_from_i64() {
        let a = GarbledInt64::from_i64(-6148914691236517205); // Two's complement binary for -6148914691236517205 is 1010101010101010101010101010101010101010101010101010101010101011
        assert_eq!(a.to_i64(), -6148914691236517205);
    }

    #[test]
    fn test_from_i128() {
        let a = GarbledInt128::from_i128(-6148914691236517205); // Two's complement binary for -6148914691236517205 is 1010101010101010101010101010101010101010101010101010101010101011
        assert_eq!(a.to_i128(), -6148914691236517205);
    }

    #[test]
    fn test_from_uint_i8() {
        let uint = GarbledUint8::from_u8(170); // 10101010 (unsigned)
        let int: GarbledInt8 = GarbledInt::from(uint); // Interpreted as -86 (two's complement signed)
        assert_eq!(int.to_i8(), -86);
    }

    #[test]
    fn test_from_uint_i16() {
        let uint = GarbledUint16::from_u16(43707); // 1010101010101011 (unsigned)
        let int: GarbledInt16 = GarbledInt::from(uint); // Interpreted as -21845 (two's complement signed)
        assert_eq!(int.to_i16(), 43707_u16 as i16);
    }

    #[test]
    fn test_into_uint_i32() {
        let uint = GarbledUint32::from_u32(2863311530); // 10101010101010101010101010101010 (unsigned)
        let int: GarbledInt32 = uint.into(); // Interpreted as -1431655766 (two's complement signed)
        assert_eq!(int.to_i32(), 2863311530_u32 as i32);
    }

    #[test]
    fn test_into_uint_i64() {
        let uint = GarbledUint64::from_u64(12297829382473034410); // 1010101010101010101010101010101010101010101010101010101010101010 (unsigned)
        let int: GarbledInt64 = uint.into(); // Interpreted as -6148914691236517206 (two's complement signed)
        assert_eq!(int.to_i64(), 12297829382473034410_u64 as i64);
    }

    #[test]
    fn test_into_uint_i128() {
        let uint = GarbledUint128::from_u128(12297829382473034410); // 1010101010101010101010101010101010101010101010101010101010101010 (unsigned)
        let int: GarbledInt128 = uint.into(); // Interpreted as -6148914691236517206 (two's complement signed)
        assert_eq!(int.to_i128(), 12297829382473034410_u128 as i128);
    }
}
