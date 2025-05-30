use crate::block::Block;
use crate::units::BlockCount;
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
#[display("{_0} bytes")]
pub struct ByteCount<T>(pub T)
where
    T: Unsigned + PrimInt + Display;

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

impl<T> ByteCount<T>
where
    T: Unsigned + PrimInt + Display,
{
    pub fn to_block_count(self) -> BlockCount<T> {
        BlockCount(*self / T::from(*Block::SIZE).unwrap())
    }
}
