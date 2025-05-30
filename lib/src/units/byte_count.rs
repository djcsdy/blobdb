use crate::block::Block;
use crate::units::{BlockCount, BlockGroupCount};
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
#[display("{_0} bytes")]
pub struct ByteCount(pub u64);

impl ByteCount {
    pub fn to_block_count(self) -> BlockCount {
        BlockCount(*self / *Block::SIZE)
    }

    pub fn to_block_group_count(self, block_group_size: BlockCount) -> BlockGroupCount {
        self.to_block_count().to_block_group_count(block_group_size)
    }
}
