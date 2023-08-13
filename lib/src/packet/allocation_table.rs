use crate::packet::raw::MAX_PAYLOAD_SIZE;
use crate::packet::type_id_and_length::{
    RANDOM_ALLOCATION_TABLE_PACKET_TYPE_ID, SEQUENTIAL_ALLOCATION_TABLE_PACKET_TYPE_ID,
};
use crate::packet::RawPacket;
use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};
use std::mem::size_of;

const OFFSET_SIZE: usize = size_of::<u64>();
pub const NEXT_BLOCK_INDEX_SIZE: usize = size_of::<u64>();
pub const MAX_SEQUENTIAL_BITMAP_SIZE: usize = MAX_PAYLOAD_SIZE - SEQUENTIAL_BITMAP_OFFSET;
pub const MAX_RANDOM_BITMAP_SIZE: usize = MAX_PAYLOAD_SIZE - RANDOM_BITMAP_OFFSET;

const OFFSET_OFFSET: usize = 0;
const OFFSET_END: usize = OFFSET_OFFSET + OFFSET_SIZE;
const NEXT_BLOCK_INDEX_OFFSET: usize = OFFSET_END;
const NEXT_BLOCK_INDEX_END: usize = NEXT_BLOCK_INDEX_OFFSET + NEXT_BLOCK_INDEX_SIZE;
const SEQUENTIAL_BITMAP_OFFSET: usize = OFFSET_END;
const RANDOM_BITMAP_OFFSET: usize = NEXT_BLOCK_INDEX_END;

pub struct SequentialAllocationTablePacket(pub(super) RawPacket);

impl AsRef<[u8]> for SequentialAllocationTablePacket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl SequentialAllocationTablePacket {
    pub fn new(offset: u64, bitmap: Vec<u8>) -> SequentialAllocationTablePacket {
        let mut payload_bytes = Vec::with_capacity(bitmap.len() + SEQUENTIAL_BITMAP_OFFSET);
        LittleEndian::write_u64(&mut payload_bytes[OFFSET_OFFSET..OFFSET_END], offset);
        payload_bytes[SEQUENTIAL_BITMAP_OFFSET..].copy_from_slice(&bitmap);

        SequentialAllocationTablePacket(RawPacket::new(
            SEQUENTIAL_ALLOCATION_TABLE_PACKET_TYPE_ID,
            &payload_bytes,
        ))
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    pub fn offset(&self) -> u64 {
        LittleEndian::read_u64(array_ref!(self.0.payload(), OFFSET_OFFSET, OFFSET_SIZE))
    }

    pub fn bitmap(&self) -> &[u8] {
        &self.0.payload()[SEQUENTIAL_BITMAP_OFFSET..]
    }
}

pub struct RandomAllocationTablePacket(pub(super) RawPacket);

impl AsRef<[u8]> for RandomAllocationTablePacket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl RandomAllocationTablePacket {
    pub fn new(offset: u64, bitmap: Vec<u8>, next_block_index: u64) -> RandomAllocationTablePacket {
        let mut payload_bytes =
            Vec::with_capacity(bitmap.len() + SEQUENTIAL_BITMAP_OFFSET + NEXT_BLOCK_INDEX_SIZE);
        LittleEndian::write_u64(&mut payload_bytes[OFFSET_OFFSET..OFFSET_END], offset);
        LittleEndian::write_u64(
            &mut payload_bytes[NEXT_BLOCK_INDEX_OFFSET..NEXT_BLOCK_INDEX_END],
            next_block_index,
        );
        payload_bytes[RANDOM_BITMAP_OFFSET..].copy_from_slice(&bitmap);

        RandomAllocationTablePacket(RawPacket::new(
            RANDOM_ALLOCATION_TABLE_PACKET_TYPE_ID,
            &payload_bytes,
        ))
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    pub fn offset(&self) -> u64 {
        LittleEndian::read_u64(array_ref!(self.0.payload(), OFFSET_OFFSET, OFFSET_END))
    }

    pub fn next_block_index(&self) -> u64 {
        LittleEndian::read_u64(array_ref!(
            self.0.payload(),
            NEXT_BLOCK_INDEX_OFFSET,
            NEXT_BLOCK_INDEX_END
        ))
    }

    pub fn bitmap(&self) -> &[u8] {
        &self.0.payload()[RANDOM_BITMAP_OFFSET..]
    }
}
