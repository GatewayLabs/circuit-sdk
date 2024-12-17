use crate::operations::circuits::builder::GateIndex;
use crate::uint::GarbledBoolean;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
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

#[allow(clippy::from_over_into)]
impl<'a> Into<&'a GateIndex> for &'a GateIndexVec {
    fn into(self) -> &'a GateIndex {
        &self.0[0]
    }
}

impl<'a> From<&'a &GateIndexVec> for &'a GateIndexVec {
    fn from(vec: &'a &GateIndexVec) -> Self {
        vec
    }
}

impl<'a> From<&'a &mut &GateIndexVec> for &'a GateIndexVec {
    fn from(vec: &'a &mut &GateIndexVec) -> Self {
        vec
    }
}

impl<'a> From<&'a mut &GateIndexVec> for &'a GateIndexVec {
    fn from(vec: &'a mut &GateIndexVec) -> Self {
        vec
    }
}
