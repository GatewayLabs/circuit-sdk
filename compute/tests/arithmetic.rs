use compute::prelude::*;

#[test]
fn test_uint16_add() {
    let a: GarbledUint16 = 11_u16.into();
    let b: GarbledUint16 = 2_u16.into();
    let c: GarbledUint16 = 3_u16.into();

    let result: u16 = (a + b - c).into(); // Perform addition on the 4-bit values
    assert_eq!(result, 11 + 2 - 3); // Expected result of addition between 1010101010101011, 0101010101010101 and 42
}

#[test]
fn test_uint_add() {
    let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    let result: u8 = (a + b).into(); // Perform addition on the 4-bit values
    assert_eq!(result, 170_u8 + 85_u8); // Expected result of addition between 10101010 and 01010101

    let a: GarbledUint16 = 4370_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 2184_u16.into(); // Binary 0101010101010101

    let result: u16 = (a + b).into(); // Perform addition on the 4-bit values
    assert_eq!(result, 4370_u16 + 2184_u16); // Expected result of addition between 1010101010101011 and 0101010101010101

    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    let result: u32 = (a + b).into(); // Perform addition on the 4-bit values
    assert_eq!(result, 2863311530_u32 + 1431655765_u32); // Expected result of addition between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    let result: u64 = (a + b).into(); // Perform addition on the 4-bit values
    assert_eq!(result, 12297829382473034410_u64 + 6148914691236517205_u64);
    // Expected result of addition between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let a: GarbledUint128 = 12297829382473034410_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 6148914691236517205_u128.into(); // Binary 01010101

    let result: u128 = (a + b).into(); // Perform addition on the 4-bit values
    assert_eq!(result, 12297829382473034410_u128 + 6148914691236517205_u128);
}

#[test]
fn test_uint_add_assign() {
    let mut a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

    a += b; // Perform addition on the 4-bit values
    assert_eq!(<GarbledUint<8> as Into<u8>>::into(a), 170_u8 + 85_u8); // Expected result of addition between 10101010 and 01010101

    let mut a: GarbledUint16 = 4370_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 2184_u16.into(); // Binary 0101010101010101

    a += b; // Perform addition on the 4-bit values
    assert_eq!(<GarbledUint<16> as Into<u16>>::into(a), 4370_u16 + 2184_u16); // Expected result of addition between 1010101010101011 and 0101010101010101

    let mut a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    a += b; // Perform addition on the 4-bit values
    assert_eq!(
        <GarbledUint<32> as Into<u32>>::into(a),
        2863311530_u32 + 1431655765_u32
    ); // Expected result of addition between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let mut a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    a += b; // Perform addition on the 4-bit values
    assert_eq!(
        <GarbledUint<64> as Into<u64>>::into(a),
        12297829382473034410_u64 + 6148914691236517205_u64
    );
    // Expected result of addition between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let mut a: GarbledUint128 = 12297829382473034410_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 6148914691236517205_u128.into(); // Binary 01010101

    a += b; // Perform addition on the 4-bit values
    assert_eq!(
        <GarbledUint<128> as Into<u128>>::into(a),
        12297829382473034410_u128 + 6148914691236517205_u128
    );
}

#[test]
fn test_int_add_assign() {
    let mut a: GarbledInt8 = 3_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    a += b; // Perform addition on the 8-bit values
    assert_eq!(<GarbledInt<8> as Into<i8>>::into(a), 3_i8 + 2_i8); // Expected result of addition between 3 and 2

    let mut a: GarbledInt16 = 1340_i16.into();
    let b: GarbledInt16 = 8543_i16.into();

    a += b; // Perform addition on the 16-bit values
    assert_eq!(<GarbledInt<16> as Into<i16>>::into(a), 1340_i16 + 8543_i16);

    let mut a: GarbledInt32 = 17034322_i32.into();
    let b: GarbledInt32 = 84928323_i32.into();

    a += b; // Perform addition on the 32-bit values
    assert_eq!(
        <GarbledInt<32> as Into<i32>>::into(a),
        17034322_i32 + 84928323_i32
    );

    let mut a: GarbledInt64 = 170343221234_i64.into();
    let b: GarbledInt64 = 849283231234_i64.into();

    a += b; // Perform addition on the 64-bit values
    assert_eq!(
        <GarbledInt<64> as Into<i64>>::into(a),
        170343221234_i64 + 849283231234_i64
    );

    let mut a: GarbledInt128 = 170343221234567890_i128.into();
    let b: GarbledInt128 = 849283231234567890_i128.into();

    a += b; // Perform addition on the 128-bit values
    assert_eq!(
        <GarbledInt<128> as Into<i128>>::into(a),
        170343221234567890_i128 + 849283231234567890_i128
    );
}

#[test]
fn test_int_add() {
    let a: GarbledInt8 = 3_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    let result: i8 = (a + b).into(); // Perform addition on the 8-bit values
    assert_eq!(result, 3_i8 + 2_i8); // Expected result of addition between 3 and 2

    // use larger values to test the 16-bit addition
    let a: GarbledInt16 = 1340_i16.into();
    let b: GarbledInt16 = 8543_i16.into();

    let result: i16 = (a + b).into(); // Perform addition on the 16-bit values
    assert_eq!(result, 1340_i16 + 8543_i16);

    // use larger values to test the 32-bit addition
    let a: GarbledInt32 = 17034322_i32.into();
    let b: GarbledInt32 = 84928323_i32.into();

    let result: i32 = (a + b).into(); // Perform addition on the 32-bit values
    assert_eq!(result, 17034322_i32 + 84928323_i32);

    // use larger values to test the 64-bit addition
    let a: GarbledInt64 = 170343221234_i64.into();
    let b: GarbledInt64 = 849283231234_i64.into();

    let result: i64 = (a + b).into(); // Perform addition on the 64-bit values
    assert_eq!(result, 170343221234_i64 + 849283231234_i64);

    // use larger values to test the 128-bit addition
    let a: GarbledInt128 = 170343221234567890_i128.into();
    let b: GarbledInt128 = 849283231234567890_i128.into();

    let result: i128 = (a + b).into(); // Perform addition on the 128-bit values
    assert_eq!(result, 170343221234567890_i128 + 849283231234567890_i128);
}

#[test]
fn test_int_subtract() {
    let a: GarbledInt8 = 3_i8.into();
    let b: GarbledInt8 = (-2_i8).into();

    let result: i8 = (a + b).into(); // Perform addition on the 8-bit values
    assert_eq!(result, 3_i8 - 2_i8); // Expected result of addition between 3 and -2

    // use larger values to test the 16-bit addition
    let a: GarbledInt16 = 1340_i16.into();
    let b: GarbledInt16 = 8543_i16.into();

    let result: i16 = (a + b).into(); // Perform addition on the 16-bit values
    assert_eq!(result, 1340_i16 + 8543_i16);

    // use larger values to test the 32-bit addition
    let a: GarbledInt32 = 17034322_i32.into();
    let b: GarbledInt32 = 84928323_i32.into();

    let result: i32 = (a + b).into(); // Perform addition on the 32-bit values
    assert_eq!(result, 17034322_i32 + 84928323_i32);

    // use larger values to test the 64-bit addition
    let a: GarbledInt64 = 170343221234_i64.into();
    let b: GarbledInt64 = 849283231234_i64.into();

    let result: i64 = (a + b).into(); // Perform addition on the 64-bit values
    assert_eq!(result, 170343221234_i64 + 849283231234_i64);

    // use larger values to test the 128-bit addition
    let a: GarbledInt128 = 170343221234567890_i128.into();
    let b: GarbledInt128 = 849283231234567890_i128.into();

    let result: i128 = (a + b).into(); // Perform addition on the 128-bit values
    assert_eq!(result, 170343221234567890_i128 + 849283231234567890_i128);
}

#[test]
fn test_uint_subtract() {
    let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 100_u8.into(); // Binary 01100100

    let result: u8 = (a - b).into();
    assert_eq!(result, 170_u8 - 100_u8); // Expected result of subtraction between 10101010 and 01010101

    let a: GarbledUint16 = 43707_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    let result: u16 = (a - b).into();
    assert_eq!(result, 43707_u16 - 21845_u16); // Expected result of subtraction between 1010101010101011 and 0101010101010101

    let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    let result: u32 = (a - b).into();
    assert_eq!(result, 2863311530_u32 - 1431655765_u32); // Expected result of subtraction between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    let result: u64 = (a - b).into();
    assert_eq!(result, 12297829382473034410_u64 - 6148914691236517205_u64);
    // Expected result of subtraction between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101

    let a: GarbledUint128 = 12297829382473034410_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 6148914691236517205_u128.into(); // Binary 01010101

    let result: u128 = (a - b).into();
    assert_eq!(result, 12297829382473034410_u128 - 6148914691236517205_u128);
}

#[test]
fn test_uint_sub_assign() {
    let mut a: GarbledUint8 = 170_u8.into(); // Binary 10101010
    let b: GarbledUint8 = 100_u8.into(); // Binary 01100100

    a -= b; // Perform subtraction on the 4-bit values
    assert_eq!(<GarbledUint<8> as Into<u8>>::into(a), 170_u8 - 100_u8); // Expected result of subtraction between 10101010 and 01010101

    let mut a: GarbledUint16 = 43707_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

    a -= b; // Perform subtraction on the 4-bit values
    assert_eq!(
        <GarbledUint<16> as Into<u16>>::into(a),
        43707_u16 - 21845_u16
    ); // Expected result of subtraction between 1010101010101011 and 0101010101010101

    let mut a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
    let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

    a -= b; // Perform subtraction on the 4-bit values
    assert_eq!(
        <GarbledUint<32> as Into<u32>>::into(a),
        2863311530_u32 - 1431655765_u32
    ); // Expected result of subtraction between 10101010101010101010101010101010 and 01010101010101010101010101010101

    let mut a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
    let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

    a -= b; // Perform subtraction on the 4-bit values
    assert_eq!(
        <GarbledUint<64> as Into<u64>>::into(a),
        12297829382473034410_u64 - 6148914691236517205_u64
    );

    let mut a: GarbledUint128 = 12297829382473034410_u128.into(); // Binary 10101010
    let b: GarbledUint128 = 6148914691236517205_u128.into(); // Binary 01010101

    a -= b; // Perform subtraction on the 4-bit values
    assert_eq!(
        <GarbledUint<128> as Into<u128>>::into(a),
        12297829382473034410_u128 - 6148914691236517205_u128
    );
}

#[test]
fn test_int_sub_assign() {
    let mut a: GarbledInt8 = 3_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    a -= b; // Perform subtraction on the 8-bit values
    assert_eq!(<GarbledInt<8> as Into<i8>>::into(a), 3_i8 - 2_i8); // Expected result of subtraction between 3 and 2

    let mut a: GarbledInt16 = 1340_i16.into();
    let b: GarbledInt16 = 8543_i16.into();

    a -= b; // Perform subtraction on the 16-bit values
    assert_eq!(<GarbledInt<16> as Into<i16>>::into(a), 1340_i16 - 8543_i16);

    let mut a: GarbledInt32 = 17034322_i32.into();
    let b: GarbledInt32 = 84928323_i32.into();

    a -= b; // Perform subtraction on the 32-bit values
    assert_eq!(
        <GarbledInt<32> as Into<i32>>::into(a),
        17034322_i32 - 84928323_i32
    );

    let mut a: GarbledInt64 = 170343221234_i64.into();
    let b: GarbledInt64 = 849283231234_i64.into();

    a -= b; // Perform subtraction on the 64-bit values
    assert_eq!(
        <GarbledInt<64> as Into<i64>>::into(a),
        170343221234_i64 - 849283231234_i64
    );

    let mut a: GarbledInt128 = 170343221234567890_i128.into();
    let b: GarbledInt128 = 849283231234567890_i128.into();

    a -= b; // Perform subtraction on the 128-bit values
    assert_eq!(
        <GarbledInt<128> as Into<i128>>::into(a),
        170343221234567890_i128 - 849283231234567890_i128
    );
}

#[test]
fn test_uint_mul() {
    let a: GarbledUint8 = 3_u8.into(); // Binary 0011
    let b: GarbledUint8 = 2_u8.into(); // Binary 0010

    let result: u8 = (a * b).into();
    assert_eq!(result, 6); // 0011 * 0010 = 0110

    let a: GarbledUint8 = 7_u8.into(); // Binary 0000 0111
    let b: GarbledUint8 = 5_u8.into(); // Binary 0000 0101

    let result: u8 = (a * b).into();
    assert_eq!(result, 35); // Binary 0010 0011

    let a: GarbledUint16 = 300_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 7_u16.into(); // Binary 0101010101010101

    let result: u16 = (a * b).into();
    assert_eq!(result, 300_u16 * 7_u16); // Expected result of multiplication between 1010101010101011 and 0101010101010101
}

#[test]
fn test_int_mul() {
    let a: GarbledInt8 = 3_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    let result: i8 = (a * b).into();
    assert_eq!(result, 3_i8 * 2_i8);

    let a: GarbledInt16 = 134_i16.into();
    let b: GarbledInt16 = 85_i16.into();

    let result: i16 = (a * b).into();
    assert_eq!(result, 134_i16 * 85_i16);
}

#[test]
fn test_uint_mul_assign() {
    let mut a: GarbledUint8 = 3_u8.into(); // Binary 0011
    let b: GarbledUint8 = 2_u8.into(); // Binary 0010

    a *= b;
    assert_eq!(<GarbledUint<8> as Into<u8>>::into(a), 6); // 0011 * 0010 = 0110

    let mut a: GarbledUint8 = 7_u8.into(); // Binary 0000 0111
    let b: GarbledUint8 = 5_u8.into(); // Binary 0000 0101

    a *= b;
    assert_eq!(<GarbledUint<8> as Into<u8>>::into(a), 35); // Binary 0010 0011

    let mut a: GarbledUint16 = 300_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 7_u16.into(); // Binary 0101010101010101

    a *= b;
    assert_eq!(<GarbledUint<16> as Into<u16>>::into(a), 300_u16 * 7_u16); // Expected result of multiplication between 1010101010101011 and 0101010101010101
}

#[test]
fn test_int_mul_assign() {
    let mut a: GarbledInt8 = 3_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    a *= b;
    assert_eq!(<GarbledInt<8> as Into<i8>>::into(a), 3_i8 * 2_i8);

    let mut a: GarbledInt16 = 134_i16.into();
    let b: GarbledInt16 = 85_i16.into();

    a *= b;
    assert_eq!(<GarbledInt<16> as Into<i16>>::into(a), 134_i16 * 85_i16);
}

#[test]
fn test_multiple_additions() {
    let a: GarbledUint32 = 170_u32.into();
    let b: GarbledUint32 = 85_u32.into();
    let c: GarbledUint32 = 42_u32.into();
    let d: GarbledUint32 = 21_u32.into();
    let e: GarbledUint32 = 10_u32.into();

    let result: u32 = (a + b + c + d + e).into();
    assert_eq!(result, 170_u32 + 85_u32 + 42_u32 + 21_u32 + 10_u32);
}

// div

#[test]
fn test_uint_div() {
    let a: GarbledUint8 = 6_u8.into(); // Binary 0110
    let b: GarbledUint8 = 2_u8.into(); // Binary 0010

    let result: u8 = (a / b).into();
    assert_eq!(result, 6 / 2); // 0110 / 0010 = 0011

    let a: GarbledUint16 = 300_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 7_u16.into(); // Binary 0101010101010101

    let result: u16 = (a / b).into();
    assert_eq!(result, 300_u16 / 7_u16); // Expected result of division between 1010101010101011 and 0101010101010101
}

#[test]
fn test_int_div() {
    let a: GarbledInt8 = 6_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    let result: i8 = (a / b).into();
    assert_eq!(result, 6_i8 / 2_i8);

    let a: GarbledInt16 = 134_i16.into();
    let b: GarbledInt16 = 85_i16.into();

    let result: i16 = (a / b).into();
    assert_eq!(result, 134_i16 / 85_i16);
}

#[test]
fn test_uint_div_assign() {
    let mut a: GarbledUint8 = 6_u8.into(); // Binary 0110
    let b: GarbledUint8 = 2_u8.into(); // Binary 0010

    a /= b;
    assert_eq!(<GarbledUint<8> as Into<u8>>::into(a), 6 / 2); // 0110 / 0010 = 0011

    let mut a: GarbledUint16 = 300_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 7_u16.into(); // Binary 0101010101010101

    a /= b;
    assert_eq!(<GarbledUint<16> as Into<u16>>::into(a), 300_u16 / 7_u16); // Expected result of division between 1010101010101011 and 0101010101010101
}

#[test]
fn test_int_div_assign() {
    let mut a: GarbledInt8 = 6_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    a /= b;
    assert_eq!(<GarbledInt<8> as Into<i8>>::into(a), 6_i8 / 2_i8);

    let mut a: GarbledInt16 = 134_i16.into();
    let b: GarbledInt16 = 85_i16.into();

    a /= b;
    assert_eq!(<GarbledInt<16> as Into<i16>>::into(a), 134_i16 / 85_i16);
}

// rem

#[test]
fn test_uint_rem() {
    let a: GarbledUint8 = 6_u8.into(); // Binary 0110
    let b: GarbledUint8 = 2_u8.into(); // Binary 0010

    let result: u8 = (a % b).into();
    assert_eq!(result, 6 % 2); // 0110 % 0010 = 0000

    let a: GarbledUint16 = 300_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 7_u16.into(); // Binary 0101010101010101

    let result: u16 = (a % b).into();
    assert_eq!(result, 300_u16 % 7_u16); // Expected result of remainder between 1010101010101011 and 0101010101010101
}

#[test]
fn test_int_rem() {
    let a: GarbledInt8 = 6_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    let result: i8 = (a % b).into();
    assert_eq!(result, 6_i8 % 2_i8);

    let a: GarbledInt16 = 134_i16.into();
    let b: GarbledInt16 = 85_i16.into();

    let result: i16 = (a % b).into();
    assert_eq!(result, 134_i16 % 85_i16);
}

#[test]
fn test_uint_rem_assign() {
    let mut a: GarbledUint8 = 6_u8.into(); // Binary 0110
    let b: GarbledUint8 = 2_u8.into(); // Binary 0010

    a %= b;
    assert_eq!(<GarbledUint<8> as Into<u8>>::into(a), 6 % 2); // 0110 % 0010 = 0000

    let mut a: GarbledUint16 = 300_u16.into(); // Binary 1010101010101011
    let b: GarbledUint16 = 7_u16.into(); // Binary 0101010101010101

    a %= b;
    assert_eq!(<GarbledUint<16> as Into<u16>>::into(a), 300_u16 % 7_u16); // Expected result of remainder between 1010101010101011 and 0101010101010101
}

#[test]
fn test_int_rem_assign() {
    let mut a: GarbledInt8 = 6_i8.into();
    let b: GarbledInt8 = 2_i8.into();

    a %= b;
    assert_eq!(<GarbledInt<8> as Into<i8>>::into(a), 6_i8 % 2_i8);

    let mut a: GarbledInt16 = 134_i16.into();
    let b: GarbledInt16 = 85_i16.into();

    a %= b;
    assert_eq!(<GarbledInt<16> as Into<i16>>::into(a), 134_i16 % 85_i16);
}
