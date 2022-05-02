use hex::FromHexError;

pub const BLOB_ID_SIZE: usize = 32;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlobId(pub [u8; BLOB_ID_SIZE]);

impl BlobId {
    pub fn anonymous() -> BlobId {
        BlobId([0; BLOB_ID_SIZE])
    }

    pub fn parse<T: AsRef<[u8]>>(text: T) -> Result<BlobId, FromHexError> {
        let mut buf = [0; BLOB_ID_SIZE];
        hex::decode_to_slice(text, &mut buf)?;
        Ok(BlobId(buf))
    }

    pub fn to_string(&self) -> String {
        hex::encode(self.0)
    }
}

impl AsRef<[u8]> for BlobId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for BlobId {
    fn as_ref(&self) -> &[u8; BLOB_ID_SIZE] {
        &self.0
    }
}
