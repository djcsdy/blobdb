pub const DB_ID_SIZE: usize = 16;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbId(pub [u8; DB_ID_SIZE]);

impl AsRef<[u8]> for DbId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; DB_ID_SIZE]> for DbId {
    fn as_ref(&self) -> &[u8; DB_ID_SIZE] {
        &self.0
    }
}
