use crate::packet::invalid::InvalidPacket;
use crate::packet::type_id_and_length::BLOB_DATA_PACKET_TYPE_ID;
use crate::packet::{BlobDataPacket, RawPacket};

pub enum Packet {
    BlobData(BlobDataPacket),
    Invalid(InvalidPacket),
}

impl Packet {
    pub fn size(&self) -> usize {
        match self {
            Packet::BlobData(packet) => packet.size(),
            Packet::Invalid(packet) => packet.size(),
        }
    }
}

impl AsRef<[u8]> for Packet {
    fn as_ref(&self) -> &[u8] {
        match self {
            Packet::BlobData(packet) => packet.as_ref(),
            Packet::Invalid(packet) => packet.as_ref(),
        }
    }
}

impl From<RawPacket> for Packet {
    fn from(raw: RawPacket) -> Self {
        match raw.type_id() {
            BLOB_DATA_PACKET_TYPE_ID => Packet::BlobData(BlobDataPacket(raw)),
            _ => Packet::Invalid(InvalidPacket(raw)),
        }
    }
}

impl From<BlobDataPacket> for Packet {
    fn from(packet: BlobDataPacket) -> Self {
        Packet::BlobData(packet)
    }
}

impl From<InvalidPacket> for Packet {
    fn from(packet: InvalidPacket) -> Self {
        Packet::Invalid(packet)
    }
}
