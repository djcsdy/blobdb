use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read};

pub struct Packet {
    pub raw_bytes: Vec<u8>,
}

impl Packet {
    pub fn read<R>(reader: &mut R) -> io::Result<Packet>
    where
        R: Read,
    {
        let type_id_and_length = reader.read_u16::<LittleEndian>()?;
        let length = type_id_and_length & 0xfff;

        let mut raw_bytes = Vec::with_capacity(length as usize + 2);
        raw_bytes
            .write_u16::<LittleEndian>(type_id_and_length)
            .unwrap();
        reader.read_exact(&mut raw_bytes[2..])?;

        Ok(Packet { raw_bytes })
    }
}

impl AsRef<[u8]> for Packet {
    fn as_ref(&self) -> &[u8] {
        return &self.raw_bytes;
    }
}
