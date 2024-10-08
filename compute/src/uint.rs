use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::marker::PhantomData;
use tandem::states::{Contributor, Evaluator};
use tandem::Circuit;

pub type GarbledUint1 = GarbledUint<1>;
pub type GarbledUint2 = GarbledUint<2>;
pub type GarbledUint4 = GarbledUint<4>;
pub type GarbledUint8 = GarbledUint<8>;
pub type GarbledUint16 = GarbledUint<16>;
pub type GarbledUint32 = GarbledUint<32>;
pub type GarbledUint64 = GarbledUint<64>;
pub type GarbledUint128 = GarbledUint<128>;

// Define a new type Uint<N>
#[derive(Debug, Clone)]
pub struct GarbledUint<const N: usize> {
    pub(crate) bits: Vec<bool>,       // Store the bits of the unsigned integer
    _phantom: PhantomData<[bool; N]>, // PhantomData to ensure the N bit size
}

// Implement Uint<N>
impl<const N: usize> GarbledUint<N> {
    // Constructor for GarbledUint<N> from a boolean vector
    pub fn new(bits: Vec<bool>) -> Self {
        assert_eq!(bits.len(), N, "The number of bits must be {}", N);
        GarbledUint {
            bits,
            _phantom: PhantomData,
        }
    }

    // Create a GarbledUint<N> from a u8 value
    pub fn from_u8(value: u8) -> Self {
        assert!(N <= 8, "Uint<N> can only support up to 8 bits for from_u8");

        // Convert u8 to bits, least-significant bit first (little-endian)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }

    // Convert a GarbledUint<N> to a u8 value
    pub fn to_u8(&self) -> u8 {
        assert!(N <= 8, "Uint<N> can only be converted to u8 if N <= 8");

        let mut value: u8 = 0;

        // Iterate through the bits and reconstruct the u8 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i; // Set the corresponding bit in the u8
            }
        }

        value
    }

    // Create a GarbledUint<N> from a u16 value
    pub fn from_u16(value: u16) -> Self {
        assert!(
            N <= 16,
            "Uint<N> can only support up to 16 bits for from_u16"
        );

        // Convert u16 to bits, least-significant bit first (little-endian)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }

    // Convert a GarbledUint<N> to a u16 value
    pub fn to_u16(&self) -> u16 {
        assert!(N <= 16, "Uint<N> can only be converted to u16 if N <= 16");

        let mut value: u16 = 0;

        // Iterate through the bits and reconstruct the u16 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i; // Set the corresponding bit in the u16
            }
        }

        value
    }

    // Create a GarbledUint<N> from a u32 value
    pub fn from_u32(value: u32) -> Self {
        assert!(
            N <= 32,
            "Uint<N> can only support up to 32 bits for from_u32"
        );

        // Convert u32 to bits, least-significant bit first (little-endian)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }

    pub fn to_u32(&self) -> u32 {
        assert!(N <= 32, "Uint<N> can only be converted to u32 if N <= 32");

        let mut value: u32 = 0;

        // Iterate through the bits and reconstruct the u32 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i; // Set the corresponding bit in the u32
            }
        }

        value
    }

    // Create a GarbledUint<N> from a u64 value
    pub fn from_u64(value: u64) -> Self {
        assert!(
            N <= 64,
            "Uint<N> can only support up to 64 bits for from_u64"
        );

        // Convert u64 to bits, least-significant bit first (little-endian)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }

    // Convert a GarbledUint<N> to a u64 value
    pub fn to_u64(&self) -> u64 {
        assert!(N <= 64, "Uint<N> can only be converted to u64 if N <= 64");

        let mut value: u64 = 0;

        // Iterate through the bits and reconstruct the u64 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i; // Set the corresponding bit in the u64
            }
        }

        value
    }

    pub fn from_u128(value: u128) -> Self {
        assert!(
            N <= 128,
            "Uint<N> can only support up to 128 bits for from_u128"
        );

        // Convert u128 to bits, least-significant bit first (little-endian)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        GarbledUint::new(bits)
    }

    pub fn to_u128(&self) -> u128 {
        assert!(
            N <= 128,
            "Uint<N> can only be converted to u128 if N <= 128"
        );

        let mut value: u128 = 0;

        // Iterate through the bits and reconstruct the u128 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << i; // Set the corresponding bit in the u128
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

// test conversions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let a = GarbledUint8::from_u8(170); // Binary 10101010
        assert_eq!(a.to_u8(), 170);
    }

    #[test]
    fn test_from_u16() {
        let a = GarbledUint16::from_u16(43707); // Binary 1010101010101011
        assert_eq!(a.to_u16(), 43707);
    }

    #[test]
    fn test_from_u32() {
        let a = GarbledUint32::from_u32(2863311530); // Binary 10101010101010101010101010101010
        assert_eq!(a.to_u32(), 2863311530);
    }

    #[test]
    fn test_from_u64() {
        let a = GarbledUint64::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        assert_eq!(a.to_u64(), 12297829382473034410);
    }

    #[test]
    fn test_from_u128() {
        let a = GarbledUint128::from_u128(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        assert_eq!(a.to_u128(), 12297829382473034410);
    }
}
