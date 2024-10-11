use crate::int::GarbledInt;
use crate::operations::circuits::{
    build_and_execute_addition, build_and_execute_multiplication, build_and_execute_subtraction,
};
use crate::uint::GarbledUint;
use std::ops::{Add, Mul, Sub};

// Implement the Add operation for Uint<N> and &GarbledUint<N>
impl<const N: usize> Add for GarbledUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_execute_addition(&self, &rhs)
    }
}

impl<const N: usize> Add for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_execute_addition(self, rhs)
    }
}

// Implement the Sub operation for Uint<N> and &GarbledUint<N>
impl<const N: usize> Sub for GarbledUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_execute_subtraction(&self, &rhs)
    }
}

impl<const N: usize> Sub for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_execute_subtraction(self, rhs)
    }
}

impl<const N: usize> Mul for GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        build_and_execute_multiplication(&self, &rhs)
    }
}

impl<const N: usize> Mul for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        build_and_execute_multiplication(self, rhs)
    }
}

// Implement the Add operation for GarbledInt<N> and &GarbledInt<N>
impl<const N: usize> Add for GarbledInt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_execute_addition(&self.into(), &rhs.into()).into()
    }
}

impl<const N: usize> Add for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn add(self, rhs: Self) -> Self::Output {
        build_and_execute_addition(&self.into(), &rhs.into()).into()
    }
}

// Implement the Sub operation for GarbledInt<N> and &GarbledInt<N>
impl<const N: usize> Sub for GarbledInt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_execute_subtraction(&self.into(), &rhs.into()).into()
    }
}

impl<const N: usize> Sub for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        build_and_execute_subtraction(&self.into(), &rhs.into()).into()
    }
}
