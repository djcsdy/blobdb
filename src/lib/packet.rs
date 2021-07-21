use crate::lib::blob_id::BlobId;
use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
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

impl AsRef<[u8]> for Packet {
    fn as_ref(&self) -> &[u8] {
        match self {
            Packet::BlobData(packet) => packet.as_ref(),
        }
    }
}

impl AsRef<[u8]> for BlobDataPacket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
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

impl BlobDataPacket {
    pub fn blob_id(&self) -> BlobId {
        BlobId(array_ref!(self.as_ref(), 0, 32).clone())
    }

    pub fn offset(&self) -> u64 {
        LittleEndian::read_u64(array_ref!(self.as_ref(), 32, 8))
    }

    pub fn data(&self) -> &[u8] {
        &self.as_ref()[40..]
    }
}
