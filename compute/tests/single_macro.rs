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
