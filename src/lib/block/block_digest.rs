#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlockDigest<'a>(pub &'a [u8; 32]);
