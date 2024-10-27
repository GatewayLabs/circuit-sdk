use crate::operations::circuits::builder::GateIndex;
use crate::uint::GarbledBoolean;

#[derive(Default, Debug, Eq, Hash, PartialEq, Clone)]
pub struct GateIndexVec(Vec<GateIndex>);

impl GateIndexVec {
    pub fn new(indices: Vec<GateIndex>) -> Self {
        Self(indices)
    }

    pub fn push(&mut self, value: GateIndex) {
        self.0.push(value);
    }

    pub fn push_all(&mut self, values: &GateIndexVec) {
        self.0.extend_from_slice(&values.0);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<GateIndex> {
        self.0.iter()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn insert(&mut self, index: usize, element: GateIndex) {
        self.0.insert(index, element);
    }

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }
}

// Implement indexing for GateVector
impl std::ops::Index<usize> for GateIndexVec {
    type Output = GateIndex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<GateIndexVec> for Vec<u32> {
    fn from(vec: GateIndexVec) -> Self {
        vec.0.to_vec()
    }
}

impl From<Vec<GateIndex>> for GateIndexVec {
    fn from(vec: Vec<GateIndex>) -> Self {
        Self(vec)
    }
}

impl From<GateIndexVec> for GarbledBoolean {
    fn from(vec: GateIndexVec) -> Self {
        GarbledBoolean::from(vec.0[0])
    }
}

impl From<Vec<&u32>> for GateIndexVec {
    fn from(vec: Vec<&u32>) -> Self {
        let mut indices = Vec::new();
        for index in vec {
            indices.push(*index);
        }
        Self(indices)
    }
}

impl From<&u32> for GateIndexVec {
    fn from(index: &u32) -> Self {
        Self(vec![*index])
    }
}

impl From<u32> for GateIndexVec {
    fn from(index: u32) -> Self {
        Self(vec![index])
    }
}

impl From<&GateIndexVec> for GateIndexVec {
    fn from(vec: &GateIndexVec) -> Self {
        vec.clone()
    }
}

impl From<GateIndexVec> for GateIndex {
    fn from(vec: GateIndexVec) -> Self {
        vec.0[0]
    }
}

impl From<&GateIndexVec> for GateIndex {
    fn from(vec: &GateIndexVec) -> Self {
        vec.0[0]
    }
}

/*
use crate::operations::circuits::builder::CircuitBuilder;

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Sub, SubAssign,
};

// Implement Add trait for GateIndexVec using the builder reference
impl Add for GateIndexVec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        CircuitBuilder::global(|builder| builder.add(&self, &other))
    }
}

impl Add for &GateIndexVec {
    type Output = GateIndexVec;

    fn add(self, other: Self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.add(self, other))
    }
}

impl AddAssign for GateIndexVec {
    fn add_assign(&mut self, other: Self) {
        *self = CircuitBuilder::global(|builder| builder.add(&self, &other));
    }
}

impl AddAssign<&GateIndexVec> for GateIndexVec {
    fn add_assign(&mut self, other: &Self) {
        *self = CircuitBuilder::global(|builder| builder.add(self, other));
    }
}

impl Sub for GateIndexVec {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        CircuitBuilder::global(|builder| builder.sub(&self, &other))
    }
}

impl Sub for &GateIndexVec {
    type Output = GateIndexVec;

    fn sub(self, other: Self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.sub(self, other))
    }
}

impl SubAssign for GateIndexVec {
    fn sub_assign(&mut self, other: Self) {
        *self = CircuitBuilder::global(|builder| builder.sub(&self, &other));
    }
}

impl SubAssign<&GateIndexVec> for GateIndexVec {
    fn sub_assign(&mut self, other: &Self) {
        *self = CircuitBuilder::global(|builder| builder.sub(self, other));
    }
}

impl Mul for GateIndexVec {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        CircuitBuilder::global(|builder| builder.mul(&self, &other))
    }
}

impl Mul for &GateIndexVec {
    type Output = GateIndexVec;

    fn mul(self, other: Self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.mul(self, other))
    }
}

impl MulAssign for GateIndexVec {
    fn mul_assign(&mut self, other: Self) {
        *self = CircuitBuilder::global(|builder| builder.mul(&self, &other));
    }
}

impl MulAssign<&GateIndexVec> for GateIndexVec {
    fn mul_assign(&mut self, other: &Self) {
        *self = CircuitBuilder::global(|builder| builder.mul(self, other));
    }
}

impl Div for GateIndexVec {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        CircuitBuilder::global(|builder| builder.div(&self, &other))
    }
}

impl Div for &GateIndexVec {
    type Output = GateIndexVec;

    fn div(self, other: Self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.div(self, other))
    }
}

impl DivAssign for GateIndexVec {
    fn div_assign(&mut self, other: Self) {
        *self = CircuitBuilder::global(|builder| builder.div(&self, &other));
    }
}

impl DivAssign<&GateIndexVec> for GateIndexVec {
    fn div_assign(&mut self, other: &Self) {
        *self = CircuitBuilder::global(|builder| builder.div(self, other));
    }
}

impl Rem for GateIndexVec {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        CircuitBuilder::global(|builder| builder.rem(&self, &other))
    }
}

impl Rem for &GateIndexVec {
    type Output = GateIndexVec;

    fn rem(self, other: Self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.rem(self, other))
    }
}

impl RemAssign for GateIndexVec {
    fn rem_assign(&mut self, other: Self) {
        *self = CircuitBuilder::global(|builder| builder.rem(&self, &other));
    }
}

impl RemAssign<&GateIndexVec> for GateIndexVec {
    fn rem_assign(&mut self, other: &Self) {
        *self = CircuitBuilder::global(|builder| builder.rem(self, other));
    }
}

impl BitAnd for GateIndexVec {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        CircuitBuilder::global(|builder| builder.and(&self, &other))
    }
}

impl BitAnd for &GateIndexVec {
    type Output = GateIndexVec;

    fn bitand(self, other: Self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.and(self, other))
    }
}

impl BitAndAssign for GateIndexVec {
    fn bitand_assign(&mut self, other: Self) {
        *self = CircuitBuilder::global(|builder| builder.and(&self, &other));
    }
}

impl BitAndAssign<&GateIndexVec> for GateIndexVec {
    fn bitand_assign(&mut self, other: &Self) {
        *self = CircuitBuilder::global(|builder| builder.and(self, other));
    }
}

impl BitOr for GateIndexVec {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        CircuitBuilder::global(|builder| builder.or(&self, &other))
    }
}

impl BitOr for &GateIndexVec {
    type Output = GateIndexVec;

    fn bitor(self, other: Self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.or(self, other))
    }
}

impl BitOrAssign for GateIndexVec {
    fn bitor_assign(&mut self, other: Self) {
        *self = CircuitBuilder::global(|builder| builder.or(&self, &other));
    }
}

impl BitOrAssign<&GateIndexVec> for GateIndexVec {
    fn bitor_assign(&mut self, other: &Self) {
        *self = CircuitBuilder::global(|builder| builder.or(self, other));
    }
}

impl Not for GateIndexVec {
    type Output = Self;

    fn not(self) -> Self {
        CircuitBuilder::global(|builder| builder.not(&self))
    }
}

impl Not for &GateIndexVec {
    type Output = GateIndexVec;

    fn not(self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.not(self))
    }
}

impl BitXor for GateIndexVec {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        CircuitBuilder::global(|builder| builder.xor(&self, &other))
    }
}

impl BitXor for &GateIndexVec {
    type Output = GateIndexVec;

    fn bitxor(self, other: Self) -> GateIndexVec {
        CircuitBuilder::global(|builder| builder.xor(self, other))
    }
}

impl BitXorAssign for GateIndexVec {
    fn bitxor_assign(&mut self, other: Self) {
        *self = CircuitBuilder::global(|builder| builder.xor(&self, &other));
    }
}

impl BitXorAssign<&GateIndexVec> for GateIndexVec {
    fn bitxor_assign(&mut self, other: &Self) {
        *self = CircuitBuilder::global(|builder| builder.xor(self, other));
    }
}
*/
