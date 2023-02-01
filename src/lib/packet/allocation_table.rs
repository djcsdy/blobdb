use crate::lib::packet::raw::MAX_PAYLOAD_SIZE;
use crate::lib::packet::type_id_and_length::SEQUENTIAL_ALLOCATION_TABLE_PACKET_TYPE_ID;
use crate::lib::packet::RawPacket;
use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};

const OFFSET_SIZE: usize = 8;
pub const MAX_BITMAP_SIZE: usize = MAX_PAYLOAD_SIZE - BITMAP_OFFSET;

const OFFSET_OFFSET: usize = 0;
const OFFSET_END: usize = OFFSET_OFFSET + OFFSET_SIZE;
const BITMAP_OFFSET: usize = OFFSET_END;

pub struct SequentialAllocationTablePacket(pub(super) RawPacket);

impl AsRef<[u8]> for SequentialAllocationTablePacket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl SequentialAllocationTablePacket {
    pub fn new(offset: u64, bitmap: Vec<u8>) -> SequentialAllocationTablePacket {
        let mut payload_bytes = Vec::with_capacity(bitmap.len() + BITMAP_OFFSET);
        LittleEndian::write_u64(&mut payload_bytes[OFFSET_OFFSET..OFFSET_END], offset);
        payload_bytes[BITMAP_OFFSET..].copy_from_slice(&bitmap);

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
        &self.0.payload()[BITMAP_OFFSET..]
    }
}
