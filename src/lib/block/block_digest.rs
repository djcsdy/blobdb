use crate::lib::block::Block;
use sha2::{Digest, Sha256};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockDigest(pub [u8; 32]);

impl BlockDigest {
    pub fn digest_block(block: &Block) -> BlockDigest {
        let mut digest = Sha256::new();
        digest.update(&block.db_id());
        digest.update(&block.0[52..]);
        BlockDigest(digest.finalize().into())
    }
}

impl AsRef<[u8; 32]> for BlockDigest {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}
