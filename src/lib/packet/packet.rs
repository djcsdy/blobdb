use crate::lib::packet::{BlobDataPacket, RawPacket};

pub enum Packet {
    BlobData(BlobDataPacket),
}

impl Packet {
    pub fn size(&self) -> usize {
        match self {
            Packet::BlobData(packet) => packet.size(),
        }
    }
}

impl AsRef<[u8]> for Packet {
    fn as_ref(&self) -> &[u8] {
        match self {
            Packet::BlobData(packet) => packet.as_ref(),
        }
    }
}

impl From<RawPacket> for Packet {
    fn from(raw: RawPacket) -> Self {
        match raw.as_ref()[1] >> 4 {
            1 => Packet::BlobData(BlobDataPacket(raw)),
            _ => panic!(),
        }
    }
}

impl From<BlobDataPacket> for Packet {
    fn from(packet: BlobDataPacket) -> Self {
        Packet::BlobData(packet)
    }
}
