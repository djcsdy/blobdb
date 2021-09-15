use crate::lib::packet::raw::MAX_PAYLOAD_SIZE;

pub const INVALID_PACKET_TYPE_ID: u8 = 0;
pub const BLOB_DATA_PACKET_TYPE_ID: u8 = 1;

pub fn combine_type_id_and_length(type_id: u8, length: usize) -> u16 {
    if length > MAX_PAYLOAD_SIZE {
        panic!();
    }

    ((type_id as u16) << 12) | ((length as u16) & 0xfff)
}

pub fn extract_type_id(type_id_and_length: u16) -> u8 {
    (type_id_and_length >> 12) as u8
}

pub fn extract_length(type_id_and_length: u16) -> u16 {
    type_id_and_length & 0xfff
}
