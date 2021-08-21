#[derive(Copy, Clone)]
pub struct DbId(pub [u8; 16]);

impl AsRef<[u8]> for DbId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 16]> for DbId {
    fn as_ref(&self) -> &[u8; 16] {
        &self.0
    }
}
