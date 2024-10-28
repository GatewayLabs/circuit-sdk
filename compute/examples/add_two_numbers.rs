use compute::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clear_a = 12297829382473034410u128;
    let clear_b = 424242424242u128;

    let a: GarbledUint128 = clear_a.into();
    let b: GarbledUint128 = clear_b.into();

    let result = &a + &b;
    let result: u128 = result.into();
    assert_eq!(result, clear_a + clear_b);
    Ok(())
}
