use crate::int::GarbledInt;
use crate::operations::circuits::builder::{build_and_execute_comparator, build_and_execute_equality};
use crate::uint::GarbledUint;
use std::cmp::Ordering;

// Implementing comparison operators for GarbledUint
impl<const N: usize> PartialEq for GarbledUint<N> {
    fn eq(&self, other: &Self) -> bool {
        matches!(build_and_execute_comparator(self, other), Ordering::Equal)
    }
}

// Implementing equality for GarbledUint
impl<const N: usize> Eq for GarbledUint<N> {
    // This is a no-op because the implementation of `Ord` is correct
    // and the implementation of `Eq` is derived from `Ord`.
}

// Implementing comparison operators for GarbledUint
#[allow(clippy::non_canonical_partial_ord_impl)]
impl<const N: usize> PartialOrd for GarbledUint<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(build_and_execute_comparator(self, other))
    }
}

// Implementing comparison operators for GarbledUint
impl<const N: usize> Ord for GarbledUint<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        build_and_execute_comparator(self, other)
    }
}

// Implementing comparison operators for GarbledUint
impl<const N: usize> PartialEq<&GarbledUint<N>> for GarbledUint<N> {
    fn eq(&self, other: &&Self) -> bool {
        !build_and_execute_equality(self, other)
    }
}

// Implementing comparison operators for GarbledUint
impl<const N: usize> PartialOrd<&GarbledUint<N>> for GarbledUint<N> {
    fn partial_cmp(&self, other: &&Self) -> Option<Ordering> {
        Some(build_and_execute_comparator(self, *other))
    }
}

// Implementing comparison operators for GarbledInt
impl<const N: usize> PartialEq for GarbledInt<N> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            build_and_execute_comparator(&self.into(), &other.into()),
            Ordering::Equal
        )
    }
}

// Implementing equality for GarbledInt
impl<const N: usize> Eq for GarbledInt<N> {
    // This is a no-op because the implementation of `Ord` is correct
    // and the implementation of `Eq` is derived from `Ord`.
}

// Implementing comparison operators for GarbledInt
#[allow(clippy::non_canonical_partial_ord_impl)]
impl<const N: usize> PartialOrd for GarbledInt<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(build_and_execute_comparator(&self.into(), &other.into()))
    }
}

// Implementing comparison operators for GarbledInt
impl<const N: usize> Ord for GarbledInt<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        build_and_execute_comparator(&self.into(), &other.into())
    }
}
