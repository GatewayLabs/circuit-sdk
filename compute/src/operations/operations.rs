use crate::u256::U256;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use tandem::states::{Contributor, Evaluator};
use tandem::GateIndex;
use tandem::{Circuit, Error, Gate};

// Test the Uint<N> XOR functionality
#[cfg(test)]
mod tests {
    // use super::*;

    use crate::operations::Uint;

    #[test]
    fn test_uint_mul() {
        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = Uint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a * b; // Perform multiplication on the 4-bit values
        assert_eq!(result.to_string(), "0000"); // Binary 0000 (Multiplication result of 1100 * 0011)
    }
}
