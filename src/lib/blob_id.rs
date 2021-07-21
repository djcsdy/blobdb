pub struct BlobId(pub [u8; 32]);

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
