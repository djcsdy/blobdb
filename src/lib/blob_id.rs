use std::fmt::Write;
use std::path::PathBuf;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlobId(pub [u8; 32]);

impl BlobId {
    pub fn name(&self) -> PathBuf {
        let mut name = String::new();
        for byte in self.0 {
            write!(&mut name, "{:2x}", byte).unwrap();
        }
        name.into()
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
