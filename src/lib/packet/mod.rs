pub use blob_data::BlobDataPacket;
pub use raw::RawPacket;

mod blob_data;
mod packet;
mod raw;

pub enum Packet {
    BlobData(BlobDataPacket),
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
