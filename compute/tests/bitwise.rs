use compute::prelude::*;

#[test]
fn test_uint_xor() {
    let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    let result: u8 = (a ^ b).into();
    assert_eq!(result, 170_u8 ^ 85_u8); // Expected result of XOR between 10101010 and 01010101

    let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    let result: u16 = (&a ^ &b).into();
    assert_eq!(result, 43690_u16 ^ 21845_u16); // Expected result of XOR between 1010101010101010 and 0101010101010101

    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    let result: u32 = (&a ^ &b).into();
    assert_eq!(result, 4294967295); // Expected result of XOR between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    let result: u64 = (&a ^ &b).into();
    assert_eq!(result, 18446744073709551615); // Expected result of XOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

    let result: u128 = (&a ^ &b).into();
    assert_eq!(result, 255); // Expected result of XOR between 10101010 and 01010101
}

#[test]
fn test_uint_xor_assign() {
    let mut a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    a ^= b;
    assert_eq!(<GarbledUint<8> as Into<u8>>::into(a), 170_u8 ^ 85_u8); // Expected result of XOR between 10101010 and 01010101

    let mut a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    a ^= b;
    assert_eq!(
        <GarbledUint<16> as Into<u16>>::into(a),
        43690_u16 ^ 21845_u16
    ); // Expected result of XOR between 1010101010101010 and 0101010101010101

    let mut a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    a ^= b;
    assert_eq!(
        <GarbledUint<32> as Into<u32>>::into(a),
        2863311530_u32 ^ 1431655765_u32
    ); // Expected result of XOR between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let mut a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    a ^= b;
    assert_eq!(
        <GarbledUint<64> as Into<u64>>::into(a),
        12297829382473034410_u64 ^ 6148914691236517205_u64
    );
    // Expected result of XOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let mut a: GarbledUint128 = 170_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

    a ^= b;
    assert_eq!(
        <GarbledUint<128> as Into<u128>>::into(a),
        170_u128 ^ 85_u128
    ); // Expected result of XOR between 10101010 and 01010101
}

#[test]
fn test_int_xor_assign() {
    let mut a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    a ^= b;
    assert_eq!(<GarbledInt8 as Into<i8>>::into(a), -86_i8 ^ -43_i8); // Expected result of XOR between 10101010 and 11010101

    let mut a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt16 = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

    a ^= b;
    assert_eq!(
        <GarbledInt16 as Into<i16>>::into(a),
        -21846_i16 ^ -10923_i16
    ); // Expected result of XOR between 1010101010101010 and 1101010101010101

    let mut a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
    let b: GarbledInt32 = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

    a ^= b;
    assert_eq!(
        <GarbledInt32 as Into<i32>>::into(a),
        -1431655766_i32 ^ -715827883_i32
    );
    // Expected result of XOR between 10101010101010101010101010101010 and 11010101010101010101010101010101

    let mut a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 110101010101010101010

    a ^= b;
    assert_eq!(
        <GarbledInt64 as Into<i64>>::into(a),
        -6148914691236517206_i64 ^ -3074457345618258603_i64
    );

    let mut a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt128 = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    a ^= b;
    assert_eq!(
        <GarbledInt128 as Into<i128>>::into(a),
        -6148914691236517206_i128 ^ -3074457345618258603_i128
    );
}

#[test]
fn test_int_xor() {
    let a: GarbledInt8 = (-86_i8).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    let result: i8 = (a ^ b).into();
    assert_eq!(result, -86_i8 ^ -43_i8); // Expected result of XOR between 10101010 and 11010101

    let a: GarbledInt<16> = (-21846_i16).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt<16> = (-10923_i16).into(); // Two's complement binary for -10923 is 1101010101010101

    let result: i16 = (a ^ b).into();
    assert_eq!(result, -21846_i16 ^ -10923_i16); // Expected result of XOR between 1010101010101010 and 1101010101010101

    let a: GarbledUint8 = 17_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    let result: u8 = (a & b).into();
    assert_eq!(result, 17_u8 & 85_u8); // Expected result of AND between 10101010 and 01010101

    let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    let result: u16 = (a & b).into();
    assert_eq!(result, 43690 & 21845); // Expected result of AND between 1010101010101010 and 0101010101010101

    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    let result: u32 = (a & b).into();
    assert_eq!(result, 2863311530 & 1431655765); // Expected result of AND between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    let result: u64 = (a & b).into();
    assert_eq!(result, 12297829382473034410 & 6148914691236517205);
    // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

    let result: u128 = (a & b).into();
    assert_eq!(result, 170 & 85); // Expected result of AND between 10101010 and 01010101
}

#[test]
fn test_uint_or() {
    let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    let result: u8 = (a | b).into();
    assert_eq!(result, 170 | 85); // Expected result of OR between 10101010 and 01010101

    let a: GarbledUint16 = 43707_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    let result: u16 = (a | b).into();
    assert_eq!(result, 43707 | 21845); // Expected result of OR between 1010101010101011 and 0101010101010101

    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    let result: u32 = (a | b).into();
    assert_eq!(result, 2863311530 | 1431655765); // Expected result of OR between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    let result: u64 = (a | b).into();
    assert_eq!(result, 12297829382473034410 | 6148914691236517205);
    // Expected result of OR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

    let result: u128 = (a | b).into();
    assert_eq!(result, 170 | 85); // Expected result of OR between 10101010 and 01010101
}

#[test]
fn test_uint_or_assign() {
    let mut a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    a |= b;
    assert_eq!(<GarbledUint<8> as Into<u8>>::into(a), 170 | 85); // Expected result of OR between 10101010 and 01010101

    let mut a: GarbledUint16 = 43707_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    a |= b;
    assert_eq!(<GarbledUint<16> as Into<u16>>::into(a), 43707 | 21845); // Expected result of OR between 1010101010101011 and 0101010101010101

    let mut a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    a |= b;
    assert_eq!(
        <GarbledUint<32> as Into<u32>>::into(a),
        2863311530 | 1431655765
    ); // Expected result of OR between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let mut a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    a |= b;
    assert_eq!(
        <GarbledUint<64> as Into<u64>>::into(a),
        12297829382473034410 | 6148914691236517205
    );
    // Expected result of OR between 101010101010101010101010101
    // 0101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let mut a: GarbledUint128 = 170_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

    a |= b;
    assert_eq!(<GarbledUint<128> as Into<u128>>::into(a), 170 | 85); // Expected result of OR between 10101010 and 01010101
}

#[test]
fn test_int_or() {
    let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    let result: i8 = (a | b).into();
    assert_eq!(result, -86_i8 | -43_i8); // Expected result of OR between 10101010 and 11010101

    let a: GarbledInt<16> = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt<16> = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

    let result: i16 = (a | b).into();
    assert_eq!(result, -21846_i16 | -10923_i16); // Expected result of OR between 1010101010101010 and 1101010101010101

    let a: GarbledInt<32> = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
    let b: GarbledInt<32> = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

    let result: i32 = (a | b).into();
    assert_eq!(result, -1431655766_i32 | -715827883_i32);
    // Expected result of OR between 10101010101010101010101010101010 and 11010101010101010101010101010101

    let a: GarbledInt<64> = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt<64> = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i64 = (a | b).into();
    assert_eq!(result, -6148914691236517206_i64 | -3074457345618258603_i64);

    let a: GarbledInt<128> = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt<128> = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i128 = (a | b).into();
    assert_eq!(
        result,
        -6148914691236517206_i128 | -3074457345618258603_i128
    );
}

#[test]
fn test_int_or_assign() {
    let mut a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    a |= b;
    assert_eq!(<GarbledInt8 as Into<i8>>::into(a), -86_i8 | -43_i8); // Expected result of OR between 10101010 and 11010101

    let mut a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt16 = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

    a |= b;
    assert_eq!(
        <GarbledInt16 as Into<i16>>::into(a),
        -21846_i16 | -10923_i16
    ); // Expected result of OR between 1010101010101010 and 1101010101010101

    let mut a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
    let b: GarbledInt32 = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

    a |= b;
    assert_eq!(
        <GarbledInt32 as Into<i32>>::into(a),
        -1431655766_i32 | -715827883_i32
    );
    // Expected result of OR between 10101010101010101010101010101010 and 11010101010101010101010101010101

    let mut a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 110101010101010101010101010101010101010101010101010101

    a |= b;
    assert_eq!(
        <GarbledInt64 as Into<i64>>::into(a),
        -6148914691236517206_i64 | -3074457345618258603_i64
    );

    let mut a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt128 = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    a |= b;
    assert_eq!(
        <GarbledInt128 as Into<i128>>::into(a),
        -6148914691236517206_i128 | -3074457345618258603_i128
    );
}

#[test]
fn test_int_and() {
    let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    let result: i8 = (a & b).into();
    assert_eq!(result, -86_i8 & -43_i8); // Expected result of AND between 10101010 and 11010101

    let a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt16 = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

    let result: i16 = (a & b).into();
    assert_eq!(result, -21846_i16 & -10923_i16); // Expected result of AND between 1010101010101010 and 1101010101010101

    let a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
    let b: GarbledInt32 = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

    let result: i32 = (a & b).into();
    assert_eq!(result, -1431655766_i32 & -715827883_i32);
    // Expected result of AND between 10101010101010101010101010101010 and 11010101010101010101010101010101

    let a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i64 = (a & b).into();
    assert_eq!(result, -6148914691236517206_i64 & -3074457345618258603_i64);
    // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101

    let a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt128 = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i128 = (a & b).into();
    assert_eq!(
        result,
        -6148914691236517206_i128 & -3074457345618258603_i128
    );
    // Expected result of AND between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
}

#[test]
fn test_int_and_assign() {
    let mut a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    a &= b;
    assert_eq!(<GarbledInt8 as Into<i8>>::into(a), -86_i8 & -43_i8); // Expected result of AND between 10101010 and 11010101

    let mut a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt16 = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

    a &= b;
    assert_eq!(
        <GarbledInt16 as Into<i16>>::into(a),
        -21846_i16 & -10923_i16
    ); // Expected result of AND between 1010101010101010 and 1101010101010101

    let mut a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
    let b: GarbledInt32 = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

    a &= b;
    assert_eq!(
        <GarbledInt32 as Into<i32>>::into(a),
        -1431655766_i32 & -715827883_i32
    );
    // Expected result of AND between 10101010101010101010101010101010 and 11010101010101010101010101010101

    let mut a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 110101010101010101010

    a &= b;
    assert_eq!(
        <GarbledInt64 as Into<i64>>::into(a),
        -6148914691236517206_i64 & -3074457345618258603_i64
    );

    let mut a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt128 = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 110101010101010101010

    a &= b;
    assert_eq!(
        <GarbledInt128 as Into<i128>>::into(a),
        -6148914691236517206_i128 & -3074457345618258603_i128
    );
}

#[test]
fn test_uint_not() {
    let a: GarbledUint8 = 170_u8.into(); // Binary 10101010

    let result: u8 = (!a).into();
    assert_eq!(result, !170); // Expected result of NOT on 10101010

    let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010

    let result: u16 = (!a).into();
    assert_eq!(result, !43690); // Expected result of NOT on 1010101010101010

    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010

    let result: u32 = (!a).into();
    assert_eq!(result, !2863311530); // Expected result of NOT on 10101010101010101010101010101010

    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010

    let result: u64 = (!a).into();
    assert_eq!(result, !12297829382473034410); // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010

    let a: GarbledUint128 = 170_u128.into(); // Binary 10101010

    let result: u128 = (!a).into();
    assert_eq!(result, !170); // Expected result of NOT on 10101010
}

#[test]
fn test_int_not() {
    let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010

    let result: i8 = (!a).into();
    assert_eq!(result, !-86_i8); // Expected result of NOT on 10101010

    let a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010

    let result: i16 = (!a).into();
    assert_eq!(result, !-21846_i16); // Expected result of NOT on 1010101010101010

    let a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010

    let result: i32 = (!a).into();
    assert_eq!(result, !-1431655766_i32); // Expected result of NOT on 10101010101010101010101010101010

    let a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010

    let result: i64 = (!a).into();
    assert_eq!(result, !-6148914691236517206_i64); // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010

    let a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010

    let result: i128 = (!a).into();
    assert_eq!(result, !-6148914691236517206_i128); // Expected result of NOT on 1010101010101010101010101010101010101010101010101010101010101010
}

#[test]
fn test_uint_left_shift() {
    let a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
    let result: u8 = (a << 1).into(); // Perform left shift by 1
    assert_eq!(result, 0b0010_u8);

    let a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
    let result: u8 = (a << 2).into(); // Perform left shift by 2
    assert_eq!(result, 0b0100_u8);

    let a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
    let result: u8 = (a << 3).into(); // Perform left shift by 3
    assert_eq!(result, 0b1000);

    let a = GarbledUint::<4>::new(vec![true, false, false, false]); // Binary 0001
    let result: u8 = (a << 2).into(); // Perform left shift by 2
    assert_eq!(result, 0b0100); // Binary 0100 (Left shift result of 0001)

    let a = GarbledUint::<4>::new(vec![true, false, false, false]); // Binary 0001

    let result: u8 = (a << 3).into(); // Perform left shift by 3
    assert_eq!(result, 0b1000); // Binary 1000 (Left shift result of 0001)
}

#[test]
fn test_uint_left_shift_and_assign() {
    let mut a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
    a <<= 1; // Perform left shift by 1
    assert_eq!(<GarbledUint8 as Into<u8>>::into(a), 0b0010_u8);

    let mut a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
    a <<= 2; // Perform left shift by 2
    assert_eq!(<GarbledUint8 as Into<u8>>::into(a), 0b0100_u8);

    let mut a: GarbledUint8 = 0b0001_u8.into(); // Binary 0001
    a <<= 3; // Perform left shift by 3
    assert_eq!(<GarbledUint8 as Into<u8>>::into(a), 0b1000_u8);

    let mut a = GarbledUint::<4>::new(vec![true, false, false, false]); // Binary 0001
    a <<= 2; // Perform left shift by 2
    assert_eq!(<GarbledUint<4> as Into<u8>>::into(a), 0b0100_u8); // Binary 0100 (Left shift result of 0001)

    let mut a = GarbledUint::<4>::new(vec![true, false, false, false]); // Binary 0001
    a <<= 3; // Perform left shift by 3
    assert_eq!(<GarbledUint<4> as Into<u8>>::into(a), 0b1000_u8); // Binary 1000 (Left shift result of 0001)
}

#[test]
fn test_int_left_shift() {
    let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

    let result: i8 = (a << 1).into(); // Perform left shift by 1
    assert_eq!(result, 0b10000_i8); // Binary 0000 (Left shift result of 1000)

    let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

    let result: i8 = (a << 2).into(); // Perform left shift by 2
    assert_eq!(result, 0b100000_i8); // Binary 0000 (Left shift result of 1000)

    let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

    let result: i8 = (a << 3).into(); // Perform left shift by 3
    assert_eq!(result, 0b1000000_i8); // Binary 0000 (Left shift result of 1000)

    let a: GarbledInt8 = 1_i8.into(); // Binary 1000

    let result: i8 = (a << 1).into(); // Perform left shift by 1
    assert_eq!(result, 0b0010_i8); // Binary 0010 (Left shift result of 0001)

    let a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001

    let result: i8 = (a << 2).into(); // Perform left shift by 2
    assert_eq!(result, 0b0100_i8); // Binary 0100 (Left shift result of 0001)

    let a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001

    let result: i8 = (a << 3).into(); // Perform left shift by 3
    assert_eq!(result, 0b1000_i8); // Binary 1000 (Left shift result of 0001)
}

#[test]
fn test_int_left_shift_and_assign() {
    let mut a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000
    a <<= 1; // Perform left shift by 1
    assert_eq!(<GarbledInt8 as Into<i8>>::into(a), 0b10000_i8); // Binary 0000 (Left shift result of 1000)

    let mut a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000
    a <<= 2; // Perform left shift by 2
    assert_eq!(<GarbledInt8 as Into<i8>>::into(a), 0b100000_i8); // Binary 0000 (Left shift result of 1000)

    let mut a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000
    a <<= 3; // Perform left shift by 3
    assert_eq!(<GarbledInt8 as Into<i8>>::into(a), 0b1000000_i8); // Binary 0000 (Left shift result of 1000)

    let mut a: GarbledInt8 = 1_i8.into(); // Binary 1000
    a <<= 1; // Perform left shift by 1
    assert_eq!(<GarbledInt8 as Into<i8>>::into(a), 0b0010_i8); // Binary 0010 (Left shift result of 0001)

    let mut a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001
    a <<= 2; // Perform left shift by 2
    assert_eq!(<GarbledInt<4> as Into<i8>>::into(a), 0b0100_i8); // Binary 0100 (Left shift result of 0001)

    let mut a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001
    a <<= 3; // Perform left shift by 3
    assert_eq!(<GarbledInt<4> as Into<i8>>::into(a), 0b1000_i8); // Binary 1000 (Left shift result of 0001)
}

#[test]
fn test_right_shift_uint() {
    let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

    let result: u8 = (a >> 1).into(); // Perform right shift by 1
    assert_eq!(result, 0b0100); // Binary 0100 (Right shift result of 1000)

    let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

    let result: u8 = (a >> 2).into(); // Perform right shift by 2
    assert_eq!(result, 0b0010); // Binary 0010 (Right shift result of 1000)

    let a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000

    let result: u8 = (a >> 3).into(); // Perform right shift by 3
    assert_eq!(result, 0b0001); // Binary 0001 (Right shift result of 1000)
}

#[test]
fn test_uint_right_shift_and_assign() {
    let mut a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000
    a >>= 1; // Perform right shift by 1
    assert_eq!(<GarbledUint<4> as Into<u8>>::into(a), 0b0100); // Binary 0100 (Right shift result of 1000)

    let mut a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000
    a >>= 2; // Perform right shift by 2
    assert_eq!(<GarbledUint<4> as Into<u8>>::into(a), 0b0010); // Binary 0010 (Right shift result of 1000)

    let mut a = GarbledUint::<4>::new(vec![false, false, false, true]); // Binary 1000
    a >>= 3; // Perform right shift by 3
    assert_eq!(<GarbledUint<4> as Into<u8>>::into(a), 0b0001); // Binary 0001 (Right shift result of 1000)
}

#[test]
fn test_uint_nand() {
    let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    let result: u8 = a.nand(b).into();
    assert_eq!(result, !(170 & 85)); // Expected result of NAND between 10101010 and 01010101

    let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    let result: u16 = a.nand(b).into();
    assert_eq!(result, !(43690 & 21845)); // Expected result of NAND between 1010101010101010 and 0101010101010101

    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    let result: u32 = a.nand(b).into();
    assert_eq!(result, !(2863311530 & 1431655765)); // Expected result of NAND between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    let result: u64 = a.nand(b).into();
    assert_eq!(result, !(12297829382473034410 & 6148914691236517205));
    // Expected result of NAND between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

    let result: u128 = a.nand(b).into();
    assert_eq!(result, !(170 & 85)); // Expected result of NAND between 10101010 and 01010101
}

#[test]
fn test_int_nand() {
    let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    let result: i8 = a.nand(b).into();
    assert_eq!(result, !(-86_i8 & -43_i8)); // Expected result of NAND between 10101010 and 11010101

    let a: GarbledInt<16> = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt<16> = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

    let result: i16 = a.nand(b).into();
    assert_eq!(result, !(-21846_i16 & -10923_i16)); // Expected result of NAND between 1010101010101010 and 1101010101010101

    let a: GarbledInt<32> = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
    let b: GarbledInt<32> = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

    let result: i32 = a.nand(b).into();
    assert_eq!(result, !(-1431655766_i32 & -715827883_i32));
    // Expected result of NAND between 10101010101010101010101010101010 and 11010101010101010101010101010101

    let a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i64 = a.nand(b).into();
    assert_eq!(
        result,
        !(-6148914691236517206_i64 & -3074457345618258603_i64)
    );

    let a: GarbledInt<128> = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt<128> = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i128 = a.nand(b).into();
    assert_eq!(
        result,
        !(-6148914691236517206_i128 & -3074457345618258603_i128)
    );
}

#[test]
fn test_from_u8_nor() {
    let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    let result: u8 = a.nor(b).into();
    assert_eq!(result, !(170 | 85)); // Expected result of NOR between 10101010 and 01010101
}

#[test]
fn test_from_u16_nor() {
    let a: GarbledUint16 = 43707_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    let result: u16 = a.nor(b).into();
    assert_eq!(result, !(43707 | 21845)); // Expected result of NOR between 1010101010101011 and 0101010101010101
}

#[test]
fn test_from_u32_nor() {
    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    let result: u32 = a.nor(b).into();
    assert_eq!(result, !(2863311530 | 1431655765)); // Expected result of NOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
}

#[test]
fn test_from_u64_nor() {
    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    let result: u64 = a.nor(b).into();
    assert_eq!(result, !(12297829382473034410 | 6148914691236517205));
    // Expected result of NOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
}

#[test]
fn test_from_u128_nor() {
    let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

    let result: u128 = a.nor(b).into();
    assert_eq!(result, !(170 | 85)); // Expected result of NOR between 10101010 and 01010101
}

#[test]
fn test_from_i8_nor() {
    let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    let result: i8 = a.nor(b).into();
    assert_eq!(result, !(-86_i8 | -43_i8)); // Expected result of NOR between 10101010 and 11010101
}

#[test]
fn test_from_i16_nor() {
    let a: GarbledInt<16> = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt<16> = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

    let result: i16 = a.nor(b).into();
    assert_eq!(result, !(-21846_i16 | -10923_i16)); // Expected result of NOR between 1010101010101010 and 1101010101010101
}

#[test]
fn test_from_i32_nor() {
    let a: GarbledInt<32> = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
    let b: GarbledInt<32> = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

    let result: i32 = a.nor(b).into();
    assert_eq!(result, !(-1431655766_i32 | -715827883_i32));
    // Expected result of NOR between 10101010101010101010101010101010 and 11010101010101010101010101010101
}

#[test]
fn test_from_i64_nor() {
    let a: GarbledInt<64> = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt<64> = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i64 = a.nor(b).into();
    assert_eq!(
        result,
        !(-6148914691236517206_i64 | -3074457345618258603_i64)
    );
    // Expected result of NOR between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
}

#[test]
fn test_from_i128_nor() {
    let a: GarbledInt<128> = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt<128> = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i128 = a.nor(b).into();
    assert_eq!(
        result,
        !(-6148914691236517206_i128 | -3074457345618258603_i128)
    );
    // Expected result of NOR between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
}

#[test]
fn test_from_u8_xnor() {
    let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    let result: u8 = a.xnor(b).into();
    assert_eq!(result, !(170 ^ 85)); // Expected result of XNOR between 10101010 and 01010101
}

#[test]
fn test_from_u16_xnor() {
    let a: GarbledUint16 = 43690_u16.into(); // Binary 1010101010101010
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    let result: u16 = a.xnor(b).into();
    assert_eq!(result, !(43690 ^ 21845)); // Expected result of XNOR between 1010101010101010 and 0101010101010101
}

#[test]
fn test_from_u32_xnor() {
    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    let result: u32 = a.xnor(b).into();
    assert_eq!(result, !(2863311530 ^ 1431655765)); // Expected result of XNOR between 10101010101010101010101010101010 and 01010101010101010101010101010101
}

#[test]
fn test_from_u64_xnor() {
    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    let result: u64 = a.xnor(b).into();
    assert_eq!(result, !(12297829382473034410 ^ 6148914691236517205));
    // Expected result of XNOR between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
}

#[test]
fn test_from_u128_xnor() {
    let a: GarbledUint128 = 170_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 85_u128.into(); // Binary 01010101

    let result: u128 = a.xnor(b).into();
    assert_eq!(result, !(170 ^ 85)); // Expected result of XNOR between 10101010 and 01010101
}

#[test]
fn test_from_i8_xnor() {
    let a: GarbledInt8 = (-86).into(); // Two's complement binary for -86 is 10101010
    let b: GarbledInt8 = (-43).into(); // Two's complement binary for -43 is 11010101

    let result: i8 = a.xnor(b).into();
    assert_eq!(result, !(-86_i8 ^ -43_i8)); // Expected result of XNOR between 10101010 and 11010101
}

#[test]
fn test_from_i16_xnor() {
    let a: GarbledInt16 = (-21846).into(); // Two's complement binary for -21846 is 1010101010101010
    let b: GarbledInt16 = (-10923).into(); // Two's complement binary for -10923 is 1101010101010101

    let result: i16 = a.xnor(b).into();
    assert_eq!(result, !(-21846_i16 ^ -10923_i16)); // Expected result of XNOR between 1010101010101010 and 1101010101010101
}

#[test]
fn test_from_i32_xnor() {
    let a: GarbledInt32 = (-1431655766).into(); // Two's complement binary for -1431655766 is 10101010101010101010101010101010
    let b: GarbledInt32 = (-715827883).into(); // Two's complement binary for -715827883 is 11010101010101010101010101010101

    let result: i32 = a.xnor(b).into();
    assert_eq!(result, !(-1431655766_i32 ^ -715827883_i32));
    // Expected result of XNOR between 10101010101010101010101010101010 and 11010101010101010101010101010101
}

#[test]
fn test_from_i64_xnor() {
    let a: GarbledInt64 = (-6148914691236517206_i64).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt64 = (-3074457345618258603_i64).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i64 = a.xnor(b).into();
    assert_eq!(
        result,
        !(-6148914691236517206_i64 ^ -3074457345618258603_i64)
    );
    // Expected result of XNOR between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
}

#[test]
fn test_from_i128_xnor() {
    let a: GarbledInt128 = (-6148914691236517206_i128).into(); // Two's complement binary for -6148914691236517206 is 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledInt128 = (-3074457345618258603_i128).into(); // Two's complement binary for -3074457345618258603 is 1101010101010101010101010101010101010101010101010101010101010101

    let result: i128 = a.xnor(b).into();
    assert_eq!(
        result,
        !(-6148914691236517206_i128 ^ -3074457345618258603_i128)
    );
    // Expected result of XNOR between 1010101010101010101010101010101010101010101010101010101010101010 and 1101010101010101010101010101010101010101010101010101010101010101
}

#[test]
fn test_right_shift_int() {
    let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

    let result: i8 = (a >> 1).into(); // Perform right shift by 1
    assert_eq!(result, 0b0100_i8); // Binary 0100 (Right shift result of 1000)

    let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

    let result: i8 = (a >> 2).into(); // Perform right shift by 2
    assert_eq!(result, 0b0010_i8); // Binary 0010 (Right shift result of 1000)

    let a: GarbledInt8 = 0b1000_i8.into(); // Binary 1000

    let result: i8 = (a >> 3).into(); // Perform right shift by 3
    assert_eq!(result, 0b0001_i8); // Binary 0001 (Right shift result of 1000)

    let a: GarbledInt8 = 1_i8.into(); // Binary 0001

    let result: i8 = (a >> 1).into(); // Perform right shift by 1
    assert_eq!(result, 0b0000_i8); // Binary 0000 (Right shift result of 0001)

    let a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001

    let result: i8 = (a >> 2).into(); // Perform right shift by 2
    assert_eq!(result, 0b0000_i8); // Binary 0000 (Right shift result of 0001)

    let a = GarbledInt::<4>::new(vec![true, false, false, false]); // Binary 0001

    let result: i8 = (a >> 3).into(); // Perform right shift by 3
    assert_eq!(result, 0b0000_i8); // Binary 0000 (Right shift result of 0001)
}
