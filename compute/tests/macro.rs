use circuit_macro::circuit;
use compute::operations::circuits::builder::CircuitBuilder;
use compute::uint::GarbledUint8;

#[test]
fn test_macro_arithmetic() {
    #[circuit]
    fn multi_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let res = a * b;
        let res = res + c;
        res - d

        //let res = context.sub(res, d);
        //res
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let result: u8 = multi_arithmetic(a, b, c, d);
    assert_eq!(result, a * b + c - d);
}

#[test]
fn test_macro_addition() {
    #[circuit]
    fn addition(a: u8, b: u8) -> u8 {
        a + b
    }

    let a = 2_u8;
    let b = 5_u8;

    let result: u8 = addition(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_subtraction() {
    #[circuit]
    fn subtraction(a: u8, b: u8) -> u8 {
        a - b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result: u8 = subtraction(a, b);
    assert_eq!(result, a - b);
}

#[test]
fn test_macro_multiplication() {
    #[circuit]
    fn multiplication(a: u8, b: u8) -> u8 {
        a * b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result: u8 = multiplication(a, b);
    assert_eq!(result, a * b);
}

#[ignore = "division not yet supported"]
#[test]
fn test_macro_division() {
    #[circuit]
    fn division(a: u8, b: u8) -> u8 {
        a / b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result: u8 = division(a, b);
    assert_eq!(result, a / b);
}

#[ignore = "modulo not yet supported"]
#[test]
fn test_macro_remainder() {
    #[circuit]
    fn remainder(a: u8, b: u8) -> u8 {
        a % b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result: u8 = remainder(a, b);
    assert_eq!(result, a % b);
}

#[test]
fn test_macro_nested_arithmetic() {
    #[circuit]
    fn nested_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let res = a * b;
        let res = res + c;
        let res = res - d;
        res
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let result: u8 = nested_arithmetic(a, b, c, d);
    assert_eq!(result, a * b + c - d);
}

// test bitwise operations
#[test]
fn test_macro_bitwise_and() {
    #[circuit]
    fn bitwise_and(a: u8, b: u8) -> u8 {
        a & b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result: u8 = bitwise_and(a, b);
    assert_eq!(result, a & b);
}

#[test]
fn test_macro_bitwise_or() {
    #[circuit]
    fn bitwise_or(a: u8, b: u8) -> u8 {
        a | b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result: u8 = bitwise_or(a, b);
    assert_eq!(result, a | b);
}

#[test]
fn test_macro_bitwise_xor() {
    #[circuit]
    fn bitwise_xor(a: u8, b: u8) -> u8 {
        a ^ b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result: u8 = bitwise_xor(a, b);
    assert_eq!(result, a ^ b);
}

#[test]
fn test_macro_bitwise_not() {
    #[circuit]
    fn bitwise_not(a: u8) -> u8 {
        !a
    }

    let a = 2_u8;

    let result: u8 = bitwise_not(a);
    assert_eq!(result, !a);
}

#[test]
fn test_macro_bitwise_nand() {
    #[circuit]
    fn bitwise_nand(a: u8, b: u8) -> u8 {
        let and = a & b;
        !and
    }

    let a = 2_u8;
    let b = 3_u8;

    let result: u8 = bitwise_nand(a, b);
    assert_eq!(result, !(a & b));
}

#[test]
fn test_macro_bitwise_nor() {
    #[circuit]
    fn bitwise_nor(a: u8, b: u8) -> u8 {
        let or = a | b;
        !or
    }

    let a = 2_u8;
    let b = 3_u8;

    let result: u8 = bitwise_nor(a, b);
    assert_eq!(result, !(a | b));
}

#[test]
fn test_macro_bitwise_xnor() {
    #[circuit]
    fn bitwise_xnor(a: u8, b: u8) -> u8 {
        let xor = a ^ b;
        !xor
    }

    let a = 2_u8;
    let b = 3_u8;

    let result: u8 = bitwise_xnor(a, b);
    assert_eq!(result, !(a ^ b));
}
