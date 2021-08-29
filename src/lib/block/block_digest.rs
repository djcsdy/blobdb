#[derive(Copy, Clone)]
pub struct BlockDigest<'a>(pub &'a [u8; 32]);
