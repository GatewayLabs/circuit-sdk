use crate::int::GarbledInt;
use crate::operations::circuits::{
    build_and_execute_and, build_and_execute_nand, build_and_execute_nor, build_and_execute_not,
    build_and_execute_or, build_and_execute_xnor, build_and_execute_xor,
};
use crate::uint::GarbledUint;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

// Implement the XOR operation for Uint<N>
impl<const N: usize> BitXor for GarbledUint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_execute_xor(&self, &rhs)
    }
}

// Implement the XOR operation for &GarbledUint<N>
impl<const N: usize> BitXor for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_execute_xor(self, rhs)
    }
}

// Implement the XOR operation for GarbledInt<N>
impl<const N: usize> BitXor for GarbledInt<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_execute_xor(&self.into(), &rhs.into()).into()
    }
}

// Implement the XOR operation for &GarbledInt<N>
impl<const N: usize> BitXor for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        build_and_execute_xor(&self.into(), &rhs.into()).into()
    }
}

// Implement the AND operation for Uint<N>
impl<const N: usize> BitAnd for GarbledUint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_execute_and(&self, &rhs)
    }
}

// Implement the AND operation for &GarbledUint<N>
impl<const N: usize> BitAnd for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_execute_and(self, rhs)
    }
}

// Implement the AND operation for GarbledInt<N>
impl<const N: usize> BitAnd for GarbledInt<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_execute_and(&self.into(), &rhs.into()).into()
    }
}

// Implement the AND operation for &GarbledInt<N>
impl<const N: usize> BitAnd for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        build_and_execute_and(&self.into(), &rhs.into()).into()
    }
}

// Implement the NOT operation for Uint<N>
impl<const N: usize> Not for GarbledUint<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        build_and_execute_not(&self)
    }
}

// Implement the NOT operation for &GarbledUint<N>
impl<const N: usize> Not for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn not(self) -> Self::Output {
        build_and_execute_not(self)
    }
}

// Implement the OR operation for GarbledUint<N>
impl<const N: usize> BitOr for GarbledUint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        build_and_execute_or(&self, &rhs)
    }
}

// Implement the OR operation for &GarbledUint<N>
impl<const N: usize> BitOr for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        build_and_execute_or(self, rhs)
    }
}

// Implement the OR operation for GarbledInt<N>
impl<const N: usize> BitOr for GarbledInt<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        build_and_execute_or(&self.into(), &rhs.into()).into()
    }
}

// Implement the OR operation for &GarbledInt<N>
impl<const N: usize> BitOr for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        build_and_execute_or(&self.into(), &rhs.into()).into()
    }
}

// Implement the NOT operation for GarbledInt<N>
impl<const N: usize> Not for GarbledInt<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        build_and_execute_not(&self.into()).into()
    }
}

// Implement the NOT operation for &GarbledInt<N>
impl<const N: usize> Not for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn not(self) -> Self::Output {
        build_and_execute_not(&self.into()).into()
    }
}

// Helper function for shift operations
fn shift_bits_left<const N: usize>(bits: &mut Vec<bool>, shift: usize) {
    for _ in 0..shift {
        bits.remove(N - 1); // Remove the most significant bit
        bits.insert(0, false); // Insert a 0 to the least significant bit
    }
}

fn shift_bits_right<const N: usize>(bits: &mut Vec<bool>, shift: usize) {
    for _ in 0..shift {
        bits.remove(0); // Remove the least significant bit
        bits.push(false); // Insert a 0 to the most significant bit
    }
}

// Implement Shift Left operation for Uint<N>
impl<const N: usize> Shl<usize> for GarbledUint<N> {
    type Output = Self;

    fn shl(mut self, shift: usize) -> Self::Output {
        shift_bits_left::<N>(&mut self.bits, shift);
        self
    }
}

// Implement Shift Left operation for &GarbledUint<N>
impl<const N: usize> Shl<usize> for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_left::<N>(&mut bits, shift);
        GarbledUint::new(bits)
    }
}

// Implement Shift Left operation for GarbledInt<N>
impl<const N: usize> Shl<usize> for GarbledInt<N> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits;
        shift_bits_left::<N>(&mut bits, shift);
        GarbledInt::new(bits)
    }
}

// Implement Shift Left operation for &GarbledInt<N>
impl<const N: usize> Shl<usize> for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn shl(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_left::<N>(&mut bits, shift);
        GarbledInt::new(bits)
    }
}

// Implement Shift Right operation for Uint<N>
impl<const N: usize> Shr<usize> for GarbledUint<N> {
    type Output = Self;

    fn shr(mut self, shift: usize) -> Self::Output {
        shift_bits_right::<N>(&mut self.bits, shift);
        self
    }
}

// Implement Shift Right operation for &GarbledUint<N>
impl<const N: usize> Shr<usize> for &GarbledUint<N> {
    type Output = GarbledUint<N>;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_right::<N>(&mut bits, shift);
        GarbledUint::new(bits)
    }
}

// Implement Shift Right operation for GarbledInt<N>
impl<const N: usize> Shr<usize> for GarbledInt<N> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits;
        shift_bits_right::<N>(&mut bits, shift);
        GarbledInt::new(bits)
    }
}

// Implement Shift Right operation for &GarbledInt<N>
impl<const N: usize> Shr<usize> for &GarbledInt<N> {
    type Output = GarbledInt<N>;

    fn shr(self, shift: usize) -> Self::Output {
        let mut bits = self.bits.clone();
        shift_bits_right::<N>(&mut bits, shift);
        GarbledInt::new(bits)
    }
}

// Implement the NAND, NOR, XNOR operators for GarbledUint<N>
impl<const N: usize> GarbledUint<N> {
    pub fn nand(self, rhs: Self) -> Self {
        build_and_execute_nand(&self, &rhs)
    }

    pub fn nor(self, rhs: Self) -> Self {
        build_and_execute_nor(&self, &rhs)
    }

    pub fn xnor(self, rhs: Self) -> Self {
        build_and_execute_xnor(&self, &rhs)
    }
}

// Implement the NAND, NOR, XNOR operators for GarbledInt<N>
impl<const N: usize> GarbledInt<N> {
    pub fn nand(self, rhs: Self) -> Self {
        build_and_execute_nand(&self.into(), &rhs.into()).into()
    }

    pub fn nor(self, rhs: Self) -> Self {
        build_and_execute_nor(&self.into(), &rhs.into()).into()
    }

    pub fn xnor(self, rhs: Self) -> Self {
        build_and_execute_xnor(&self.into(), &rhs.into()).into()
    }
}
