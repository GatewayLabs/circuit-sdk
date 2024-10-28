use compute::prelude::*;

#[test]
fn test_uint_equality() {
    let a: GarbledUint8 = 123_u8.into();
    let b: GarbledUint8 = 123_u8.into();
    let c: GarbledUint8 = 124_u8.into();

    assert_eq!(&a, &b);
    assert_ne!(&a, &c);
}

#[test]
fn test_unsigned_comparison() {
    let a: GarbledUint8 = 100_u8.into();
    let b: GarbledUint8 = 150_u8.into();

    assert!(a < b);
    assert!(b > a);
    assert!(a != b);

    let c: GarbledUint8 = 200_u8.into();
    let d: GarbledUint8 = 200_u8.into();

    assert!(c == d);
    assert!(c <= d);
    assert!(c >= d);
}

#[test]
fn test_uint_edge_cases() {
    let zero: GarbledUint8 = 0_u8.into();
    let max: GarbledUint8 = u8::MAX.into();

    assert!(zero < max);
    assert!(max > zero);
    assert!(zero != max);
}

#[test]
fn test_uint_larger_comparison() {
    let a16: GarbledUint16 = 1000_u16.into();
    let b16: GarbledUint16 = 2000_u16.into();
    assert!(a16 < b16);

    let a32: GarbledUint32 = 10000_u32.into();
    let b32: GarbledUint32 = 20000_u32.into();
    assert!(a32 < b32);

    let a64: GarbledUint64 = 10000000000_u64.into();
    let b64: GarbledUint64 = 20000000000_u64.into();
    assert!(a64 < b64);

    let a128: GarbledUint128 = 100000000000000000000_u128.into();
    let b128: GarbledUint128 = 200000000000000000000_u128.into();
    assert!(a128 < b128);
}

// test signed integer comparison with different sizes
#[test]
#[ignore = "reason: int with negative values not implemented"]
fn test_int_comparison() {
    let d8: GarbledInt8 = (-100_i8).into();
    let e8: GarbledInt8 = 100_i8.into();

    assert!(d8 < e8);
}

#[test]
fn test_int_larger_comparison() {
    let a16: GarbledInt16 = 1000_i16.into();
    let b16: GarbledInt16 = 2000_i16.into();
    assert!(a16 < b16);

    let a32: GarbledInt32 = 10000_i32.into();
    let b32: GarbledInt32 = 20000_i32.into();
    assert!(a32 < b32);

    let a64: GarbledInt64 = 10000000000_i64.into();
    let b64: GarbledInt64 = 20000000000_i64.into();
    assert!(a64 < b64);

    let a128: GarbledInt128 = 100000000000000000000_i128.into();
    let b128: GarbledInt128 = 200000000000000000000_i128.into();
    assert!(a128 < b128);
}
