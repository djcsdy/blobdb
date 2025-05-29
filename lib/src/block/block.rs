use std::convert::TryFrom;
use std::io;
use std::io::{Read, Write};

use arrayref::array_ref;

use crate::block::block_digest::{BLOCK_DIGEST_SIZE, BlockDigest};
use crate::block::packets::Packets;
use crate::block::signature::{BLOCK_SIGNATURE_SIZE, BlockArity, BlockSignature};
use crate::db::{DB_ID_SIZE, DbId};
use crate::packet::Packet;
use crate::packet::RawPacket;
use std::mem::size_of;

const PACKET_COUNT_SIZE: usize = size_of::<u8>();
pub const ONE_PACKET_MAX_SIZE: usize = Block::SIZE - ONE_PACKET_OFFSET;
pub const MANY_PACKETS_MAX_SIZE: usize = Block::SIZE - MANY_PACKETS_OFFSET;

const SIGNATURE_OFFSET: usize = 0;
const SIGNATURE_END: usize = SIGNATURE_OFFSET + BLOCK_SIGNATURE_SIZE;
const DB_ID_OFFSET: usize = SIGNATURE_END;
const DB_ID_END: usize = DB_ID_OFFSET + DB_ID_SIZE;
const DIGEST_OFFSET: usize = DB_ID_END;
const DIGEST_END: usize = DIGEST_OFFSET + BLOCK_DIGEST_SIZE;
const PACKET_COUNT_OFFSET: usize = DIGEST_END;
const ONE_PACKET_OFFSET: usize = PACKET_COUNT_OFFSET;
const MANY_PACKETS_OFFSET: usize = PACKET_COUNT_OFFSET + PACKET_COUNT_SIZE;

#[derive(Clone)]
pub struct Block(pub(super) [u8; Block::SIZE]);

impl Block {
    pub const SIZE: usize = 4096;

    pub fn new<P: Into<Packet>>(db_id: DbId, packets: Vec<P>) -> Block {
        let mut buffer = [0; Block::SIZE];

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
                    .write_all(RawPacket::from(P::into(packet)).as_ref())
                    .unwrap();
            }
        }

        let digest = BlockDigest::digest_block(&Block(buffer));
        buffer[DIGEST_OFFSET..DIGEST_END].copy_from_slice(digest.as_ref());

        Block(buffer)
    }

    pub fn read<R: Read>(reader: &mut R) -> io::Result<Block> {
        let mut buffer = [0; Block::SIZE];
        reader.read_exact(&mut buffer)?;
        Ok(Block(buffer))
    }

    fn arity(&self) -> BlockArity {
        BlockSignature(array_ref!(self.0, SIGNATURE_OFFSET, BLOCK_SIGNATURE_SIZE)).arity()
    }

    pub fn db_id(&self) -> DbId {
        DbId(*array_ref!(self.0, DB_ID_OFFSET, DB_ID_SIZE))
    }

    pub fn valid(&self) -> bool {
        self.digest() == BlockDigest::digest_block(self)
    }

    fn digest(&self) -> BlockDigest {
        BlockDigest(*array_ref!(self.0, DIGEST_OFFSET, BLOCK_DIGEST_SIZE))
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
