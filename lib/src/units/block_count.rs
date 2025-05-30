use crate::units::BlockGroupCount;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, From, Into, Sub, SubAssign};

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
#[display("{_0} blocks")]
pub struct BlockCount(pub u64);

impl BlockCount {
    pub fn to_block_group_count(self, block_group_size: BlockCount) -> BlockGroupCount {
        BlockGroupCount(*self / *block_group_size)
    }
}
