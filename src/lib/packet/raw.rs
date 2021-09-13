use std::io;
use std::io::Read;
use std::mem::size_of;

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::lib::block::ONE_PACKET_MAX_SIZE;
use crate::lib::packet::blob_data::BlobDataPacket;
use crate::lib::packet::invalid::InvalidPacket;
use crate::lib::packet::packet::Packet;

pub const MAX_PACKET_SIZE: usize = ONE_PACKET_MAX_SIZE;
const TYPE_ID_AND_LENGTH_SIZE: usize = size_of::<u16>();

const TYPE_ID_AND_LENGTH_OFFSET: usize = 0;
const TYPE_ID_AND_LENGTH_END: usize = TYPE_ID_AND_LENGTH_OFFSET + TYPE_ID_AND_LENGTH_SIZE;
const DATA_OFFSET: usize = TYPE_ID_AND_LENGTH_END;

pub struct RawPacket(pub(super) Vec<u8>);

impl RawPacket {
    pub fn read<R>(reader: &mut R) -> io::Result<RawPacket>
    where
        R: Read,
    {
        let type_id_and_length = reader.read_u16::<LittleEndian>()?;
        let length = type_id_and_length & 0xfff;

        let mut raw_bytes = Vec::with_capacity(length as usize + DATA_OFFSET);
        raw_bytes
            .write_u16::<LittleEndian>(type_id_and_length)
            .unwrap();
        reader.read_exact(&mut raw_bytes[DATA_OFFSET..])?;

        Ok(RawPacket(raw_bytes))
    }

    pub fn new_invalid() -> RawPacket {
        RawPacket(vec![0; DATA_OFFSET])
    }

    fn type_id_and_length(&self) -> u16 {
        LittleEndian::read_u16(&self.0[TYPE_ID_AND_LENGTH_OFFSET..TYPE_ID_AND_LENGTH_END])
    }

    pub fn type_id(&self) -> u8 {
        (self.type_id_and_length() >> 12) as u8
    }

    pub fn data(&self) -> &[u8] {
        &self.0[DATA_OFFSET..]
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl AsRef<[u8]> for RawPacket {
    fn as_ref(&self) -> &[u8] {
        return &self.0;
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
