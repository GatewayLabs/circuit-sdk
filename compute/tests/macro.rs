use compute::prelude::*;

#[test]
fn test_macro_arithmetic_compiler() {
    #[circuit(compile)]
    fn multi_arithmetic(a: u8, b: u8, c: u8, d: u8) -> (Circuit, Vec<bool>) {
        let res = a * b;
        let res = res + c;
        res - d
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let (circuit, inputs) = multi_arithmetic(a, b, c, d);
    let result = get_executor().execute(&circuit, &inputs, &[]).unwrap();
    let result: GarbledUint<8> = GarbledUint::new(result);
    let result: u8 = result.into();
    assert_eq!(result, a * b + c - d);
}

#[test]
fn test_macro_arithmetic() {
    #[circuit(execute)]
    fn multi_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
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
    #[circuit(execute)]
    fn multi_arithmetic_u128(a: u8, b: u8, c: u8, d: u8) -> u8 {
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
    #[circuit(execute)]
    fn mixed_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let res = a * b;
        let res = context.add(&res, c);
        let res = res - d;
        context.mul(&res, a)
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
    #[circuit(execute)]
    fn addition(a: u8, b: u8) -> u8 {
        a + b
    }

    let a = 2_u8;
    let b = 5_u8;

    let result = addition(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_subtraction() {
    #[circuit(execute)]
    fn subtraction(a: u8, b: u8) -> u8 {
        a - b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = subtraction(a, b);
    assert_eq!(result, a - b);
}

#[test]
fn test_macro_multiplication() {
    #[circuit(execute)]
    fn multiplication(a: u8, b: u8) -> u8 {
        a * b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = multiplication(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_mux() {
    #[circuit(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        let condition = a == b;
        &context.mux(&condition, &a, &b)
    }

    let a = 5_u8;
    let b = 10_u8;

    let result = mux_circuit(a, b);
    assert_eq!(result, b);
}

#[test]
fn test_macro_if_else() {
    #[circuit(execute)]
    fn mux_circuit(a: T, b: T) -> T {
        if a == b {
            let c = a * b;
            c + a
        } else {
            a + b
        }
    }

    let a = 10_u16;
    let b = 5_u16;

    let result: u16 = mux_circuit(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_if_else2() {
    #[circuit(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        let true_branch = a * b;
        let false_branch = a + b;
        let condition = a == b;
        if condition {
            true_branch
        } else {
            false_branch
        }
    }

    let a = 10_u8;
    let b = 5_u8;

    let result = mux_circuit(a, b);
    assert_eq!(result, a + b);

    let a = 5_u8;
    let result = mux_circuit(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_if_else3() {
    #[circuit(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        if a == b {
            a * b
        } else {
            a + b
        }
    }

    let a = 4_u8;
    let b = 4_u8;

    let result = mux_circuit(a, b);
    assert_eq!(result, a * b);

    let a = 5_u8;
    let result = mux_circuit(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_if_else4() {
    #[circuit(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        if a == b {
            let c = a * b;
            c + a
        } else {
            let x = a + b;
            x * x
        }
    }

    let a = 5_u8;
    let b = 7_u8;

    let result = mux_circuit(a, b);
    assert_eq!(result, (a + b) * (a + b));
}

#[ignore = "division not yet supported"]
#[test]
fn test_macro_division() {
    #[circuit(execute)]
    fn division(a: u8, b: u8) -> u8 {
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
    #[circuit(execute)]
    fn remainder(a: u8, b: u8) -> u8 {
        a % b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = remainder(a, b);
    assert_eq!(result, a % b);
}

#[test]
fn test_macro_nested_arithmetic() {
    #[circuit(execute)]
    fn nested_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let res = a * b;
        let res = res + c;
        res - d
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
    #[circuit(execute)]
    fn bitwise_and(a: u8, b: u8) -> u8 {
        a & b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_and(a, b);
    assert_eq!(result, a & b);
}

#[test]
fn test_macro_bitwise_or() {
    #[circuit(execute)]
    fn bitwise_or(a: u8, b: u8) -> u8 {
        a | b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_or(a, b);
    assert_eq!(result, a | b);
}

#[test]
fn test_macro_bitwise_xor() {
    #[circuit(execute)]
    fn bitwise_xor(a: u8, b: u8) -> u8 {
        a ^ b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_xor(a, b);
    assert_eq!(result, a ^ b);
}

#[test]
fn test_macro_bitwise_not() {
    #[circuit(execute)]
    fn bitwise_not(a: u8) -> u8 {
        !a
    }

    let a = 2_u8;

    let result = bitwise_not(a);
    assert_eq!(result, !a);
}

#[test]
fn test_macro_bitwise_nand() {
    #[circuit(execute)]
    fn bitwise_nand(a: u8, b: u8) -> u8 {
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
    #[circuit(execute)]
    fn bitwise_nor(a: u8, b: u8) -> u8 {
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
    #[circuit(execute)]
    fn bitwise_xnor(a: u8, b: u8) -> u8 {
        let xor = a ^ b;
        !xor
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_xnor(a, b);
    assert_eq!(result, !(a ^ b));
}

#[test]
fn test_macro_equal() {
    #[circuit(execute)]
    fn equal(a: u8, b: u8) -> u8 {
        if a == b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = equal(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_not_equal() {
    #[circuit(execute)]
    fn not_equal(a: u8, b: u8) -> u8 {
        if a != b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = not_equal(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_greater_than() {
    #[circuit(execute)]
    fn greater_than(a: u8, b: u8) -> u8 {
        if a > b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = greater_than(a, b);
    assert_eq!(result, a + b);

    let a = 3_u8;
    let result = greater_than(a, b);
    assert_eq!(result, a + b);

    let a = 4_u8;
    let result = greater_than(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_greater_than_or_equal() {
    #[circuit(execute)]
    fn greater_than_or_equal(a: u8, b: u8) -> u8 {
        if a >= b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = greater_than_or_equal(a, b);
    assert_eq!(result, a + b);

    let a = 3_u8;
    let result = greater_than_or_equal(a, b);
    assert_eq!(result, a * b);

    let a = 4_u8;
    let result = greater_than_or_equal(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_less_than() {
    #[circuit(execute)]
    fn less_than(a: u8, b: u8) -> u8 {
        if a < b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = less_than(a, b);
    assert_eq!(result, a * b);

    let a = 3_u8;
    let result = less_than(a, b);
    assert_eq!(result, a + b);

    let a = 4_u8;
    let result = less_than(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_less_than_or_equal() {
    #[circuit(execute)]
    fn less_than_or_equal(a: u8, b: u8) -> u8 {
        if a <= b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = less_than_or_equal(a, b);
    assert_eq!(result, a * b);

    let a = 3_u8;
    let result = less_than_or_equal(a, b);
    assert_eq!(result, a * b);

    let a = 4_u8;
    let result = less_than_or_equal(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_bool_return() {
    #[circuit(execute)]
    fn equal(a: u8, b: u8) -> bool {
        a == b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = equal(a, b);
    assert!(!result);
}

// div
#[test]
fn test_macro_div() {
    #[circuit(execute)]
    fn div(a: u8, b: u8) -> u8 {
        a / b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = div(a, b);
    assert_eq!(result, a / b);
}

#[test]
fn test_macro_div_with_remainder() {
    #[circuit(execute)]
    fn div(a: u8, b: u8) -> u8 {
        a / b
    }

    let a = 20_u8;
    let b = 3_u8;

    let result = div(a, b);
    assert_eq!(result, a / b);
}

#[test]
fn test_macro_div_with_remainder2() {
    #[circuit(execute)]
    fn div(a: u8, b: u8) -> u8 {
        a / b
    }

    let a = 20_u8;
    let b = 7_u8;

    let result = div(a, b);
    assert_eq!(result, a / b);
}

// rem
#[test]
fn test_macro_rem() {
    #[circuit(execute)]
    fn rem(a: u8, b: u8) -> u8 {
        a % b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = rem(a, b);
    assert_eq!(result, a % b);
}

#[test]
fn test_macro_rem_with_remainder() {
    #[circuit(execute)]
    fn rem(a: u8, b: u8) -> u8 {
        a % b
    }

    let a = 20_u8;
    let b = 3_u8;

    let result = rem(a, b);
    assert_eq!(result, a % b);
}

#[test]
fn test_macro_constants() {
    #[circuit(execute)]
    fn constants(a: u8) -> u8 {
        a + 20
    }

    let a = 10_u8;
    let result = constants(a);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_macro_embedded_constants() {
    #[circuit(execute)]
    fn embedded_constants(a: u8) -> u8 {
        let B = 20;
        a + B
    }

    let a = 10_u8;
    let result = embedded_constants(a);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_order_of_operations() {
    #[circuit(execute)]
    fn order_of_operations(a: u16, b: u16, c: u16) -> u16 {
        a + b * c
    }

    let a = 10_u16;
    let b = 20_u16;
    let c = 30_u16;
    let result = order_of_operations(a, b, c);
    assert_eq!(result, 610_u16);
}

#[test]
fn test_order_of_operations2() {
    #[circuit(execute)]
    fn order_of_operations(a: u16, b: u16, c: u16) -> u16 {
        (a + b) * c
    }

    let a = 10_u16;
    let b = 20_u16;
    let c = 30_u16;
    let result = order_of_operations(a, b, c);
    assert_eq!(result, 900);
}

#[test]
fn test_add_assign() {
    #[circuit(execute)]
    fn add_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c += b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = add_assign(a, b);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_sub_assign() {
    #[circuit(execute)]
    fn sub_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c -= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = sub_assign(a, b);
    assert_eq!(result, 246_u8);
}

#[test]
fn test_mul_assign() {
    #[circuit(execute)]
    fn mul_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c *= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = mul_assign(a, b);
    assert_eq!(result, 200_u8);
}

#[test]
fn test_div_assign() {
    #[circuit(execute)]
    fn div_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c /= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = div_assign(a, b);
    assert_eq!(result, 0_u8);
}

#[test]
fn test_rem_assign() {
    #[circuit(execute)]
    fn rem_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c %= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = rem_assign(a, b);
    assert_eq!(result, 10_u8);
}

#[test]
fn test_bitand_assign() {
    #[circuit(execute)]
    fn bitand_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c &= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = bitand_assign(a, b);
    assert_eq!(result, 0_u8);
}

#[test]
fn test_bitor_assign() {
    #[circuit(execute)]
    fn bitor_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c |= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = bitor_assign(a, b);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_bitxor_assign() {
    #[circuit(execute)]
    fn bitxor_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c ^= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = bitxor_assign(a, b);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_if_elif_else() {
    #[circuit(execute)]
    fn if_statement(a: u8) -> u8 {
        if a > 100 {
            a + 1
        } else if a > 50 {
            a + 2
        } else {
            a
        }
    }

    let a = 60_u8;
    let result = if_statement(a);
    assert_eq!(result, 62_u8);

    let a = 110_u8;
    let result = if_statement(a);
    assert_eq!(result, 111_u8);

    let a = 40_u8;
    let result = if_statement(a);
    assert_eq!(result, 40_u8);
}

#[test]
fn test_nested_if() {
    #[circuit(execute)]
    fn nested_if(a: u8) -> u8 {
        if a > 100 {
            if a > 200 {
                a + 1
            } else {
                a + 2
            }
        } else {
            a
        }
    }

    let a = 150_u8;
    let result = nested_if(a);
    assert_eq!(result, 152_u8);

    let a = 250_u8;
    let result = nested_if(a);
    assert_eq!(result, 251_u8);

    let a = 50_u8;
    let result = nested_if(a);
    assert_eq!(result, 50_u8);
}

#[test]
fn test_nested_if_else() {
    #[circuit(execute)]
    fn nested_if_else(a: u8) -> u8 {
        if a > 100 {
            if a > 200 {
                a + 1
            } else {
                a + 2
            }
        } else {
            if a > 50 {
                a + 3
            } else {
                a + 4
            }
        }
    }

    let a = 150_u8;
    let result = nested_if_else(a);
    assert_eq!(result, 152_u8);

    let a = 250_u8;
    let result = nested_if_else(a);
    assert_eq!(result, 251_u8);

    let a = 60_u8;
    let result = nested_if_else(a);
    assert_eq!(result, 63_u8);

    let a = 40_u8;
    let result = nested_if_else(a);
    assert_eq!(result, 44_u8);
}

#[test]
fn test_nested_if_else_if() {
    #[circuit(execute)]
    fn nested_if_else_if(a: u8) -> u8 {
        if a > 100 {
            if a > 200 {
                a + 1
            } else {
                a + 2
            }
        } else if a > 50 {
            a + 3
        } else {
            a + 4
        }
    }

    let a = 150_u8;
    let result = nested_if_else_if(a);
    assert_eq!(result, 152_u8);

    let a = 250_u8;
    let result = nested_if_else_if(a);
    assert_eq!(result, 251_u8);

    let a = 60_u8;
    let result = nested_if_else_if(a);
    assert_eq!(result, 63_u8);

    let a = 40_u8;
    let result = nested_if_else_if(a);
    assert_eq!(result, 44_u8);
}

#[test]
fn test_if_else() {
    #[circuit(execute)]
    fn if_else(a: u8) -> u8 {
        if a > 100 {
            a + 1
        } else {
            a + 2
        }
    }

    let a = 150_u8;
    let result = if_else(a);
    assert_eq!(result, 151_u8);

    let a = 50_u8;
    let result = if_else(a);
    assert_eq!(result, 52_u8);
}

#[test]
fn test_macro_bool_literal() {
    #[circuit(execute)]
    fn boolean_literal(a: bool) -> bool {
        let x = false;
        let y = true;

        if a {
            x
        } else {
            y
        }
    }

    let bool1 = true;
    let result = boolean_literal(bool1);
    assert_eq!(result, !bool1);
}

#[test]
fn test_macro_bool_literal2() {
    #[circuit(execute)]
    fn boolean_literal2(a: bool) -> bool {
        if a {
            false
        } else {
            true
        }
    }

    let bool1 = false;
    let result = boolean_literal2(bool1);
    assert_eq!(result, !bool1);
}

#[test]
fn macro_test_if_assign() {
    #[circuit(execute)]
    fn if_test(a: u8) -> u8 {
        let y = 22;

        let mut c = 100;
        if a == 1 {
            c = c + 1;
            c
        } else {
            c = y + 1;
            c
        }
    }

    let a = 1_u8;
    let result = if_test(a);
    assert_eq!(result, 101);

    let a = 2_u8;
    let result = if_test(a);
    assert_eq!(result, 23);
}

#[test]
fn macro_test_assignment2() {
    #[circuit(execute)]
    fn assignment_test2(a: u8) -> u8 {
        let mut x = 11;
        x = a + 1;
        x
    }

    let a = 42_u8;
    let result = assignment_test2(a);
    assert_eq!(result, 43);
}

#[test]
fn macro_test_assignment() {
    #[circuit(execute)]
    fn assignment_test(a: u8) -> u8 {
        let mut x = 11;
        x = a;
        x
    }

    let a = 42_u8;
    let result = assignment_test(a);
    assert_eq!(result, 42);
}

#[test]
fn test_macro_match() {
    #[circuit(execute)]
    fn match_test(a: u8) -> u8 {
        match a {
            1 => 7,
            2 => 8,
            3 => 9,
            _ => 10,
        }
    }

    let a = 1_u8;
    let result = match_test(a);
    assert_eq!(result, 7_u8);

    let a = 2_u8;
    let result = match_test(a);
    assert_eq!(result, 8_u8);

    let a = 3_u8;
    let result = match_test(a);
    assert_eq!(result, 9_u8);

    let a = 4_u8;
    let result = match_test(a);
    assert_eq!(result, 10_u8);
}

#[test]
fn test_macro_match_with_expr() {
    #[circuit(execute)]
    fn match_test_with_expr(a: u8) -> u8 {
        match a {
            1 => {
                let b = 5;
                b + 2
            }
            2 => 8,
            3 => 9,
            _ => 10,
        }
    }

    let a = 1_u8;
    let result = match_test_with_expr(a);
    assert_eq!(result, 7_u8);

    let a = 2_u8;
    let result = match_test_with_expr(a);
    assert_eq!(result, 8_u8);

    let a = 3_u8;
    let result = match_test_with_expr(a);
    assert_eq!(result, 9_u8);

    let a = 4_u8;
    let result = match_test_with_expr(a);
    assert_eq!(result, 10_u8);
}

#[test]
fn test_macro_match_with_block() {
    #[circuit(execute)]
    fn match_test_with_block(a: u8) -> u8 {
        match a {
            1 => {
                let b = 5;
                b + 2
            }
            2 => {
                let c = 6;
                c + 2
            }
            3 => {
                let d = 7;
                d + 2
            }
            _ => 10,
        }
    }

    let a = 1_u8;
    let result = match_test_with_block(a);
    assert_eq!(result, 7_u8);

    let a = 2_u8;
    let result = match_test_with_block(a);
    assert_eq!(result, 8_u8);

    let a = 3_u8;
    let result = match_test_with_block(a);
    assert_eq!(result, 9_u8);

    let a = 4_u8;
    let result = match_test_with_block(a);
    assert_eq!(result, 10_u8);
}

#[test]
fn macro_test_if_with_consts() {
    #[circuit(execute)]
    fn if_test(a: u8) -> u8 {
        if a == 42 {
            a + 1
        } else {
            54
        }
    }

    let a = 42_u8;
    let result = if_test(a);
    assert_eq!(result, 43);

    let a = 43_u8;
    let result = if_test(a);
    assert_eq!(result, 54);
}
