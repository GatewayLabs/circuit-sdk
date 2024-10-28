use compute::prelude::*;

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

/*
#[test]
fn test_macro_embedded_constants() {
    #[circuit(execute)]
    fn embedded_constants(a: u8) -> u8 {
        let B = 20;
        if a > 5 {
            let A = 10;
            A + B
        }
    }

    let a = 10_u8;
    let result = embedded_constants(a);
    assert_eq!(result, 30_u8);
}
*/

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
