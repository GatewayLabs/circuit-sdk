use crate::u256::U256;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitXor, Not, Shl, Shr};
use tandem::states::{Contributor, Evaluator};
use tandem::GateIndex;
use tandem::{Circuit, Error, Gate};

// Define a new type Uint<N>
#[derive(Debug, Clone)]
pub struct Uint<const N: usize> {
    pub(crate) bits: Vec<bool>,       // Store the bits of the unsigned integer
    _phantom: PhantomData<[bool; N]>, // PhantomData to ensure the N bit size
}

// Implement Uint<N>
impl<const N: usize> Uint<N> {
    // Constructor for Uint<N> from a boolean vector
    pub fn new(bits: Vec<bool>) -> Self {
        assert_eq!(bits.len(), N, "The number of bits must be {}", N);
        Uint {
            bits,
            _phantom: PhantomData,
        }
    }

    // Create a Uint<N> from a u8 value
    pub fn from_u8(value: u8) -> Self {
        assert!(N <= 8, "Uint<N> can only support up to 8 bits for from_u8");

        // Convert u8 to bits, least-significant bit first (little-endian)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i) & 1 == 1);
        }

        Uint::new(bits)
    }

    // Convert a Uint<N> to a u8 value
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

    // Create a Uint<N> from a u16 value
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

        Uint::new(bits)
    }

    // Convert a Uint<N> to a u16 value
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

    // Create a Uint<N> from a u32 value
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

        Uint::new(bits)
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

    // Create a Uint<N> from a u64 value
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

        Uint::new(bits)
    }

    // Convert a Uint<N> to a u64 value
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

        Uint::new(bits)
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

    pub fn to_u256(&self) -> U256 {
        assert!(
            N <= 256,
            "Uint<N> can only be converted to u256 if N <= 256"
        );

        let mut value: U256 = U256::zero();

        // Iterate through the bits and reconstruct the u256 value
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                value |= U256::one() << i as u32; //U256::from_u64(i as u64); // Set the corresponding bit in the u256
            }
        }

        value
    }

    pub fn from_u256(value: U256) -> Self {
        assert!(
            N <= 256,
            "Uint<N> can only support up to 256 bits for from_u256"
        );

        // Convert u256 to bits, least-significant bit first (little-endian)
        let mut bits = Vec::with_capacity(N);
        for i in 0..N {
            bits.push((value >> i as u32) & U256::one() == U256::one());
        }

        Uint::new(bits)
    }

    // Create a simple display method for Uint<N>
    fn to_string(&self) -> String {
        self.bits
            .iter()
            .map(|&b| if b { '1' } else { '0' })
            .collect::<String>()
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
    ) -> Result<Vec<bool>, Error> {
        let mut eval = Evaluator::new(
            circuit.clone(),
            input_evaluator,
            ChaCha20Rng::from_entropy(),
        )?;
        let (mut contrib, mut msg_for_eval) =
            Contributor::new(circuit, input_contributor, ChaCha20Rng::from_entropy())?;

        //println!("contributor ciphertext: {:?}", hex::encode(&msg_for_eval));

        assert_eq!(contrib.steps(), eval.steps());

        for _ in 0..eval.steps() {
            let (next_state, msg_for_contrib) = eval.run(&msg_for_eval)?;
            eval = next_state;

            let (next_state, reply) = contrib.run(&msg_for_contrib)?;
            contrib = next_state;

            msg_for_eval = reply;
        }
        eval.output(&msg_for_eval)
    }

    // Helper method to simulate a full-adder multiplier
    pub fn push_multiplier(
        &self,
        gates: &mut Vec<Gate>,
        a: GateIndex,
        b: GateIndex,
        z: GateIndex,
        carry: GateIndex,
    ) -> (GateIndex, GateIndex) {
        // AND gate for partial product (a & b)
        let and_ab = self.push_and(gates, a, b);

        // Full adder for combining the partial product with sum and carry
        let (sum, carry_out) = self.push_adder(gates, and_ab, z, carry);
        (sum, carry_out)
    }

    // Helper method to simulate an OR operation using XOR and AND gates
    pub fn push_or(&self, gates: &mut Vec<Gate>, x: GateIndex, y: GateIndex) -> GateIndex {
        // Simulate OR with XOR and AND gates
        let xor = self.push_xor(gates, x, y);
        let and = self.push_and(gates, x, y);
        self.push_xor(gates, xor, and) // Return the simulated OR result
    }

    // Simulates an AND gate and returns its index
    pub fn push_and(&self, gates: &mut Vec<Gate>, x: GateIndex, y: GateIndex) -> GateIndex {
        // Create the AND gate and return its index
        let index = gates.len();
        gates.push(Gate::And(x, y));
        index.try_into().unwrap()
    }

    // Simulates an XOR gate and returns its index
    pub fn push_xor(&self, gates: &mut Vec<Gate>, x: GateIndex, y: GateIndex) -> GateIndex {
        // Create the XOR gate and return its index
        let index = gates.len();
        gates.push(Gate::Xor(x, y));
        index.try_into().unwrap()
    }

    // Simulates a full-adder, returning the sum and carry
    pub fn push_adder(
        &self,
        gates: &mut Vec<Gate>,
        x: GateIndex,
        y: GateIndex,
        carry: GateIndex,
    ) -> (GateIndex, GateIndex) {
        // First half-adder: x ^ y (sum) and x & y (carry)
        let sum1 = self.push_xor(gates, x, y);
        let carry1 = self.push_and(gates, x, y);

        // Second half-adder: (sum1 ^ carry) (final sum) and (sum1 & carry) (additional carry)
        let final_sum = self.push_xor(gates, sum1, carry);
        let carry2 = self.push_and(gates, sum1, carry);

        // OR the two carry bits to get the final carry
        let final_carry = self.push_or(gates, carry1, carry2);

        (final_sum, final_carry)
    }
}

// Implement the XOR operation for Uint<N>
impl<const N: usize> BitXor for Uint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit XOR operation
        let mut gates = Vec::new();

        // Push the input gates for both Uint<N> objects
        for _ in 0..N {
            gates.push(Gate::InContrib); // From first Uint<N> (self)
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define XOR gates for each corresponding bit in self and rhs
        for i in 0..N {
            gates.push(Gate::Xor(
                i.try_into().unwrap(),
                (N + i).try_into().unwrap(),
            )); // XOR gate between corresponding bits
        }

        // Define the output indices (for N-bit XOR)
        let n = N as u32;
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the XOR operation for &Uint<N>
impl<const N: usize> BitXor for &Uint<N> {
    type Output = Uint<N>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit XOR operation
        let mut gates = Vec::new();

        // Push the input gates for both Uint<N> objects
        for _ in 0..N {
            gates.push(Gate::InContrib); // From first Uint<N> (self)
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define XOR gates for each corresponding bit in self and rhs
        for i in 0..N {
            gates.push(Gate::Xor(
                i.try_into().unwrap(),
                (N + i).try_into().unwrap(),
            )); // XOR gate between corresponding bits
        }

        // Define the output indices (for N-bit XOR)
        let n = N as u32;
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the AND operation for Uint<N>
impl<const N: usize> BitAnd for Uint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit AND operation
        let mut gates = Vec::new();

        // Push the input gates for both Uint<N> objects
        for _ in 0..N {
            gates.push(Gate::InContrib); // From first Uint<N> (self)
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define AND gates for each corresponding bit in self and rhs
        for i in 0..N {
            gates.push(Gate::And(
                i.try_into().unwrap(),
                (N + i).try_into().unwrap(),
            )); // AND gate between corresponding bits
        }

        // Define the output indices (for N-bit AND)
        let n = N as u32;
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the AND operation for &Uint<N>
impl<const N: usize> BitAnd for &Uint<N> {
    type Output = Uint<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit AND operation
        let mut gates = Vec::new();

        // Push the input gates for both Uint<N> objects
        for _ in 0..N {
            gates.push(Gate::InContrib); // From first Uint<N> (self)
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define AND gates for each corresponding bit in self and rhs
        for i in 0..N {
            gates.push(Gate::And(
                i.try_into().unwrap(),
                (N + i).try_into().unwrap(),
            )); // AND gate between corresponding bits
        }

        // Define the output indices (for N-bit AND)
        let n = N as u32;
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the NOT operation for Uint<N>
impl<const N: usize> Not for Uint<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        // Build a circuit that performs an N-bit NOT operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define NOT gates for each bit in the Uint<N>
        for i in 0..N * 2 {
            gates.push(Gate::Not(i.try_into().unwrap())); // NOT gate for each bit
        }

        // Define the output indices (for N-bit NOT)
        let n = N as u32;
        //let output_indices: Vec<u32> = (n..2 * n).collect();
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        //let output_indices = vec![2];

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &self.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the NOT operation for &Uint<N>
impl<const N: usize> Not for &Uint<N> {
    type Output = Uint<N>;

    fn not(self) -> Self::Output {
        // Build a circuit that performs an N-bit NOT operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define NOT gates for each bit in the Uint<N>
        for i in 0..N * 2 {
            gates.push(Gate::Not(i.try_into().unwrap())); // NOT gate for each bit
        }

        // Define the output indices (for N-bit NOT)
        let n = N as u32;
        //let output_indices: Vec<u32> = (n..2 * n).collect();
        let output_indices: Vec<u32> = (2 * n..2 * n + n).collect();

        //let output_indices = vec![2];

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &self.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement Shift Left operation for Uint<N> and &Uint<N>
impl<const N: usize> Shl<usize> for Uint<N> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();

        // Shift the bits to the left by the specified amount
        for _ in 0..shift {
            bits.remove(0); // Remove the least significant bit
            bits.push(false); // Add a 0 to the most significant bit
        }

        Uint::new(bits)
    }
}

impl<const N: usize> Shl<usize> for &Uint<N> {
    type Output = Uint<N>;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();

        // Shift the bits to the left by the specified amount
        for _ in 0..shift {
            bits.remove(0); // Remove the least significant bit
            bits.push(false); // Add a 0 to the most significant bit
        }

        Uint::new(bits)
    }
}

// Implement Shift Right operation for Uint<N> and &Uint<N>
impl<const N: usize> Shr<usize> for Uint<N> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();

        // Shift the bits to the right by the specified amount
        for _ in 0..shift {
            bits.pop(); // Remove the most significant bit
            bits.insert(0, false); // Add a 0 to the least significant bit
        }

        Uint::new(bits)
    }
}

impl<const N: usize> Shr<usize> for &Uint<N> {
    type Output = Uint<N>;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();

        // Shift the bits to the right by the specified amount
        for _ in 0..shift {
            bits.pop(); // Remove the most significant bit
            bits.insert(0, false); // Add a 0 to the least significant bit
        }

        Uint::new(bits)
    }
}

// arithmetic operations

// Implement the Add operation for Uint<N> and &Uint<N>
impl<const N: usize> std::ops::Add for Uint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit ADD operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define the carry bit
        let mut carry_index = None; // No carry initially

        // Define sum bit indices
        let mut sum_bit_indices = Vec::with_capacity(N);

        // Generate gates for N-bit addition
        for i in 0..N {
            let a = i; // Index for bit i of self
            let b = N + i; // Index for bit i of rhs

            // XOR gate for sum bit (a ⊕ b)
            let sum_xor_index = gates.len();
            gates.push(Gate::Xor(a.try_into().unwrap(), b.try_into().unwrap()));

            // If carry exists, XOR the result of the previous XOR with the carry
            let final_sum_index = if let Some(carry) = carry_index {
                let sum_with_carry_index = gates.len();
                gates.push(Gate::Xor(sum_xor_index.try_into().unwrap(), carry));
                sum_with_carry_index
            } else {
                sum_xor_index
            };

            sum_bit_indices.push(final_sum_index);

            // Compute the new carry: (a & b) | (a & carry) | (b & carry)
            let and_ab = gates.len();
            gates.push(Gate::And(a.try_into().unwrap(), b.try_into().unwrap())); // a & b

            if let Some(carry) = carry_index {
                let and_a_carry = gates.len();
                gates.push(Gate::And(a.try_into().unwrap(), carry)); // a & carry

                let and_b_carry = gates.len();
                gates.push(Gate::And(b.try_into().unwrap(), carry)); // b & carry

                // Combine the carry parts using XOR and AND to simulate OR
                let xor_ab_carry = gates.len();
                gates.push(Gate::Xor(
                    and_ab.try_into().unwrap(),
                    and_a_carry.try_into().unwrap(),
                )); // XOR part 1
                gates.push(Gate::Xor(
                    xor_ab_carry.try_into().unwrap(),
                    and_b_carry.try_into().unwrap(),
                )); // Final carry (simulated OR)
                carry_index = Some((gates.len() - 1).try_into().unwrap());
            } else {
                // If there is no previous carry, the carry is just a & b
                carry_index = Some(and_ab.try_into().unwrap());
            }
        }

        // Define output indices (sum bits from the addition)
        let output_indices: Vec<u32> = sum_bit_indices.iter().map(|&index| index as u32).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

impl<const N: usize> std::ops::Add for &Uint<N> {
    type Output = Uint<N>;

    fn add(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit ADD operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define the carry bit
        let mut carry_index = None; // No carry initially

        // Define sum bit indices
        let mut sum_bit_indices = Vec::with_capacity(N);

        // Generate gates for N-bit addition
        for i in 0..N {
            let a = i; // Index for bit i of self
            let b = N + i; // Index for bit i of rhs

            // XOR gate for sum bit (a ⊕ b)
            let sum_xor_index = gates.len();
            gates.push(Gate::Xor(a.try_into().unwrap(), b.try_into().unwrap()));

            // If carry exists, XOR the result of the previous XOR with the carry
            let final_sum_index = if let Some(carry) = carry_index {
                let sum_with_carry_index = gates.len();
                gates.push(Gate::Xor(sum_xor_index.try_into().unwrap(), carry));
                sum_with_carry_index
            } else {
                sum_xor_index
            };

            sum_bit_indices.push(final_sum_index);

            // Compute the new carry: (a & b) | (a & carry) | (b & carry)
            let and_ab = gates.len();
            gates.push(Gate::And(a.try_into().unwrap(), b.try_into().unwrap())); // a & b

            if let Some(carry) = carry_index {
                let and_a_carry = gates.len();
                gates.push(Gate::And(a.try_into().unwrap(), carry)); // a & carry

                let and_b_carry = gates.len();
                gates.push(Gate::And(b.try_into().unwrap(), carry)); // b & carry

                // Combine the carry parts using XOR and AND to simulate OR
                let xor_ab_carry = gates.len();
                gates.push(Gate::Xor(
                    and_ab.try_into().unwrap(),
                    and_a_carry.try_into().unwrap(),
                )); // XOR part 1
                gates.push(Gate::Xor(
                    xor_ab_carry.try_into().unwrap(),
                    and_b_carry.try_into().unwrap(),
                )); // Final carry (simulated OR)
                carry_index = Some((gates.len() - 1).try_into().unwrap());
            } else {
                // If there is no previous carry, the carry is just a & b
                carry_index = Some(and_ab.try_into().unwrap());
            }
        }

        // Define output indices (sum bits from the addition)
        let output_indices: Vec<u32> = sum_bit_indices.iter().map(|&index| index as u32).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the Sub operation for Uint<N> and &Uint<N>
impl<const N: usize> std::ops::Sub for Uint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit SUB operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define the borrow bit
        let mut borrow_index = None; // No borrow initially

        // Define difference bit indices
        let mut diff_bit_indices = Vec::with_capacity(N);

        // Generate gates for N-bit subtraction
        for i in 0..N {
            let a = i; // Index for bit i of self
            let b = N + i; // Index for bit i of rhs

            // XOR gate for difference bit (a ⊕ b)
            let diff_xor_index = gates.len();
            gates.push(Gate::Xor(a.try_into().unwrap(), b.try_into().unwrap()));

            // If borrow exists, XOR the result of the previous XOR with the borrow
            let final_diff_index = if let Some(borrow) = borrow_index {
                let diff_with_borrow_index = gates.len();
                gates.push(Gate::Xor(diff_xor_index.try_into().unwrap(), borrow));
                diff_with_borrow_index
            } else {
                diff_xor_index
            };

            diff_bit_indices.push(final_diff_index);

            // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
            let not_a = gates.len();
            gates.push(Gate::Not(a.try_into().unwrap())); // !a

            let and_not_a_b = gates.len();
            gates.push(Gate::And(not_a.try_into().unwrap(), b.try_into().unwrap())); // !a & b

            if let Some(borrow) = borrow_index {
                let and_a_borrow = gates.len();
                gates.push(Gate::And(a.try_into().unwrap(), borrow)); // a & borrow

                let not_b = gates.len();
                gates.push(Gate::Not(b.try_into().unwrap())); // !b

                let and_not_borrow = gates.len();
                gates.push(Gate::And(not_b.try_into().unwrap(), borrow)); // !b & borrow

                // Combine the borrow parts using XOR and AND to simulate OR
                let xor_borrow_parts = gates.len();
                gates.push(Gate::Xor(
                    and_not_a_b.try_into().unwrap(),
                    and_a_borrow.try_into().unwrap(),
                )); // XOR part 1
                gates.push(Gate::Xor(
                    xor_borrow_parts.try_into().unwrap(),
                    and_not_borrow.try_into().unwrap(),
                )); // Final borrow (simulated OR)
                borrow_index = Some((gates.len() - 1).try_into().unwrap());
            } else {
                // If there is no previous borrow, the borrow is just !a & b
                borrow_index = Some(and_not_a_b.try_into().unwrap());
            }
        }

        // Define output indices (difference bits from the subtraction)
        let output_indices: Vec<u32> = diff_bit_indices.iter().map(|&index| index as u32).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the Sub operation for &Uint<N>
impl<const N: usize> std::ops::Sub for &Uint<N> {
    type Output = Uint<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        // Build a circuit that performs an N-bit SUB operation
        let mut gates = Vec::new();

        // Push the input gates for the Uint<N> object
        for _ in 0..N {
            gates.push(Gate::InContrib); // From the Uint<N> object
        }
        for _ in 0..N {
            gates.push(Gate::InEval); // From second Uint<N> (rhs)
        }

        // Define the borrow bit
        let mut borrow_index = None; // No borrow initially

        // Define difference bit indices
        let mut diff_bit_indices = Vec::with_capacity(N);

        // Generate gates for N-bit subtraction
        for i in 0..N {
            let a = i; // Index for bit i of self
            let b = N + i; // Index for bit i of rhs

            // XOR gate for difference bit (a ⊕ b)
            let diff_xor_index = gates.len();
            gates.push(Gate::Xor(a.try_into().unwrap(), b.try_into().unwrap()));

            // If borrow exists, XOR the result of the previous XOR with the borrow
            let final_diff_index = if let Some(borrow) = borrow_index {
                let diff_with_borrow_index = gates.len();
                gates.push(Gate::Xor(diff_xor_index.try_into().unwrap(), borrow));
                diff_with_borrow_index
            } else {
                diff_xor_index
            };

            diff_bit_indices.push(final_diff_index);

            // Compute the new borrow: (!a & b) | (a & borrow) | (!b & borrow)
            let not_a = gates.len();
            gates.push(Gate::Not(a.try_into().unwrap())); // !a

            let and_not_a_b = gates.len();
            gates.push(Gate::And(not_a.try_into().unwrap(), b.try_into().unwrap())); // !a & b

            if let Some(borrow) = borrow_index {
                let and_a_borrow = gates.len();
                gates.push(Gate::And(a.try_into().unwrap(), borrow)); // a & borrow

                let not_b = gates.len();
                gates.push(Gate::Not(b.try_into().unwrap())); // !b

                let and_not_borrow = gates.len();
                gates.push(Gate::And(not_b.try_into().unwrap(), borrow)); // !b & borrow

                // Combine the borrow parts using XOR and AND to simulate OR
                let xor_borrow_parts = gates.len();
                gates.push(Gate::Xor(
                    and_not_a_b.try_into().unwrap(),
                    and_a_borrow.try_into().unwrap(),
                )); // XOR part 1
                gates.push(Gate::Xor(
                    xor_borrow_parts.try_into().unwrap(),
                    and_not_borrow.try_into().unwrap(),
                )); // Final borrow (simulated OR)
                borrow_index = Some((gates.len() - 1).try_into().unwrap());
            } else {
                // If there is no previous borrow, the borrow is just !a & b
                borrow_index = Some(and_not_a_b.try_into().unwrap());
            }
        }

        // Define output indices (difference bits from the subtraction)
        let output_indices: Vec<u32> = diff_bit_indices.iter().map(|&index| index as u32).collect();

        // Create the circuit
        let program = Circuit::new(gates, output_indices);

        // Simulate the circuit
        let result = self.simulate(&program, &self.bits, &rhs.bits).unwrap();

        // Return the resulting Uint<N>
        Uint::new(result)
    }
}

// Implement the Mul operation for Uint<N>

// Test the Uint<N> XOR functionality
#[cfg(test)]
mod tests {
    // use super::*;

    use crate::operations::Uint;

    #[test]
    fn test_uint_xor() {
        let a = Uint::<2>::new(vec![true, false]); // Binary 10
        let b = Uint::<2>::new(vec![false, true]); // Binary 01

        let result = a ^ b; // Perform XOR on the 2-bit values
        assert_eq!(result.to_u8(), 3); // Expected result of XOR between 10 and 01

        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = Uint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a ^ b; // Perform XOR on the 4-bit values
        assert_eq!(result.to_u8(), 15); // Expected result of XOR between 1100 and 0011
    }

    #[test]
    fn test_from_u8_xor() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010
        let b = Uint::<8>::from_u8(85); // Binary 01010101

        let result = a ^ b;
        assert_eq!(result.to_string(), "11111111"); // Expected result of XOR between 10101010 and 01010101

        assert_eq!(result.to_u8(), 255); // Expected result of XOR between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_xor() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010
        let b = Uint::<16>::from_u16(21845); // Binary 0101010101010101

        let result = a ^ b;
        assert_eq!(result.to_string(), "1111111111111111"); // Expected result of XOR between 1010101010101010 and 0101010101010101
        assert_eq!(result.to_u16(), 65535); // Expected result of XOR between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_xor() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = Uint::<32>::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a ^ b;
        assert_eq!(result.to_string(), "11111111111111111111111111111111"); // Expected result of XOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
        assert_eq!(result.to_u32(), 4294967295); // Expected result of XOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_xor() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = Uint::<64>::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a ^ b;
        assert_eq!(
            result.to_string(),
            "1111111111111111111111111111111111111111111111111111111111111111"
        ); // Expected result of XOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
        assert_eq!(result.to_u64(), 18446744073709551615); // Expected result of XOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_xor() {
        let a = Uint::<128>::from_u128(170); // Binary 10101010
        let b = Uint::<128>::from_u128(85); // Binary 01010101

        let result = a ^ b;
        assert_eq!(result.to_string(), "11111111000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"); // Expected result of XOR between 10101010 and 01010101
        assert_eq!(result.to_u128(), 255); // Expected result of XOR between 10101010 and 01010101
    }

    #[test]
    fn test_uint_and() {
        let a = Uint::<2>::new(vec![true, false]); // Binary 10
        let b = Uint::<2>::new(vec![false, true]); // Binary 01

        let result = a & b; // Perform AND on the 2-bit values
        assert_eq!(result.to_string(), "00"); // Binary 00 (AND result of 10 & 01)

        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = Uint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a & b; // Perform AND on the 4-bit values
        assert_eq!(result.to_string(), "0000"); // Binary 0000 (AND result of 1100 & 0011)
    }

    #[test]
    fn test_from_u8_and() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010
        let b = Uint::<8>::from_u8(85); // Binary 01010101

        let result = a & b;
        assert_eq!(result.to_u8(), 170 & 85); // Expected result of AND between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_and() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010
        let b = Uint::<16>::from_u16(21845); // Binary 0101010101010101

        let result = a & b;
        assert_eq!(result.to_u16(), 43690 & 21845); // Expected result of AND between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_and() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = Uint::<32>::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a & b;
        assert_eq!(result.to_u32(), 2863311530 & 1431655765); // Expected result of AND between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_and() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = Uint::<64>::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a & b;
        assert_eq!(result.to_u64(), 12297829382473034410 & 6148914691236517205);
        // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_and() {
        let a = Uint::<128>::from_u128(170); // Binary 10101010
        let b = Uint::<128>::from_u128(85); // Binary 01010101

        let result = a & b;
        assert_eq!(result.to_u128(), 170 & 85); // Expected result of AND between 10101010 and 01010101
    }

    #[test]
    fn test_uint_not() {
        let a = Uint::<1>::new(vec![true]);

        let result = !a; // Perform NOT
        assert_eq!(result.to_string(), "0");

        let a = Uint::<2>::new(vec![true, false]); // Binary 10

        let result = !a; // Perform NOT on the 2-bit value
        assert_eq!(result.to_string(), "01"); // Binary 01 (NOT result of 10)

        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100

        let result = !a; // Perform NOT on the 4-bit value
        assert_eq!(result.to_string(), "0011"); // Binary 0011 (NOT result of 1100)
    }

    #[test]
    fn test_from_u8_not() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010

        let result = !a;
        assert_eq!(result.to_u8(), !170); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_from_u16_not() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010

        let result = !a;
        assert_eq!(result.to_u16(), !43690); // Expected result of NOT on 1010101010101010
    }

    #[test]
    fn test_from_u32_not() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010

        let result = !a;
        assert_eq!(result.to_u32(), !2863311530); // Expected result of NOT on 10101010101010101010101010101010
    }

    #[test]
    fn test_from_u64_not() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010

        let result = !a;
        assert_eq!(result.to_u64(), !12297829382473034410);
        // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010
    }

    #[test]
    fn test_from_u128_not() {
        let a = Uint::<128>::from_u128(170); // Binary 10101010

        let result = !a;
        assert_eq!(result.to_u128(), !170); // Expected result of NOT on 10101010
    }

    #[test]
    fn test_left_shift() {
        let a = Uint::<4>::new(vec![true, false, false, false]); // Binary 1000

        let result = a << 1; // Perform left shift by 1
        assert_eq!(result.to_string(), "0000"); // Binary 0000 (Left shift result of 1000)

        let a = Uint::<4>::new(vec![true, false, false, false]); // Binary 1000

        let result = a << 2; // Perform left shift by 2
        assert_eq!(result.to_string(), "0000"); // Binary 0000 (Left shift result of 1000)

        let a = Uint::<4>::new(vec![true, false, false, false]); // Binary 1000

        let result = a << 3; // Perform left shift by 3
        assert_eq!(result.to_string(), "0000"); // Binary 0000 (Left shift result of 1000)

        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 0001

        let result = a << 1; // Perform left shift by 1
        assert_eq!(result.to_string(), "0010"); // Binary 0010 (Left shift result of 0001)

        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 0001

        let result = a << 2; // Perform left shift by 2
        assert_eq!(result.to_string(), "0100"); // Binary 0100 (Left shift result of 0001)

        let a = Uint::<4>::new(vec![false, false, false, true]); // Binary 0001

        let result = a << 3; // Perform left shift by 3
        assert_eq!(result.to_string(), "1000"); // Binary 1000 (Left shift result of 0001)
    }

    #[test]
    fn test_right_shift() {
        let a = Uint::<4>::new(vec![true, false, false, false]); // Binary 1000

        let result = a >> 1; // Perform right shift by 1
        assert_eq!(result.to_string(), "0100"); // Binary 0100 (Right shift result of 1000)

        let a = Uint::<4>::new(vec![true, false, false, false]); // Binary 1000

        let result = a >> 2; // Perform right shift by 2
        assert_eq!(result.to_string(), "0010"); // Binary 0010 (Right shift result of 1000)

        let a = Uint::<4>::new(vec![true, false, false, false]); // Binary 1000

        let result = a >> 3; // Perform right shift by 3
        assert_eq!(result.to_string(), "0001"); // Binary 0001 (Right shift result of 1000)
    }

    #[test]
    fn test_uint_add() {
        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = Uint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a + b; // Perform addition on the 4-bit values
        assert_eq!(result.to_string(), "1111"); // Binary 1111 (Addition result of 1100 + 0011)
    }

    #[test]
    fn test_from_u8_add() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010
        let b = Uint::<8>::from_u8(85); // Binary 01010101

        let result = a + b; // Perform addition on the 4-bit values
        assert_eq!(result.to_u8(), 170 + 85); // Expected result of addition between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_add() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010
        let b = Uint::<16>::from_u16(21845); // Binary 0101010101010101

        let result = a + b;
        assert_eq!(result.to_u16(), 43690 + 21845); // Expected result of addition between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_add() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = Uint::<32>::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a + b;
        assert_eq!(result.to_u32(), 2863311530 + 1431655765); // Expected result of addition between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_add() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = Uint::<64>::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a + b;
        assert_eq!(result.to_u64(), 12297829382473034410 + 6148914691236517205);
        // Expected result of addition between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_add() {
        let a = Uint::<128>::from_u128(12297829382473034410); // Binary 10101010
        let b = Uint::<128>::from_u128(6148914691236517205); // Binary 01010101

        let result = a + b;
        assert_eq!(result.to_u128(), 12297829382473034410 + 6148914691236517205);

        println!("{}", result.to_u128());
        // Expected result of addition between 10101010 and 01010101
    }

    #[test]
    fn test_from_u128_add_loop() {
        let clear_a = 12297829382473034410u128;
        let clear_b = 1u128;

        let a = Uint::<128>::from_u128(clear_a);
        let b = Uint::<128>::from_u128(clear_b);

        let mut result = &a + &b;
        let iterations = 100;
        for _ in 0..iterations {
            result = &result + &b;
        }
        assert_eq!(result.to_u128(), clear_a + 101);

        println!("{}", result.to_u128());
        // Expected result of addition between 10101010 and 01010101
    }

    #[test]
    fn test_uint_sub() {
        let a = Uint::<4>::from_u8(3);
        let b = Uint::<4>::from_u8(2);

        let result = a - b; // Perform subtraction on the 4-bit values
        assert_eq!(result.to_u8(), 3 - 2);
    }

    #[test]
    fn test_from_u8_sub() {
        let a = Uint::<8>::from_u8(170); // Binary 10101010
        let b = Uint::<8>::from_u8(100); // Binary 01100100

        let result = a - b;
        assert_eq!(result.to_u8(), 170 - 100); // Expected result of subtraction between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_sub() {
        let a = Uint::<16>::from_u16(43690); // Binary 1010101010101010
        let b = Uint::<16>::from_u16(21845); // Binary 0101010101010101

        let result = a - b;
        assert_eq!(result.to_u16(), 43690 - 21845); // Expected result of subtraction between 1010101010101010 and 0101010101010101
    }

    #[test]
    fn test_from_u32_sub() {
        let a = Uint::<32>::from_u32(2863311530); // Binary 10101010101010101010101010101010
        let b = Uint::<32>::from_u32(1431655765); // Binary 01010101010101010101010101010101

        let result = a - b;
        assert_eq!(result.to_u32(), 2863311530 - 1431655765); // Expected result of subtraction between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_sub() {
        let a = Uint::<64>::from_u64(12297829382473034410); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b = Uint::<64>::from_u64(6148914691236517205); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result = a - b;
        assert_eq!(result.to_u64(), 12297829382473034410 - 6148914691236517205);
        // Expected result of subtraction between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_sub() {
        let a = Uint::<128>::from_u128(170); // Binary 10101010
        let b = Uint::<128>::from_u128(85); // Binary 01010101

        let result = a - b;
        assert_eq!(result.to_u128(), 170 - 85); // Expected result of subtraction between 10101010 and 01010101
    }

    #[test]
    fn test_uint_mul() {
        let a = Uint::<4>::new(vec![true, true, false, false]); // Binary 1100
        let b = Uint::<4>::new(vec![false, false, true, true]); // Binary 0011

        let result = a * b; // Perform multiplication on the 4-bit values
        assert_eq!(result.to_string(), "0000"); // Binary 0000 (Multiplication result of 1100 * 0011)
    }
}
