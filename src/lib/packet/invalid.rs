use crate::lib::packet::RawPacket;

pub struct InvalidPacket(pub(super) RawPacket);

impl AsRef<[u8]> for InvalidPacket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl InvalidPacket {
    pub fn size(&self) -> usize {
        self.0.size()
    }
}
