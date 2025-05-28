use crate::volume::block_device::block_group_count::BlockGroupCount;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Into, Sub, SubAssign};
use std::cmp::Ordering;
use std::ops::Add;

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
    Into,
    Display,
    Deref,
    DerefMut,
)]
#[display("Block Group {_0}")]
pub struct BlockGroupIndex(pub u64);

impl Add<BlockGroupCount> for BlockGroupIndex {
    type Output = Self;

    fn add(self, rhs: BlockGroupCount) -> Self {
        Self((*self).add(*rhs))
    }
}

impl PartialEq<BlockGroupCount> for BlockGroupIndex {
    fn eq(&self, other: &BlockGroupCount) -> bool {
        (**self).eq(&**other)
    }
}

impl PartialOrd<BlockGroupCount> for BlockGroupIndex {
    fn partial_cmp(&self, other: &BlockGroupCount) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }

    fn lt(&self, other: &BlockGroupCount) -> bool {
        (**self).lt(&**other)
    }

    fn le(&self, other: &BlockGroupCount) -> bool {
        (**self).le(&**other)
    }

    fn gt(&self, other: &BlockGroupCount) -> bool {
        (**self).gt(&**other)
    }

    fn ge(&self, other: &BlockGroupCount) -> bool {
        (**self).ge(&**other)
    }
}
