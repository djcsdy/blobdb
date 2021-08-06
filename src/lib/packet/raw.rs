use std::io;
use std::io::{Error, ErrorKind, Read};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::lib::packet::blob_data::BlobDataPacket;
use crate::lib::packet::Packet;

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

        let mut raw_bytes = Vec::with_capacity(length as usize + 2);
        raw_bytes
            .write_u16::<LittleEndian>(type_id_and_length)
            .unwrap();
        reader.read_exact(&mut raw_bytes[2..])?;

        Ok(RawPacket(raw_bytes))
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
        }
    }
}
