use crate::int::GarbledInt;
use crate::operations::circuits::{
    build_and_execute_addition, build_and_execute_multiplication, build_and_execute_subtraction,
};
use crate::uint::GarbledUint;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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

// Implement the AddAssign operation for Uint<N> and &GarbledUint<N>
impl<const N: usize> AddAssign for GarbledUint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = build_and_execute_addition(self, &rhs);
    }
}

impl<const N: usize> AddAssign<&GarbledUint<N>> for GarbledUint<N> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_addition(self, rhs);
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

// Implement the SubAssign operation for GarbledUint<N> and &GarbledUint<N>
impl<const N: usize> SubAssign for GarbledUint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = build_and_execute_subtraction(self, &rhs);
    }
}

impl<const N: usize> SubAssign<&GarbledUint<N>> for GarbledUint<N> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_subtraction(self, rhs);
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

// Implement the MulAssign operation for GarbledUint<N> and &GarbledUint<N>
impl<const N: usize> MulAssign for GarbledUint<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = build_and_execute_multiplication(self, &rhs);
    }
}

impl<const N: usize> MulAssign<&GarbledUint<N>> for GarbledUint<N> {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_multiplication(self, rhs);
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

// Implement the AddAssign operation for GarbledInt<N> and &GarbledInt<N>
impl<const N: usize> AddAssign for GarbledInt<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = build_and_execute_addition(&self.clone().into(), &rhs.into()).into();
    }
}

impl<const N: usize> AddAssign<&GarbledInt<N>> for GarbledInt<N> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_addition(&self.clone().into(), &rhs.into()).into();
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

// Implement the SubAssign operation for GarbledInt<N> and &GarbledInt<N>
impl<const N: usize> SubAssign for GarbledInt<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = build_and_execute_subtraction(&self.clone().into(), &rhs.into()).into();
    }
}

impl<const N: usize> SubAssign<&GarbledInt<N>> for GarbledInt<N> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_subtraction(&self.clone().into(), &rhs.into()).into();
    }
}

// Implement the Mul operation for GarbledInt<N> and &GarbledInt<N>
impl<const N: usize> Mul for GarbledInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        build_and_execute_multiplication(&self.into(), &rhs.into()).into()
    }
}

impl<const N: usize> Mul for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        build_and_execute_multiplication(&self.into(), &rhs.into()).into()
    }
}

// Implement the MulAssign operation for GarbledInt<N> and &GarbledInt<N>
impl<const N: usize> MulAssign for GarbledInt<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = build_and_execute_multiplication(&self.clone().into(), &rhs.into()).into();
    }
}

impl<const N: usize> MulAssign<&GarbledInt<N>> for GarbledInt<N> {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_multiplication(&self.clone().into(), &rhs.into()).into();
    }
}
