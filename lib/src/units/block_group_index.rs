use crate::units::Count;
use crate::units::block_group_count::BlockGroupCount;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Into, Sub, SubAssign};
use std::cmp::Ordering;
use std::ops::{Add, Range, RangeInclusive, Shr};

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

impl Shr<u64> for BlockGroupIndex {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self {
        Self(*self >> rhs)
    }
}

impl Count for Range<BlockGroupIndex> {
    type Output = BlockGroupCount;

    fn count(&self) -> Self::Output {
        if self.end < self.start {
            panic!("Invalid Range<BlockGroupIndex>");
        }
        BlockGroupCount(*(self.end - self.start))
    }
}

impl Count for RangeInclusive<BlockGroupIndex> {
    type Output = BlockGroupCount;

    fn count(&self) -> Self::Output {
        if self.end() < self.start() {
            panic!("Invalid RangeInclusive<BlockGroupIndex>");
        }
        BlockGroupCount(*(*self.end() - *self.start()) + 1)
    }
}
