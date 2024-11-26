use compute::prelude::*;

#[test]
fn test_divide_bug() {
    #[encrypted(execute)]
    fn divide(a: u8, b: u8) -> u8 {
        a / b
    }
    let d = divide(7_u8, 3_u8);
    assert_eq!(d, 2);
}
