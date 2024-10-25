use circuit_macro::circuit;
use compute::operations::circuits::builder::CircuitBuilder;
use compute::uint::GarbledUint;

#[test]
fn test_macro_arithmetic() {
    #[circuit]
    fn multi_arithmetic(a: T, b: T, c: T, d: T) -> T {
        let res = a * b;
        let res = res + c;
        res - d
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let result = multi_arithmetic(a, b, c, d);
    assert_eq!(result, a * b + c - d);
}

#[test]
fn test_macro_arithmetic_u128() {
    #[circuit]
    fn multi_arithmetic_u128(a: T, b: T, c: T, d: T) -> T {
        let res = a + b;
        let res = res + c;
        res - d
    }

    let a = 2_u128;
    let b = 5_u128;
    let c = 3_u128;
    let d = 4_u128;

    let result = multi_arithmetic_u128(a, b, c, d);
    assert_eq!(result, a + b + c - d);
}

#[test]
fn test_macro_mixed_arithmetic() {
    #[circuit]
    fn mixed_arithmetic(a: T, b: T, c: T, d: T) -> T {
        let res = a.clone() * b;
        let res = context.add(res, c);
        let res = res - d;
        context.mul(res, a)
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let result = mixed_arithmetic(a, b, c, d);
    assert_eq!(result, ((a * b + c - d) * a));
}

#[test]
fn test_macro_addition() {
    #[circuit]
    fn addition(a: T, b: T) -> T {
        a + b
    }

    let a = 2_u8;
    let b = 5_u8;

    let result = addition(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_subtraction() {
    #[circuit]
    fn subtraction(a: T, b: T) -> T {
        a - b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = subtraction(a, b);
    assert_eq!(result, a - b);
}

#[test]
fn test_macro_multiplication() {
    #[circuit]
    fn multiplication(a: T, b: T) -> T {
        a * b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = multiplication(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_mux() {
    #[circuit]
    fn mux_circuit(s: T, a: T, b: T) -> T {
        context.mux(s, a, b)
    }

    let s = 0_u8;
    let a = 5_u8;
    let b = 10_u8;

    let result = mux_circuit(s, a, b);
    assert_eq!(result, b);
}

#[test]
fn test_macro_mux3() {
    #[circuit]
    fn mux_circuit(s: T, a: T, b: T) -> T {
        let true_branch = a.clone() * b.clone();
        let false_branch = a + b;
        context.mux(s, true_branch, false_branch)
    }

    let s = 0_u8;
    let a = 10_u8;
    let b = 7_u8;

    // false case
    let result = mux_circuit(s, a, b);
    assert_eq!(result, a + b);

    // true case
    let s = 0b11111111_u8;
    let result = mux_circuit(s, a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_if_else() {
    #[circuit]
    fn mux_circuit(s: T, a: T, b: T) -> T {
        if s {
            a.clone() * b.clone()
        } else {
            a + b
        }
    }

    let s = 0_u8;
    let a = 10_u8;
    let b = 5_u8;

    let result = mux_circuit(s, a, b);
    assert_eq!(result, a + b);

    let s = 0b11111111_u8;
    let result = mux_circuit(s, a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_if_else2() {
    #[circuit]
    fn mux_circuit(s: T, a: T, b: T) -> T {
        let true_branch = a.clone() * b.clone();
        let false_branch = a + b;
        if s {
            true_branch
        } else {
            false_branch
        }
    }

    let s = 0_u8;
    let a = 10_u8;
    let b = 5_u8;

    let result = mux_circuit(s, a, b);
    assert_eq!(result, a + b);

    let s = 0b11111111_u8;
    let result = mux_circuit(s, a, b);
    assert_eq!(result, a * b);
}

#[ignore = "division not yet supported"]
#[test]
fn test_macro_division() {
    #[circuit]
    fn division(a: T, b: T) -> T {
        a / b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = division(a, b);
    assert_eq!(result, a / b);
}

#[ignore = "modulo not yet supported"]
#[test]
fn test_macro_remainder() {
    #[circuit]
    fn remainder(a: T, b: T) -> T {
        a % b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = remainder(a, b);
    assert_eq!(result, a % b);
}

#[test]
fn test_macro_nested_arithmetic() {
    #[circuit]
    fn nested_arithmetic(a: T, b: T, c: T, d: T) -> T {
        let res = a * b;
        let res = res + c;
        let res = res - d;
        res
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let result = nested_arithmetic(a, b, c, d);
    assert_eq!(result, a * b + c - d);
}

// test bitwise operations
#[test]
fn test_macro_bitwise_and() {
    #[circuit]
    fn bitwise_and(a: T, b: T) -> T {
        a & b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_and(a, b);
    assert_eq!(result, a & b);
}

#[test]
fn test_macro_bitwise_or() {
    #[circuit]
    fn bitwise_or(a: T, b: T) -> T {
        a | b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_or(a, b);
    assert_eq!(result, a | b);
}

#[test]
fn test_macro_bitwise_xor() {
    #[circuit]
    fn bitwise_xor(a: T, b: T) -> T {
        a ^ b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_xor(a, b);
    assert_eq!(result, a ^ b);
}

#[test]
fn test_macro_bitwise_not() {
    #[circuit]
    fn bitwise_not(a: T) -> T {
        !a
    }

    let a = 2_u8;

    let result = bitwise_not(a);
    assert_eq!(result, !a);
}

#[test]
fn test_macro_bitwise_nand() {
    #[circuit]
    fn bitwise_nand(a: T, b: T) -> T {
        let and = a & b;
        !and
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_nand(a, b);
    assert_eq!(result, !(a & b));
}

#[test]
fn test_macro_bitwise_nor() {
    #[circuit]
    fn bitwise_nor(a: T, b: T) -> T {
        let or = a | b;
        !or
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_nor(a, b);
    assert_eq!(result, !(a | b));
}

#[test]
fn test_macro_bitwise_xnor() {
    #[circuit]
    fn bitwise_xnor(a: T, b: T) -> T {
        let xor = a ^ b;
        !xor
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_xnor(a, b);
    assert_eq!(result, !(a ^ b));
}
