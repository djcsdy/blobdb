use crate::volume::block_device::block_group_count::BlockGroupCount;
use crate::volume::block_device::block_group_index::BlockGroupIndex;

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct Extent {
    pub first_block_group_index: BlockGroupIndex,
    pub block_group_count: BlockGroupCount,
}

impl Extent {
    pub fn end_block_group_index(&self) -> BlockGroupIndex {
        self.first_block_group_index + self.block_group_count
    }

    pub fn split(self, block_group_count: BlockGroupCount) -> (Self, Option<Self>) {
        if block_group_count < self.block_group_count {
            (
                Extent {
                    first_block_group_index: self.first_block_group_index,
                    block_group_count,
                },
                Some(Extent {
                    first_block_group_index: self.first_block_group_index + block_group_count,
                    block_group_count: self.block_group_count - block_group_count,
                }),
            )
        } else if block_group_count == self.block_group_count {
            (self, None)
        } else {
            panic!("block_group_count must be less than or equal to extent block_group_count");
        }
    }

    pub fn merge(self, other: Self) -> Option<Self> {
        if self.first_block_group_index < other.first_block_group_index {
            if self.first_block_group_index + self.block_group_count
                == other.first_block_group_index
            {
                Some(Extent {
                    first_block_group_index: self.first_block_group_index,
                    block_group_count: self.block_group_count + other.block_group_count,
                })
            } else {
                None
            }
        } else {
            other.merge(self)
        }
    }

    pub fn overlaps(self, other: Self) -> bool {
        (self.first_block_group_index <= other.first_block_group_index
            && self.end_block_group_index() > other.first_block_group_index)
            || (self.first_block_group_index > other.first_block_group_index
                && self.first_block_group_index < other.end_block_group_index())
    }
}
