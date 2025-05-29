use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Sub, SubAssign};
use std::fmt::Display;

#[derive(
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Copy,
    Hash,
    Debug,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    From,
    Display,
    Deref,
    DerefMut,
)]
#[display("{_0} bytes")]
pub struct ByteCount<T>(pub T)
where
    T: Copy + Ord + Display;

impl From<ByteCount<u32>> for u32 {
    fn from(value: ByteCount<u32>) -> Self {
        value.0
    }
}

impl From<ByteCount<u64>> for u64 {
    fn from(value: ByteCount<u64>) -> Self {
        value.0
    }
}
