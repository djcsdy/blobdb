mod signature;

use crate::lib::blob_id::BlobId;
use crate::lib::db_id::DbId;
use crate::lib::packet::{BlobDataPacket, ImportBlobDataPackets, Packet, RawPacket};
use byteorder::ReadBytesExt;
use itertools::Itertools;
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::io::{self, Error, ErrorKind, Read, Result};
use Iterator;

pub struct Block {
    pub db_id: DbId,
    pub packets: Vec<RawPacket>,
}

impl Block {
    pub fn new<I, P>(db_id: DbId, packets: I) -> Block
    where
        P: Into<Packet>,
        I: IntoIterator<Item = P>,
    {
        Block {
            db_id,
            packets: packets
                .into_iter()
                .map(P::into)
                .map(RawPacket::from)
                .collect(),
        }
    }

    pub fn read<R>(reader: &mut R) -> io::Result<Block>
    where
        R: Read,
    {
        let mut signature = [0 as u8; 4];
        reader.read_exact(&mut signature)?;

        if &signature[0..1] != "bD".as_bytes() || (signature[2] & 0x7f) != b'B' || signature[3] != 0
        {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        let mut db_id = [0 as u8; 16];
        reader.read_exact(&mut db_id)?;

        let packet_count = if signature[2] & 0x80 == 0x80 {
            reader.read_u8()?
        } else {
            1
        };

        let mut hash = [0 as u8; 32];
        reader.read_exact(&mut hash)?;

        let packets = (0..packet_count)
            .map(|_| RawPacket::read(reader))
            .try_collect()?;

        let block = Block {
            db_id: DbId(db_id),
            packets,
        };

        if block.digest() == hash {
            Ok(block)
        } else {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }

    fn digest(&self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(&self.db_id);
        digest.update(&[u8::try_from(self.packets.len()).unwrap()]);
        for packet in &self.packets {
            digest.update(packet);
        }
        digest.finalize().into()
    }

    pub fn import_blob<R: Read>(db_id: DbId, reader: R) -> ImportBlobDataBlocks<R> {
        ImportBlobDataBlocks {
            db_id,
            packets: BlobDataPacket::import_blob(reader),
        }
    }
}

pub struct ImportBlobDataBlocks<R: Read> {
    db_id: DbId,
    packets: ImportBlobDataPackets<R>,
}

impl<R: Read> ImportBlobDataBlocks<R> {
    pub fn end_and_digest_id(self) -> BlobId {
        self.packets.end_and_digest_id()
    }
}

impl<R: Read> Iterator for ImportBlobDataBlocks<R> {
    type Item = Result<Block>;

    fn next(&mut self) -> Option<Self::Item> {
        self.packets
            .next()
            .map(|result| result.map(|packet| Block::new(self.db_id, [packet])))
    }
}
