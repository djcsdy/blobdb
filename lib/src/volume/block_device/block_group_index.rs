use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct BlockGroupIndex(pub u64);

impl Add for BlockGroupIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for BlockGroupIndex {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for BlockGroupIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for BlockGroupIndex {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl From<u64> for BlockGroupIndex {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<BlockGroupIndex> for u64 {
    fn from(value: BlockGroupIndex) -> Self {
        value.0
    }
}
