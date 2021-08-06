use crate::lib::blob_id::BlobId;
use crate::lib::packet::RawPacket;
use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};

pub struct BlobDataPacket(pub(super) RawPacket);

impl AsRef<[u8]> for BlobDataPacket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl BlobDataPacket {
    pub fn new(blob_id: BlobId, offset: u64, data: Vec<u8>) -> BlobDataPacket {
        let mut raw_bytes = Vec::with_capacity(data.len() + 40);
        raw_bytes[0..32].copy_from_slice(&blob_id.0);
        LittleEndian::write_u64(&mut raw_bytes[32..40], offset);
        raw_bytes[40..].copy_from_slice(&data);

        BlobDataPacket(RawPacket(raw_bytes))
    }

    pub fn blob_id(&self) -> BlobId {
        BlobId(array_ref!(self.as_ref(), 0, 32).clone())
    }

    pub fn offset(&self) -> u64 {
        LittleEndian::read_u64(array_ref!(self.as_ref(), 32, 8))
    }

    pub fn data(&self) -> &[u8] {
        &self.as_ref()[40..]
    }
}
