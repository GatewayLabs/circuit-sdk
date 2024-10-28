use compute::prelude::*;

#[test]
fn test_display() {
    let a: GarbledInt8 = 123_i8.into();
    assert_eq!(format!("{}", a), "123");

    let b: GarbledInt16 = 12345_i16.into();
    assert_eq!(format!("{}", b), "12345");

    let c: GarbledInt32 = 1234567890_i32.into();
    assert_eq!(format!("{}", c), "1234567890");

    let d: GarbledInt64 = 123456789012345_i64.into();
    assert_eq!(format!("{}", d), "123456789012345");

    let e: GarbledInt128 = 1234567890123456789012345_i128.into();
    assert_eq!(format!("{}", e), "1234567890123456789012345");

    let f: GarbledInt8 = (-123_i8).into();
    assert_eq!(format!("{}", f), "-123");

    let g: GarbledInt16 = (-12345_i16).into();
    assert_eq!(format!("{}", g), "-12345");

    let h: GarbledInt32 = (-1234567890_i32).into();
    assert_eq!(format!("{}", h), "-1234567890");

    let i: GarbledInt64 = (-123456789012345_i64).into();
    assert_eq!(format!("{}", i), "-123456789012345");

    let j: GarbledInt128 = (-1234567890123456789012345_i128).into();
    assert_eq!(format!("{}", j), "-1234567890123456789012345");
}

#[test]
fn test_from_negative_i8() {
    let a: GarbledInt8 = (-2_i8).into(); // Two's complement binary for -2 is 11111110
    let result: i8 = a.into();
    assert_eq!(result, -2_i8);
}

#[test]
fn test_from_positive_i8() {
    let a: GarbledInt8 = 3_i8.into(); // Binary for 3 is 00000011
    let result: i8 = a.into();
    assert_eq!(result, 3);
}

#[test]
fn test_from_negative_i16() {
    let a: GarbledInt16 = (-21845_i16).into(); // Two's complement binary for -21845 is 1010101010101011
    let result: i16 = a.into();
    assert_eq!(result, -21845);
}

#[test]
fn test_from_positive_i16() {
    let a: GarbledInt16 = 21845_i16.into(); // Binary for 21845 is 0101010101010101
    let result: i16 = a.into();
    assert_eq!(result, 21845);
}

#[test]
fn test_from_negative_i32() {
    let a: GarbledInt32 = (-1431655765_i32).into(); // Two's complement binary for -1431655765 is 10101010101010101010101010101011
    let result: i32 = a.into();
    assert_eq!(result, -1431655765);
}

#[test]
fn test_from_positive_i32() {
    let a: GarbledInt32 = 1431655765_i32.into(); // Binary for 1431655765 is 01010101010101010101010101010101
    let result: i32 = a.into();
    assert_eq!(result, 1431655765);
}

#[test]
fn test_from_negative_i64() {
    let a: GarbledInt64 = (-6148914691236517205_i64).into(); // Two's complement binary for -6148914691236517205 is 1010101010101010101010101010101010101010101010101010101010101011
    let result: i64 = a.into();
    assert_eq!(result, -6148914691236517205);
}

#[test]
fn test_from_positive_i64() {
    let a: GarbledInt64 = 6148914691236517205_i64.into(); // Binary for 6148914691236517205 is 0101010101010101010101010101010101010101010101010101010101010101
    let result: i64 = a.into();
    assert_eq!(result, 6148914691236517205);
}

#[test]
fn test_from_negative_i128() {
    let a: GarbledInt128 = (-6148914691236517205_i128).into(); // Two's complement binary for -6148914691236517205 is 1010101010101010101010101010101010101010101010101010101010101011
    let result: i128 = a.into();
    assert_eq!(result, -6148914691236517205);
}

#[test]
fn test_from_positive_i128() {
    let a: GarbledInt128 = 6148914691236517205_i128.into(); // Binary for 6148914691236517205 is 0101010101010101010101010101010101010101010101010101010101010101
    let result: i128 = a.into();
    assert_eq!(result, 6148914691236517205);
}

#[test]
fn test_from_uint_to_int_i8() {
    let uint: GarbledUint8 = 170_u8.into(); // 10101010 (unsigned)
    let int: GarbledInt8 = uint.into(); // Interpreted as -86 (two's complement signed)
    let result: i8 = int.into();
    assert_eq!(result, 170_u8 as i8);
}

#[test]
fn test_from_uint_to_int_i16() {
    let uint: GarbledUint16 = 43707_u16.into(); // 1010101010101011 (unsigned)
    let int: GarbledInt16 = uint.into(); // Interpreted as -21845 (two's complement signed)
    let result: i16 = int.into();
    assert_eq!(result, 43707_u16 as i16);
}

#[test]
fn test_from_uint_to_int_i32() {
    let uint: GarbledUint32 = 2863311530_u32.into(); // 10101010101010101010101010101010 (unsigned)
    let int: GarbledInt32 = uint.into(); // Interpreted as -1431655766 (two's complement signed)
    let result: i32 = int.into();
    assert_eq!(result, 2863311530_u32 as i32);
}

#[test]
fn test_from_uint_to_int_i64() {
    let uint: GarbledUint64 = 12297829382473034410_u64.into(); // 1010101010101010101010101010101010101010101010101010101010101010 (unsigned)
    let int: GarbledInt64 = uint.into(); // Interpreted as -6148914691236517206 (two's complement signed)
    let result: i64 = int.into();
    assert_eq!(result, 12297829382473034410_u64 as i64);
}

#[test]
fn test_from_uint_to_int_i128() {
    let uint: GarbledUint128 = 12297829382473034410_u128.into(); // 1010101010101010101010101010101010101010101010101010101010101010 (unsigned)
    let int: GarbledInt128 = uint.into(); // Interpreted as -6148914691236517206 (two's complement signed)
    let result: i128 = int.into();
    assert_eq!(result, 12297829382473034410_u128 as i128);
}
