use compute::uint::{GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8};

#[test]
fn test_display() {
    let a: GarbledUint8 = 170u8.into(); // Binary 10101010
    assert_eq!(format!("{}", a), "170");

    let b: GarbledUint16 = 43707u16.into(); // Binary 1010101010101011
    assert_eq!(format!("{}", b), "43707");

    let c: GarbledUint32 = 2863311530u32.into(); // Binary 10101010101010101010101010101010
    assert_eq!(format!("{}", c), "2863311530");

    let d: GarbledUint64 = 12297829382473034410u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    assert_eq!(format!("{}", d), "12297829382473034410");

    let e: GarbledUint128 = 12297829382473034410u128.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    assert_eq!(format!("{}", e), "12297829382473034410");
}

#[test]
fn test_from_u8() {
    let a: GarbledUint8 = 170u8.into(); // Binary 10101010
    let value: u8 = a.into();
    assert_eq!(value, 170);
}

#[test]
fn test_from_u16() {
    let a: GarbledUint16 = 43707u16.into(); // Binary 1010101010101011
    let value: u16 = a.into();
    assert_eq!(value, 43707);
}

#[test]
fn test_from_u32() {
    let a: GarbledUint32 = 2863311530u32.into(); // Binary 10101010101010101010101010101010
    let value: u32 = a.into();
    assert_eq!(value, 2863311530);
}

#[test]
fn test_from_u64() {
    let a: GarbledUint64 = 12297829382473034410u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let value: u64 = a.into();
    assert_eq!(value, 12297829382473034410);
}

#[test]
fn test_from_u128() {
    let a: GarbledUint128 = 12297829382473034410u128.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let value: u128 = a.into();
    assert_eq!(value, 12297829382473034410);
}
