use std::convert::TryFrom;
use std::io;
use std::io::{Read, Write};

use arrayref::array_ref;

use crate::lib::block::block_digest::{BlockDigest, BLOCK_DIGEST_SIZE};
use crate::lib::block::packets::Packets;
use crate::lib::block::signature::{BlockArity, BlockSignature, BLOCK_SIGNATURE_SIZE};
use crate::lib::db::{DbId, DB_ID_SIZE};
use crate::lib::packet::Packet;
use crate::lib::packet::RawPacket;

pub const BLOCK_SIZE: usize = 4096;
pub const ONE_PACKET_MAX_SIZE: usize = BLOCK_SIZE - ONE_PACKET_OFFSET;
pub const MANY_PACKETS_MAX_SIZE: usize = BLOCK_SIZE - MANY_PACKETS_OFFSET;

const SIGNATURE_OFFSET: usize = 0;
const SIGNATURE_END: usize = SIGNATURE_OFFSET + BLOCK_SIGNATURE_SIZE;
const DB_ID_OFFSET: usize = SIGNATURE_END;
const DB_ID_END: usize = DB_ID_OFFSET + DB_ID_SIZE;
const DIGEST_OFFSET: usize = DB_ID_END;
const DIGEST_END: usize = DIGEST_OFFSET + BLOCK_DIGEST_SIZE;
const PACKET_COUNT_OFFSET: usize = DIGEST_END;
const ONE_PACKET_OFFSET: usize = PACKET_COUNT_OFFSET;
const MANY_PACKETS_OFFSET: usize = PACKET_COUNT_OFFSET + 1;

#[derive(Clone)]
pub struct Block(pub(super) [u8; BLOCK_SIZE]);

impl Block {
    pub fn new<P: Into<Packet>>(db_id: DbId, packets: Vec<P>) -> Block {
        let mut buffer = [0; BLOCK_SIZE];

        let (arity, packets_offset) = match packets.len() {
            1 => (BlockArity::OnePacket, ONE_PACKET_OFFSET),
            _ => (BlockArity::ManyPackets, MANY_PACKETS_OFFSET),
        };

        buffer[SIGNATURE_OFFSET..SIGNATURE_END]
            .copy_from_slice(BlockSignature::new(arity).as_ref());

        buffer[DB_ID_OFFSET..DB_ID_END].copy_from_slice(db_id.as_ref());

        if arity == BlockArity::ManyPackets {
            buffer[PACKET_COUNT_OFFSET] = u8::try_from(packets.len()).unwrap()
        }

        {
            let mut packets_buffer = &mut buffer[packets_offset..];
            for packet in packets {
                packets_buffer
                    .write(RawPacket::from(P::into(packet)).as_ref())
                    .unwrap();
            }
        }

        let digest = BlockDigest::digest_block(&Block(buffer));
        buffer[DIGEST_OFFSET..DIGEST_END].copy_from_slice(digest.as_ref());

        Block(buffer)
    }

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Block> {
        let mut buffer = [0; BLOCK_SIZE];
        reader.read_exact(&mut buffer)?;
        Ok(Block(buffer))
    }

    fn arity(&self) -> BlockArity {
        BlockSignature(array_ref!(self.0, SIGNATURE_OFFSET, BLOCK_SIGNATURE_SIZE)).arity()
    }

    pub fn db_id(&self) -> DbId {
        DbId(*array_ref!(self.0, DB_ID_OFFSET, DB_ID_SIZE))
    }

    fn digest(&self) -> BlockDigest {
        BlockDigest(array_ref!(self.0, DIGEST_OFFSET, BLOCK_DIGEST_SIZE).clone())
    }

    pub(super) fn packet_count(&self) -> u8 {
        match self.arity() {
            BlockArity::Invalid => 0,
            BlockArity::OnePacket => 1,
            BlockArity::ManyPackets => self.0[PACKET_COUNT_OFFSET],
        }
    }

    pub(super) fn raw_packets_bytes(&self) -> &[u8] {
        match self.arity() {
            BlockArity::Invalid => &[],
            BlockArity::OnePacket => array_ref!(self.0, ONE_PACKET_OFFSET, ONE_PACKET_MAX_SIZE),
            BlockArity::ManyPackets => {
                array_ref!(self.0, MANY_PACKETS_OFFSET, MANY_PACKETS_MAX_SIZE)
            }
        }
    }

    pub fn into_packets(self) -> Packets {
        Packets::new(self)
    }
}

impl AsRef<[u8]> for Block {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
