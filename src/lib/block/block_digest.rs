use crate::lib::block::block::Block;
use sha2::{Digest, Sha256};

pub const BLOCK_DIGEST_SIZE: usize = 32;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockDigest(pub [u8; BLOCK_DIGEST_SIZE]);

impl BlockDigest {
    pub fn digest_block(block: &Block) -> BlockDigest {
        let mut digest = Sha256::new();
        digest.update(&block.db_id());
        digest.update(&block.0[52..]);
        BlockDigest(digest.finalize().into())
    }
}

impl AsRef<[u8; BLOCK_DIGEST_SIZE]> for BlockDigest {
    fn as_ref(&self) -> &[u8; BLOCK_DIGEST_SIZE] {
        &self.0
    }
}
