use std::path::PathBuf;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlobId(pub [u8; 32]);

impl BlobId {
    pub fn name(&self) -> PathBuf {
        hex::encode(self.0).into()
    }
}

impl AsRef<[u8]> for BlobId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for BlobId {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}
