use crate::lib::block::Block;
use sha2::{Digest, Sha256};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockDigest<'a>(pub &'a [u8; 32]);

impl BlockDigest<'_> {
    pub fn digest_block(block: &Block) -> BlockDigest {
        let mut digest = Sha256::new();
        digest.update(&block.db_id());
        digest.update(&block.0[52..]);
        BlockDigest(&digest.finalize().into())
    }
}

impl<'a> AsRef<[u8; 32]> for BlockDigest<'a> {
    fn as_ref(&self) -> &'a [u8; 32] {
        self.0
    }
}
