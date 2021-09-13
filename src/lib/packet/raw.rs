use std::io;
use std::io::{Error, ErrorKind, Read};
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::lib::packet::blob_data::BlobDataPacket;
use crate::lib::packet::invalid::InvalidPacket;
use crate::lib::packet::packet::Packet;

const TYPE_ID_AND_LENGTH_SIZE: usize = size_of::<u16>();

const TYPE_ID_AND_LENGTH_OFFSET: usize = 0;
const TYPE_ID_AND_LENGTH_END: usize = TYPE_ID_AND_LENGTH_OFFSET + TYPE_ID_AND_LENGTH_SIZE;
const BODY_OFFSET: usize = TYPE_ID_AND_LENGTH_END;

pub struct RawPacket(pub(super) Vec<u8>);

impl RawPacket {
    pub fn read<R>(reader: &mut R) -> io::Result<RawPacket>
    where
        R: Read,
    {
        let type_id_and_length = reader.read_u16::<LittleEndian>()?;
        let type_id = (type_id_and_length >> 12) as u8;
        let length = type_id_and_length & 0xfff;

        if type_id != 1 {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        let mut raw_bytes = Vec::with_capacity(length as usize + BODY_OFFSET);
        raw_bytes
            .write_u16::<LittleEndian>(type_id_and_length)
            .unwrap();
        reader.read_exact(&mut raw_bytes[BODY_OFFSET..])?;

        Ok(RawPacket(raw_bytes))
    }

    pub fn new_invalid() -> RawPacket {
        RawPacket(vec![])
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
