use compute::prelude::*;

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
