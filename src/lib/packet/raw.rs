use std::io;
use std::io::Read;
use std::mem::size_of;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::lib::block::ONE_PACKET_MAX_SIZE;
use crate::lib::packet::blob_data::BlobDataPacket;
use crate::lib::packet::invalid::InvalidPacket;
use crate::lib::packet::packet::Packet;
use crate::lib::packet::type_id_and_length::{
    combine_type_id_and_length, extract_length, extract_type_id,
};

pub const MIN_PACKET_SIZE: usize = PAYLOAD_OFFSET;
pub const MAX_PACKET_SIZE: usize = ONE_PACKET_MAX_SIZE;
pub const TYPE_ID_AND_LENGTH_SIZE: usize = size_of::<u16>();
pub const MAX_PAYLOAD_SIZE: usize = MAX_PACKET_SIZE - PAYLOAD_OFFSET;

pub const TYPE_ID_AND_LENGTH_OFFSET: usize = 0;
pub const TYPE_ID_AND_LENGTH_END: usize = TYPE_ID_AND_LENGTH_OFFSET + TYPE_ID_AND_LENGTH_SIZE;
pub const PAYLOAD_OFFSET: usize = TYPE_ID_AND_LENGTH_END;

pub struct RawPacket(Vec<u8>);

impl RawPacket {
    pub fn read<R: Read>(reader: &mut R) -> io::Result<RawPacket> {
        let type_id_and_length = reader.read_u16::<LittleEndian>()?;
        let length = extract_length(type_id_and_length);

        let mut raw_bytes = Vec::with_capacity(length as usize + PAYLOAD_OFFSET);
        raw_bytes
            .write_u16::<LittleEndian>(type_id_and_length)
            .unwrap();
        reader.read_exact(&mut raw_bytes[PAYLOAD_OFFSET..])?;

        Ok(RawPacket(raw_bytes))
    }

    pub(super) fn new(type_id: u8, payload: &[u8]) -> RawPacket {
        if payload.len() > MAX_PAYLOAD_SIZE {
            panic!();
        }
        let mut raw_bytes = Vec::with_capacity(payload.len() + PAYLOAD_OFFSET);
        LittleEndian::write_u16(
            &mut raw_bytes[TYPE_ID_AND_LENGTH_OFFSET..TYPE_ID_AND_LENGTH_END],
            combine_type_id_and_length(type_id, payload.len()),
        );
        raw_bytes[PAYLOAD_OFFSET..].copy_from_slice(payload);
        RawPacket(raw_bytes)
    }

    pub fn new_invalid() -> RawPacket {
        RawPacket::new(0, &[])
    }

    fn type_id_and_length(&self) -> u16 {
        LittleEndian::read_u16(&self.0[TYPE_ID_AND_LENGTH_OFFSET..TYPE_ID_AND_LENGTH_END])
    }

    pub fn type_id(&self) -> u8 {
        extract_type_id(self.type_id_and_length())
    }

    pub fn payload(&self) -> &[u8] {
        &self.0[PAYLOAD_OFFSET..]
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl AsRef<[u8]> for RawPacket {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Packet> for RawPacket {
    fn from(packet: Packet) -> Self {
        match packet {
            Packet::BlobData(BlobDataPacket(raw_packet)) => raw_packet,
            Packet::Invalid(InvalidPacket(raw_packet)) => raw_packet,
        }
    }
}
