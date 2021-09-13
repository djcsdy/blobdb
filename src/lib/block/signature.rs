pub const BLOCK_SIGNATURE_SIZE: usize = 4;

#[derive(Copy, Clone)]
pub struct BlockSignature<'a>(pub &'a [u8; BLOCK_SIGNATURE_SIZE]);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BlockArity {
    Invalid,
    OnePacket,
    ManyPackets,
}

impl BlockSignature<'_> {
    pub fn new(arity: BlockArity) -> BlockSignature<'static> {
        match arity {
            BlockArity::Invalid => BlockSignature(b"\0\0\0\0"),
            BlockArity::OnePacket => BlockSignature(b"bDB\0"),
            BlockArity::ManyPackets => BlockSignature(b"bD\xC2\0"),
        }
    }

    pub fn arity(&self) -> BlockArity {
        match self.0 {
            b"bDB\0" => BlockArity::OnePacket,
            b"bD\xC2\0" => BlockArity::ManyPackets,
            _ => BlockArity::Invalid,
        }
    }
}

impl<'a> AsRef<[u8; BLOCK_SIGNATURE_SIZE]> for BlockSignature<'a> {
    fn as_ref(&self) -> &'a [u8; BLOCK_SIGNATURE_SIZE] {
        self.0
    }
}
