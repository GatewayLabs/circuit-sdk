use std::vec;

use crate::operations::circuits::builder::CircuitBuilder;
use crate::operations::circuits::builder::GateOp;
use crate::uint::GarbledUint;

pub(crate) fn build_and_execute_xor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_inputs(vec![lhs, rhs]);

    // Add XOR gates for each bit
    builder.add_op(GateOp::Xor(1));

    // Build the circuit using the stack of operations
    builder.build_circuit();

    // Simulate the circuit
    builder.execute().expect("Failed to execute XOR circuit")
}

pub(crate) fn build_and_execute_and<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_inputs(vec![lhs, rhs]);

    // Add AND gates for each bit
    builder.add_op(GateOp::And(1));

    // Build the circuit using the stack of operations
    builder.build_circuit();

    // Simulate the circuit
    builder.execute().expect("Failed to execute AND circuit")
}

pub(crate) fn build_and_execute_or<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_inputs(vec![lhs, rhs]);

    // Add OR gates for each bit
    builder.add_op(GateOp::Or(1));

    // Build the circuit using the stack of operations
    builder.build_circuit();

    // Simulate the circuit
    builder.execute().expect("Failed to execute OR circuit")
}

pub(crate) fn build_and_execute_nand<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_inputs(vec![lhs, rhs]);

    // Add NAND gates for each bit
    builder.add_op(GateOp::And(1));
    builder.add_op(GateOp::Not);

    // Build the circuit using the stack of operations
    builder.build_circuit();

    // Simulate the circuit
    builder.execute().expect("Failed to execute NAND circuit")
}

pub(crate) fn build_and_execute_nor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_inputs(vec![lhs, rhs]);

    // Add NOR gates for each bit
    builder.add_op(GateOp::Or(1));
    builder.add_op(GateOp::Not);

    // Build the circuit using the stack of operations
    builder.build_circuit();

    // Simulate the circuit
    builder.execute().expect("Failed to execute NOR circuit")
}

pub(crate) fn build_and_execute_xnor<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_inputs(vec![lhs, rhs]);

    // Add XNOR gates for each bit
    builder.add_op(GateOp::Xnor(1));

    // Build the circuit using the stack of operations
    builder.build_circuit();

    // Simulate the circuit
    builder.execute().expect("Failed to execute XNOR circuit")
}

pub(crate) fn build_and_execute_addition<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> GarbledUint<N> {
    let mut builder = CircuitBuilder::default();
    builder.add_inputs(vec![lhs, rhs]);

    // Add XOR gates for each bit
    builder.add_op(GateOp::Add(1));

    // Build the circuit using the stack of operations
    builder.build_circuit();

    // Simulate the circuit
    builder.execute().expect("Failed to execute XOR circuit")
}

pub(crate) fn build_and_execute_equality<const N: usize>(
    lhs: &GarbledUint<N>,
    rhs: &GarbledUint<N>,
) -> bool {
    let mut builder = CircuitBuilder::default();
    builder.add_inputs(vec![lhs, rhs]);

    let mut result = builder.push_xnor(0, N as u32);

    for i in 1..N {
        let current_comparison = builder.push_xnor(i as u32, (N + i) as u32);
        result = builder.push_and(result, current_comparison);
    }

    // Simulate the circuit
    let result = builder.execute().expect("Failed to execute XNOR circuit");

    // Check if all bits are equal
    result.bits[0]
}
mod test {
    // test the helper functions
    use super::*;

    #[test]
    fn test_xor() {
        let a: GarbledUint<8> = 8_u8.into();
        let b: GarbledUint<8> = 42_u8.into();

        let result = build_and_execute_xor(&a, &b);

        let result: u8 = result.into();
        assert_eq!(result, 8_u8 ^ 42_u8); // Explicit precedence
    }

    #[test]
    fn test_and() {
        let a: GarbledUint<8> = 8_u8.into();
        let b: GarbledUint<8> = 42_u8.into();

        let result = build_and_execute_and(&a, &b);

        let result: u8 = result.into();
        assert_eq!(result, 8_u8 & 42_u8); // Explicit precedence
    }

    #[test]
    fn test_or() {
        let a: GarbledUint<8> = 8_u8.into();
        let b: GarbledUint<8> = 42_u8.into();

        let result = build_and_execute_or(&a, &b);

        let result: u8 = result.into();
        assert_eq!(result, 8_u8 | 42_u8); // Explicit precedence
    }

    #[test]
    fn test_nand() {
        let a: GarbledUint<8> = 8_u8.into();
        let b: GarbledUint<8> = 42_u8.into();

        let result = build_and_execute_nand(&a, &b);

        let result: u8 = result.into();
        assert_eq!(result, !(8_u8 & 42_u8)); // Explicit precedence
    }

    #[test]
    fn test_nor() {
        let a: GarbledUint<8> = 8_u8.into();
        let b: GarbledUint<8> = 42_u8.into();

        let result = build_and_execute_nor(&a, &b);

        let result: u8 = result.into();
        assert_eq!(result, !(8_u8 | 42_u8)); // Explicit precedence
    }

    #[test]
    fn test_xnor() {
        let a: GarbledUint<8> = 8_u8.into();
        let b: GarbledUint<8> = 42_u8.into();

        let result = build_and_execute_xnor(&a, &b);

        let result: u8 = result.into();
        assert_eq!(result, !(8_u8 ^ 42_u8)); // Explicit precedence
    }

    #[test]
    fn test_not() {
        let a: GarbledUint<8> = 8_u8.into();

        let mut builder = CircuitBuilder::default();
        builder.add_input(a.clone());
        builder.add_op(GateOp::Not);

        // Build the circuit using the stack of operations
        builder.build_circuit();

        // Execute the circuit
        let result = builder.execute().expect("Failed to execute circuit");

        let result: u8 = result.into();
        assert_eq!(result, !8_u8); // Explicit precedence
    }

    #[test]
    fn test_addition() {
        let a: GarbledUint<8> = 8_u8.into();
        let b: GarbledUint<8> = 42_u8.into();

        let result = build_and_execute_addition(&a, &b);

        let result: u8 = result.into();
        assert_eq!(result, 8_u8 + 42_u8); // Explicit precedence
    }

    #[test]
    fn test_equality() {
        let a: GarbledUint<8> = 8_u8.into();
        let b: GarbledUint<8> = 42_u8.into();

        let result = build_and_execute_equality(&a, &b);

        assert_eq!(result, false);
    }
}
