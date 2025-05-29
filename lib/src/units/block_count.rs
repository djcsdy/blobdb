use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Sub, SubAssign};
use num_traits::{PrimInt, Unsigned};
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
#[display("{_0} blocks")]
pub struct BlockCount<T>(pub T)
where
    T: Unsigned + PrimInt + Display;

impl From<BlockCount<u32>> for u32 {
    fn from(value: BlockCount<u32>) -> Self {
        value.0
    }
}

impl From<BlockCount<u64>> for u64 {
    fn from(value: BlockCount<u64>) -> Self {
        value.0
    }
}
