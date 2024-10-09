use crate::int::GarbledInt;
use crate::operations::helpers::{
    add_gate_fn, build_and_simulate_arithmetic, build_and_simulate_multiplication, sub_gate_fn,
};
use crate::uint::GarbledUint;
use std::ops::{Add, Mul, Sub};

// Implement the Add operation for Uint<N> and &GarbledUint<N>
impl<const N: usize> Add for GarbledUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(&self, &rhs, add_gate_fn)
    }
}

impl<const N: usize> Add for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(self, rhs, add_gate_fn)
    }
}

// Implement the Sub operation for Uint<N> and &GarbledUint<N>
impl<const N: usize> Sub for GarbledUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(&self, &rhs, sub_gate_fn)
    }
}

impl<const N: usize> Sub for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(self, rhs, sub_gate_fn)
    }
}

impl<const N: usize> Mul for GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        build_and_simulate_multiplication(&self, &rhs)
    }
}

impl<const N: usize> Mul for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        build_and_simulate_multiplication(self, rhs)
    }
}

// Implement the Add operation for GarbledInt<N> and &GarbledInt<N>
impl<const N: usize> Add for GarbledInt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(&self.into(), &rhs.into(), add_gate_fn).into()
    }
}

impl<const N: usize> Add for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(&self.into(), &rhs.into(), add_gate_fn).into()
    }
}

// Implement the Sub operation for GarbledInt<N> and &GarbledInt<N>
impl<const N: usize> Sub for GarbledInt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(&self.into(), &rhs.into(), sub_gate_fn).into()
    }
}

impl<const N: usize> Sub for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_simulate_arithmetic(&self.into(), &rhs.into(), sub_gate_fn).into()
    }
}

// tests
#[cfg(test)]
mod tests {
    use crate::int::{
        GarbledInt128, GarbledInt16, GarbledInt32, GarbledInt4, GarbledInt64, GarbledInt8,
    };
    use crate::uint::{GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8};

    #[test]
    fn test_from_u8_add() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
        let b: GarbledUint8 = 85_u8.into(); // Binary 01010101

        let result: u8 = (a + b).into(); // Perform addition on the 4-bit values
        assert_eq!(result, 170_u8 + 85_u8); // Expected result of addition between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_add() {
        let a: GarbledUint16 = 4370_u16.into(); // Binary 1010101010101011
        let b: GarbledUint16 = 2184_u16.into(); // Binary 0101010101010101

        let result: u16 = (a + b).into(); // Perform addition on the 4-bit values
        assert_eq!(result, 4370_u16 + 2184_u16); // Expected result of addition between 1010101010101011 and 0101010101010101
    }

    #[test]
    fn test_from_u32_add() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
        let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

        let result: u32 = (a + b).into(); // Perform addition on the 4-bit values
        assert_eq!(result, 2863311530_u32 + 1431655765_u32); // Expected result of addition between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_add() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result: u64 = (a + b).into(); // Perform addition on the 4-bit values
        assert_eq!(result, 12297829382473034410_u64 + 6148914691236517205_u64);
        // Expected result of addition between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_add() {
        let a: GarbledUint128 = 12297829382473034410_u128.into(); // Binary 10101010
        let b: GarbledUint128 = 6148914691236517205_u128.into(); // Binary 01010101

        let result: u128 = (a + b).into(); // Perform addition on the 4-bit values
        assert_eq!(result, 12297829382473034410_u128 + 6148914691236517205_u128);
    }

    #[test]
    fn test_from_i8_add() {
        let a: GarbledInt8 = 3_i8.into();
        let b: GarbledInt8 = (-2_i8).into();

        let result: i8 = (a + b).into(); // Perform addition on the 8-bit values
        assert_eq!(result, 3_i8 - 2_i8); // Expected result of addition between 3 and -2
    }

    #[test]
    fn test_from_i16_add() {
        // use larger values to test the 16-bit addition
        let a: GarbledInt16 = 1340_i16.into();
        let b: GarbledInt16 = 8543_i16.into();

        let result: i16 = (a + b).into(); // Perform addition on the 16-bit values
        assert_eq!(result, 1340_i16 + 8543_i16);
    }

    #[test]
    fn test_from_i32_add() {
        // use larger values to test the 32-bit addition
        let a: GarbledInt32 = 17034322_i32.into();
        let b: GarbledInt32 = 84928323_i32.into();

        let result: i32 = (a + b).into(); // Perform addition on the 32-bit values
        assert_eq!(result, 17034322_i32 + 84928323_i32);
    }

    #[test]
    fn test_from_i64_add() {
        // use larger values to test the 64-bit addition
        let a: GarbledInt64 = 170343221234_i64.into();
        let b: GarbledInt64 = 849283231234_i64.into();

        let result: i64 = (a + b).into(); // Perform addition on the 64-bit values
        assert_eq!(result, 170343221234_i64 + 849283231234_i64);
    }

    #[test]
    fn test_from_i128_add() {
        // use larger values to test the 128-bit addition
        let a: GarbledInt128 = 170343221234567890_i128.into();
        let b: GarbledInt128 = 849283231234567890_i128.into();

        let result: i128 = (a + b).into(); // Perform addition on the 128-bit values
        assert_eq!(result, 170343221234567890_i128 + 849283231234567890_i128);
    }

    #[test]
    fn test_from_u8_sub() {
        let a: GarbledUint8 = 170_u8.into(); // Binary 10101010
        let b: GarbledUint8 = 100_u8.into(); // Binary 01100100

        let result: u8 = (a - b).into();
        assert_eq!(result, 170_u8 - 100_u8); // Expected result of subtraction between 10101010 and 01010101
    }

    #[test]
    fn test_from_u16_sub() {
        let a: GarbledUint16 = 43707_u16.into(); // Binary 1010101010101011
        let b: GarbledUint16 = 21845_u16.into(); // Binary 0101010101010101

        let result: u16 = (a - b).into();
        assert_eq!(result, 43707_u16 - 21845_u16); // Expected result of subtraction between 1010101010101011 and 0101010101010101
    }

    #[test]
    fn test_from_u32_sub() {
        let a: GarbledUint32 = 2863311530_u32.into(); // Binary 10101010101010101010101010101010
        let b: GarbledUint32 = 1431655765_u32.into(); // Binary 01010101010101010101010101010101

        let result: u32 = (a - b).into();
        assert_eq!(result, 2863311530_u32 - 1431655765_u32); // Expected result of subtraction between 10101010101010101010101010101010 and 01010101010101010101010101010101
    }

    #[test]
    fn test_from_u64_sub() {
        let a: GarbledUint64 = 12297829382473034410_u64.into(); // Binary 1010101010101010101010101010101010101010101010101010101010101010
        let b: GarbledUint64 = 6148914691236517205_u64.into(); // Binary 0101010101010101010101010101010101010101010101010101010101010101

        let result: u64 = (a - b).into();
        assert_eq!(result, 12297829382473034410_u64 - 6148914691236517205_u64);
        // Expected result of subtraction between 1010101010101010101010101010101010101010101010101010101010101010 and 0101010101010101010101010101010101010101010101010101010101010101
    }

    #[test]
    fn test_from_u128_sub() {
        let a: GarbledUint128 = 12297829382473034410_u128.into(); // Binary 10101010
        let b: GarbledUint128 = 6148914691236517205_u128.into(); // Binary 01010101

        let result: u128 = (a - b).into();
        assert_eq!(result, 12297829382473034410_u128 - 6148914691236517205_u128);
    }

    #[test]
    fn test_uint_mul() {
        let a: GarbledUint8 = 3_u8.into(); // Binary 0011
        let b: GarbledUint8 = 2_u8.into(); // Binary 0010

        let result: u8 = (a * b).into();
        assert_eq!(result, 6); // 0011 * 0010 = 0110
    }

    #[test]
    fn test_from_u8_mul() {
        let a: GarbledUint8 = 7_u8.into(); // Binary 0000 0111
        let b: GarbledUint8 = 5_u8.into(); // Binary 0000 0101

        let result: u8 = (a * b).into();
        assert_eq!(result, 35); // Binary 0010 0011
    }

    #[test]
    fn test_from_u16_mul() {
        let a: GarbledUint16 = 300_u16.into(); // Binary 1010101010101011
        let b: GarbledUint16 = 7_u16.into(); // Binary 0101010101010101

        let result: u16 = (a * b).into();
        assert_eq!(result, 300_u16 * 7_u16); // Expected result of multiplication between 1010101010101011 and 0101010101010101
    }

    #[test]
    fn test_int_sub() {
        let a: GarbledInt4 = 3_i8.into();
        let b: GarbledInt4 = 2_i8.into();

        let result: i8 = (a - b).into();
        assert_eq!(result, 3_i8 - 2_i8);
    }

    #[test]
    fn test_from_i8_sub() {
        let a: GarbledInt8 = 3_i8.into();
        let b: GarbledInt8 = (-2_i8).into();

        let result: i8 = (a - b).into(); // Perform subtraction on the 8-bit values
        assert_eq!(result, 3_i8 - (-2_i8)); // Expected result of subtraction between 3 and -2
    }

    #[test]
    fn test_from_i16_sub() {
        // use larger values to test the 16-bit subtraction
        let a: GarbledInt16 = 1340_i16.into();
        let b: GarbledInt16 = 8543_i16.into();

        let result: i16 = (a - b).into(); // Perform subtraction on the 16-bit values
        assert_eq!(result, 1340_i16 - 8543_i16);
    }

    #[test]
    fn test_from_i32_sub() {
        // use larger values to test the 32-bit subtraction
        let a: GarbledInt32 = 17034322_i32.into();
        let b: GarbledInt32 = 84928323_i32.into();

        let result: i32 = (a - b).into(); // Perform subtraction on the 32-bit values
        assert_eq!(result, 17034322_i32 - 84928323_i32);
    }

    #[test]
    fn test_from_i64_sub() {
        // use larger values to test the 64-bit subtraction
        let a: GarbledInt64 = 170343221234_i64.into();
        let b: GarbledInt64 = 849283231234_i64.into();

        let result: i64 = (a - b).into(); // Perform subtraction on the 64-bit values
        assert_eq!(result, 170343221234_i64 - 849283231234_i64);
    }

    #[test]
    fn test_from_i128_sub() {
        // use larger values to test the 128-bit subtraction
        let a: GarbledInt128 = 170343221234567890_i128.into();
        let b: GarbledInt128 = 849283231234567890_i128.into();

        let result: i128 = (a - b).into(); // Perform subtraction on the 128-bit values
        assert_eq!(result, 170343221234567890_i128 - 849283231234567890_i128);
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
}
