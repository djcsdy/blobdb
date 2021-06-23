use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::convert::TryFrom;
use std::io::{self, Error, ErrorKind, Read};

pub enum Packet {
    BlobData(BlobDataPacket),
}

pub struct BlobDataPacket(RawPacket);

pub struct RawPacket(Vec<u8>);

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

impl From<RawPacket> for Packet {
    fn from(raw: RawPacket) -> Self {
        match raw.0[1] >> 4 {
            1 => Packet::BlobData(BlobDataPacket(raw)),
            _ => panic!(),
        }
    }
}

impl From<Packet> for RawPacket {
    fn from(packet: Packet) -> Self {
        match packet {
            Packet::BlobData(BlobDataPacket(raw_packet)) => raw_packet,
        }
    }
}
