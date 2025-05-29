use crate::units::block_group_index::BlockGroupIndex;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Into, Sub, SubAssign};
use std::cmp::Ordering;

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
#[display("{_0} Block Groups")]
pub struct BlockGroupCount(pub u64);

impl PartialEq<BlockGroupIndex> for BlockGroupCount {
    fn eq(&self, other: &BlockGroupIndex) -> bool {
        (**self).eq(&**other)
    }
}

impl PartialOrd<BlockGroupIndex> for BlockGroupCount {
    fn partial_cmp(&self, other: &BlockGroupIndex) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }

    fn lt(&self, other: &BlockGroupIndex) -> bool {
        (**self).lt(&**other)
    }

    fn le(&self, other: &BlockGroupIndex) -> bool {
        (**self).le(&**other)
    }

    fn gt(&self, other: &BlockGroupIndex) -> bool {
        (**self).gt(&**other)
    }

    fn ge(&self, other: &BlockGroupIndex) -> bool {
        (**self).ge(&**other)
    }
}
