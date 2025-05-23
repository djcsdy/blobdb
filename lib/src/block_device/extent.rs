use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct Extent {
    pub first_block_index: u64,
    pub block_count: u64,
}

impl Extent {
    pub fn end_block_index(&self) -> u64 {
        self.first_block_index + self.block_count
    }

    pub fn split(self, block_count: u64) -> (Self, Option<Self>) {
        if block_count < self.block_count {
            (
                Extent {
                    first_block_index: self.first_block_index,
                    block_count,
                },
                Some(Extent {
                    first_block_index: self.first_block_index + block_count,
                    block_count: self.block_count - block_count,
                }),
            )
        } else if block_count == self.block_count {
            (self, None)
        } else {
            panic!("block_count must be less than or equal to extent block_count");
        }
    }

    pub fn merge(self, other: Self) -> Option<Self> {
        if self.first_block_index < other.first_block_index {
            if self.first_block_index + self.block_count == other.first_block_index {
                Some(Extent {
                    first_block_index: self.first_block_index,
                    block_count: self.block_count + other.block_count,
                })
            } else {
                None
            }
        } else {
            other.merge(self)
        }
    }

    pub fn overlaps(self, other: Self) -> bool {
        (self.first_block_index <= other.first_block_index
            && self.end_block_index() > other.first_block_index)
            || (self.first_block_index > other.first_block_index
                && self.first_block_index < other.end_block_index())
    }
}
