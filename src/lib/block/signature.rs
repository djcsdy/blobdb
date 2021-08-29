#[derive(Copy, Clone)]
pub struct BlockSignature([u8; 4]);

pub enum BlockArity {
    Invalid,
    OnePacket,
    ManyPackets,
}

impl BlockSignature {
    pub fn arity(&self) -> BlockArity {
        match &self.0 {
            b"bDB\0" => BlockArity::OnePacket,
            b"bD\xC2\0" => BlockArity::ManyPackets,
            _ => BlockArity::Invalid,
        }
    }
}
