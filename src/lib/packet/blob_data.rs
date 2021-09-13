use crate::lib::blob::BlobId;
use crate::lib::packet::packet::MAX_PACKET_SIZE;
use crate::lib::packet::RawPacket;
use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};
use sha2::{Digest, Sha256};
use std::io::{ErrorKind, Read, Result};
use tee_readwrite::TeeReader;

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

    pub fn size(&self) -> usize {
        self.0.size()
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

    pub fn import_blob<R: Read>(reader: R) -> ImportBlobDataPackets<R> {
        ImportBlobDataPackets {
            tee_reader: TeeReader::new(reader, Sha256::new(), false),
            offset: 0,
            end: false,
        }
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ImportBlobDataPackets<R: Read> {
    tee_reader: TeeReader<R, Sha256>,
    offset: u64,
    end: bool,
}

impl<R: Read> ImportBlobDataPackets<R> {
    pub fn end_and_digest_id(self) -> BlobId {
        let (_, digest) = self.tee_reader.into_inner();
        BlobId(digest.finalize().into())
    }
}

impl<R: Read> Iterator for ImportBlobDataPackets<R> {
    type Item = Result<BlobDataPacket>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        let offset = self.offset;
        let mut buf = [0 as u8; MAX_PACKET_SIZE];
        let mut pos = 0;
        loop {
            match self.tee_reader.read(&mut buf[pos..]) {
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
