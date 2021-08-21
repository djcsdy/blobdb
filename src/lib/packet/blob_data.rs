use crate::lib::blob_id::BlobId;
use crate::lib::packet::Packet::BlobData;
use crate::lib::packet::RawPacket;
use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};
use std::io::{ErrorKind, Read, Result};

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

    fn new_anonymous(offset: u64, data: Vec<u8>) -> BlobDataPacket {
        BlobDataPacket::new(BlobId([0; 32]), offset, data)
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

    pub fn import_blob<R: Read>(reader: R) -> ImportBlob<R> {
        ImportBlob {
            reader,
            offset: 0,
            end: false,
        }
    }
}

pub struct ImportBlob<R: Read> {
    reader: R,
    offset: u64,
    end: bool,
}

impl<R: Read> Iterator for ImportBlob<R> {
    type Item = Result<BlobDataPacket>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        let offset = self.offset;
        let mut buf = [0 as u8; 4004];
        let mut pos = 0;
        loop {
            match self.reader.read(&mut buf[pos..]) {
                Ok(0) => break,
                Ok(count) => pos += count,
                Err(error) => match error.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => {
                        self.end = true;
                        return Some(Err(error));
                    }
                },
            }
        }

        self.offset += pos as u64;

        if pos == 0 {
            self.end = true;
            return None;
        }

        Some(Ok(BlobDataPacket::new_anonymous(offset, buf.into())))
    }
}
