use std::convert::TryFrom;
use std::io;
use std::io::{Read, Write};

use arrayref::array_ref;

use crate::lib::block::block_digest::BlockDigest;
use crate::lib::block::packets::Packets;
use crate::lib::block::signature::{BlockArity, BlockSignature, BLOCK_SIGNATURE_SIZE};
use crate::lib::db::DbId;
use crate::lib::packet::Packet;
use crate::lib::packet::RawPacket;

pub const BLOCK_SIZE: usize = 4096;

const SIGNATURE_OFFSET: usize = 0;
const SIGNATURE_END: usize = SIGNATURE_OFFSET + BLOCK_SIGNATURE_SIZE;

#[derive(Clone)]
pub struct Block(pub(super) [u8; BLOCK_SIZE]);

impl Block {
    pub fn new<P: Into<Packet>>(db_id: DbId, packets: Vec<P>) -> Block {
        let mut buffer = [0; BLOCK_SIZE];

        let (arity, packets_start) = match packets.len() {
            1 => (BlockArity::OnePacket, 52),
            _ => (BlockArity::ManyPackets, 53),
        };

        buffer[SIGNATURE_OFFSET..SIGNATURE_END]
            .copy_from_slice(BlockSignature::new(arity).as_ref());

        buffer[4..20].copy_from_slice(db_id.as_ref());

        if arity == BlockArity::ManyPackets {
            buffer[52] = u8::try_from(packets.len()).unwrap()
        }

        {
            let mut packets_buffer = &mut buffer[packets_start..];
            for packet in packets {
                packets_buffer
                    .write(RawPacket::from(P::into(packet)).as_ref())
                    .unwrap();
            }
        }

        let digest = BlockDigest::digest_block(&Block(buffer));
        buffer[20..52].copy_from_slice(digest.as_ref());

        Block(buffer)
    }

    pub fn read<R>(reader: &mut R) -> io::Result<Block>
    where
        R: Read,
    {
        let mut buffer = [0; BLOCK_SIZE];
        reader.read_exact(&mut buffer)?;
        Ok(Block(buffer))
    }

    fn arity(&self) -> BlockArity {
        BlockSignature(array_ref!(self.0, SIGNATURE_OFFSET, BLOCK_SIGNATURE_SIZE)).arity()
    }

    pub fn db_id(&self) -> DbId {
        DbId(*array_ref!(self.0, 4, 16))
    }

    fn digest(&self) -> BlockDigest {
        BlockDigest(array_ref!(self.0, 20, 32).clone())
    }

    pub fn into_packets(self) -> Packets {
        let (start, count) = match self.arity() {
            BlockArity::Invalid => (52, 0),
            BlockArity::OnePacket => (52, 1),
            BlockArity::ManyPackets => (53, 1),
        };
        Packets::new(self, start, count)
    }
}

impl AsRef<[u8]> for Block {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
