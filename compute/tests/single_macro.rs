use compute::prelude::*;

#[test]
fn test_macro_range_expr() {
    #[circuit(execute)]
    fn describe_number(n: u16) -> u16 {
        match n {
            1..=5 => 5,
            6..10 => 10,
            11..=20 => 20,
            _ => 100,
        }
    }

    let n = 3_u16;
    let result = describe_number(n);
    assert_eq!(result, 5);

    let n = 8_u16;
    let result = describe_number(n);
    assert_eq!(result, 10);

    let n = 15_u16;
    let result = describe_number(n);
    assert_eq!(result, 20);

    let n = 25_u16;
    let result = describe_number(n);
    assert_eq!(result, 100);
}

#[test]
fn test_macro_range_expr_bool() {
    #[circuit(execute)]
    fn describe_number(n: u16) -> bool {
        match n {
            1..=5 => true,
            6..=10 => false,
            11..=20 => true,
            _ => false,
        }
    }

    let n = 3_u16;
    let result = describe_number(n);
    assert!(result);

    let n = 8_u16;
    let result = describe_number(n);
    assert!(!result);

    let n = 15_u16;
    let result = describe_number(n);
    assert!(result);

    let n = 25_u16;
    let result = describe_number(n);
    assert!(!result);
}

#[test]
fn test_macro_range_if_let() {
    #[circuit(execute)]
    fn describe_number(n: u16) -> u16 {
        if let 1..=5 = n {
            5
        } else if let 6..10 = n {
            10
        } else if let 11..=20 = n {
            20
        } else if let 77 = n {
            77
        } else {
            100
        }
    }

    let n = 3_u16;
    let result = describe_number(n);
    assert_eq!(result, 5);

    let n = 8_u16;
    let result = describe_number(n);
    assert_eq!(result, 10);

    let n = 15_u16;
    let result = describe_number(n);
    assert_eq!(result, 20);

    let n = 77_u16;
    let result = describe_number(n);
    assert_eq!(result, 77);

    let n = 25_u16;
    let result = describe_number(n);
    assert_eq!(result, 100);
}
