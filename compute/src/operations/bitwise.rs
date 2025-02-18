use crate::int::GarbledInt;
use crate::operations::circuits::builder::{
    build_and_execute_and, build_and_execute_nand, build_and_execute_nor, build_and_execute_not,
    build_and_execute_or, build_and_execute_shl, build_and_execute_shr, build_and_execute_xnor,
    build_and_execute_xor,
};
use crate::uint::GarbledUint;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

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

// Implement the XorAssign operation for Uint<N>
impl<const N: usize> BitXorAssign for GarbledUint<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = build_and_execute_xor(self, &rhs);
    }
}

// Implement the XorAssign operation for &GarbledUint<N>
impl<const N: usize> BitXorAssign<&GarbledUint<N>> for GarbledUint<N> {
    fn bitxor_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_xor(self, rhs);
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

// Implement the XorAssign operation for GarbledInt<N>
impl<const N: usize> BitXorAssign for GarbledInt<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = build_and_execute_xor(&self.clone().into(), &rhs.into()).into();
    }
}

// Implement the XorAssign operation for &GarbledInt<N>
impl<const N: usize> BitXorAssign<&GarbledInt<N>> for GarbledInt<N> {
    fn bitxor_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_xor(&self.clone().into(), &rhs.into()).into();
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

// Implement the BitAndAssign operation for Uint<N>
impl<const N: usize> BitAndAssign for GarbledUint<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = build_and_execute_and(self, &rhs);
    }
}

// Implement the BitAndAssign operation for &GarbledUint<N>
impl<const N: usize> BitAndAssign<&GarbledUint<N>> for GarbledUint<N> {
    fn bitand_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_and(self, rhs);
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

// Implement the BitAndAssign operation for GarbledInt<N>
impl<const N: usize> BitAndAssign for GarbledInt<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = build_and_execute_and(&self.clone().into(), &rhs.into()).into();
    }
}

// Implement the BitAndAssign operation for &GarbledInt<N>
impl<const N: usize> BitAndAssign<&GarbledInt<N>> for GarbledInt<N> {
    fn bitand_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_and(&self.clone().into(), &rhs.into()).into();
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

// Implement the BitOrAssign operation for Uint<N>
impl<const N: usize> BitOrAssign for GarbledUint<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = build_and_execute_or(self, &rhs);
    }
}

// Implement the BitOrAssign operation for &GarbledUint<N>
impl<const N: usize> BitOrAssign<&GarbledUint<N>> for GarbledUint<N> {
    fn bitor_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_or(self, rhs);
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

// Implement the BitOrAssign operation for GarbledInt<N>
impl<const N: usize> BitOrAssign for GarbledInt<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = build_and_execute_or(&self.clone().into(), &rhs.into()).into();
    }
}

// Implement the BitOrAssign operation for &GarbledInt<N>
impl<const N: usize> BitOrAssign<&GarbledInt<N>> for GarbledInt<N> {
    fn bitor_assign(&mut self, rhs: &Self) {
        *self = build_and_execute_or(&self.clone().into(), &rhs.into()).into();
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

// Implement the Shift-left operation for GarbledUint<N>
impl<const N: usize, const K: usize> Shl<&GarbledUint<K>> for GarbledUint<N> {
    type Output = GarbledUint<N>;
    fn shl(self, rhs: &GarbledUint<K>) -> Self::Output {
        build_and_execute_shl::<N, K>(&self, rhs)
    }
}

// Implement the Shift-left operation for &GarbledUint<N>
impl<const N: usize, const K: usize> Shl<&GarbledUint<K>> for &GarbledUint<N> {
    type Output = GarbledUint<N>;
    fn shl(self, rhs: &GarbledUint<K>) -> Self::Output {
        build_and_execute_shl::<N, K>(self, rhs)
    }
}

// Implement the Shift-left operation for GarbledUint<N> with a literal shift amount
impl<const N: usize> Shl<usize> for GarbledUint<N> {
    type Output = GarbledUint<N>;
    fn shl(self, shift: usize) -> Self::Output {
        let shift_wire: GarbledUint<8> = shift.into();
        build_and_execute_shl::<N, 8>(&self, &shift_wire)
    }
}

// Implement the Shift-left operation for &GarbledUint<N> with a literal shift amount
impl<const N: usize> Shl<usize> for &GarbledUint<N> {
    type Output = GarbledUint<N>;
    fn shl(self, shift: usize) -> Self::Output {
        let shift_wire: GarbledUint<8> = shift.into();
        build_and_execute_shl::<N, 8>(self, &shift_wire)
    }
}

// Implement the Shift-left assignment operation for GarbledUint<N>
impl<const N: usize, const K: usize> ShlAssign<&GarbledUint<K>> for GarbledUint<N> {
    fn shl_assign(&mut self, rhs: &GarbledUint<K>) {
        *self = self.clone().shl(rhs);
    }
}

// Implement the Shift-left assignment operation for GarbledUint<N> with a literal shift amount
impl<const N: usize> ShlAssign<usize> for GarbledUint<N> {
    fn shl_assign(&mut self, shift: usize) {
        *self = self.clone().shl(shift);
    }
}

// Implement the Shift-right operation for GarbledUint<N>
impl<const N: usize, const K: usize> Shr<&GarbledUint<K>> for GarbledUint<N> {
    type Output = GarbledUint<N>;
    fn shr(self, rhs: &GarbledUint<K>) -> Self::Output {
        build_and_execute_shr::<N, K>(&self, rhs)
    }
}

// Implement the Shift-right operation for &GarbledUint<N>
impl<const N: usize, const K: usize> Shr<&GarbledUint<K>> for &GarbledUint<N> {
    type Output = GarbledUint<N>;
    fn shr(self, rhs: &GarbledUint<K>) -> Self::Output {
        build_and_execute_shr::<N, K>(self, rhs)
    }
}

// Implement the Shift-right operation for GarbledUint<N> with a literal shift amount
impl<const N: usize> Shr<usize> for GarbledUint<N> {
    type Output = GarbledUint<N>;
    fn shr(self, shift: usize) -> Self::Output {
        let shift_wire: GarbledUint<8> = shift.into();
        build_and_execute_shr::<N, 8>(&self, &shift_wire)
    }
}

// Implement the Shift-right operation for &GarbledUint<N> with a literal shift amount
impl<const N: usize> Shr<usize> for &GarbledUint<N> {
    type Output = GarbledUint<N>;
    fn shr(self, shift: usize) -> Self::Output {
        let shift_wire: GarbledUint<8> = shift.into();
        build_and_execute_shr::<N, 8>(self, &shift_wire)
    }
}

// Implement the Shift-right assignment operation for GarbledUint<N>
impl<const N: usize, const K: usize> ShrAssign<&GarbledUint<K>> for GarbledUint<N> {
    fn shr_assign(&mut self, rhs: &GarbledUint<K>) {
        *self = self.clone().shr(rhs);
    }
}

// Implement the Shift-right assignment operation for GarbledUint<N> with a literal shift amount
impl<const N: usize> ShrAssign<usize> for GarbledUint<N> {
    fn shr_assign(&mut self, shift: usize) {
        *self = self.clone().shr(shift);
    }
}

// Implement the Shift-left operation for GarbledInt<N>
impl<const N: usize, const K: usize> Shl<&GarbledUint<K>> for GarbledInt<N> {
    type Output = GarbledInt<N>;
    fn shl(self, rhs: &GarbledUint<K>) -> Self::Output {
        let u: GarbledUint<N> = self.into();
        u.shl(rhs).into()
    }
}

// Implement the Shift-left operation for &GarbledInt<N>
impl<const N: usize, const K: usize> Shl<&GarbledUint<K>> for &GarbledInt<N> {
    type Output = GarbledInt<N>;
    fn shl(self, rhs: &GarbledUint<K>) -> Self::Output {
        let u: GarbledUint<N> = self.clone().into();
        u.shl(rhs).into()
    }
}

// Implement the Shift-left operation for GarbledInt<N> with a literal shift amount
impl<const N: usize> Shl<usize> for GarbledInt<N> {
    type Output = GarbledInt<N>;
    fn shl(self, shift: usize) -> Self::Output {
        let shift_wire: GarbledUint<8> = shift.into();
        let u: GarbledUint<N> = self.into();
        u.shl(&shift_wire).into()
    }
}

// Implement the Shift-left operation for &GarbledInt<N> with a literal shift amount
impl<const N: usize> Shl<usize> for &GarbledInt<N> {
    type Output = GarbledInt<N>;
    fn shl(self, shift: usize) -> Self::Output {
        let shift_wire: GarbledUint<8> = shift.into();
        let u: GarbledUint<N> = self.clone().into();
        u.shl(&shift_wire).into()
    }
}

// Implement the Shift-left assignment operation for GarbledInt<N>
impl<const N: usize, const K: usize> ShlAssign<&GarbledUint<K>> for GarbledInt<N> {
    fn shl_assign(&mut self, rhs: &GarbledUint<K>) {
        *self = self.clone().shl(rhs);
    }
}

// Implement the Shift-left assignment operation for GarbledInt<N> with a literal shift amount
impl<const N: usize> ShlAssign<usize> for GarbledInt<N> {
    fn shl_assign(&mut self, shift: usize) {
        *self = self.clone().shl(shift);
    }
}

// Implement the Shift-right operation for GarbledInt<N>
impl<const N: usize, const K: usize> Shr<&GarbledUint<K>> for GarbledInt<N> {
    type Output = GarbledInt<N>;
    fn shr(self, rhs: &GarbledUint<K>) -> Self::Output {
        let u: GarbledUint<N> = self.into();
        u.shr(rhs).into()
    }
}

// Implement the Shift-right operation for &GarbledInt<N>
impl<const N: usize, const K: usize> Shr<&GarbledUint<K>> for &GarbledInt<N> {
    type Output = GarbledInt<N>;
    fn shr(self, rhs: &GarbledUint<K>) -> Self::Output {
        let u: GarbledUint<N> = self.clone().into();
        u.shr(rhs).into()
    }
}

// Implement the Shift-right operation for GarbledInt<N> with a literal shift amount
impl<const N: usize> Shr<usize> for GarbledInt<N> {
    type Output = GarbledInt<N>;
    fn shr(self, shift: usize) -> Self::Output {
        let shift_wire: GarbledUint<8> = shift.into();
        let u: GarbledUint<N> = self.into();
        u.shr(&shift_wire).into()
    }
}

// Implement the Shift-right operation for &GarbledInt<N> with a literal shift amount
impl<const N: usize> Shr<usize> for &GarbledInt<N> {
    type Output = GarbledInt<N>;
    fn shr(self, shift: usize) -> Self::Output {
        let shift_wire: GarbledUint<8> = shift.into();
        let u: GarbledUint<N> = self.clone().into();
        u.shr(&shift_wire).into()
    }
}

// Implement the Shift-right assignment operation for GarbledInt<N>
impl<const N: usize, const K: usize> ShrAssign<&GarbledUint<K>> for GarbledInt<N> {
    fn shr_assign(&mut self, rhs: &GarbledUint<K>) {
        *self = self.clone().shr(rhs);
    }
}

// Implement the Shift-right assignment operation for GarbledInt<N> with a literal shift amount
impl<const N: usize> ShrAssign<usize> for GarbledInt<N> {
    fn shr_assign(&mut self, shift: usize) {
        *self = self.clone().shr(shift);
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
