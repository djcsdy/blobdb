use crate::db_id::DbId;
use crate::packet::RawPacket;
use byteorder::ReadBytesExt;
use itertools::Itertools;
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::io::{self, Error, ErrorKind, Read};

pub struct Block {
    pub db_id: DbId,
    pub packets: Vec<RawPacket>,
}

impl Block {
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
}
