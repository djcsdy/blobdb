use crate::lib::packet::RawPacket;

pub struct KeyTablePacket(pub(super) RawPacket);

impl AsRef<[u8]> for KeyTablePacket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
