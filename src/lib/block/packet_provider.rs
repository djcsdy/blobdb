use std::io::{Result, Seek, Write};

pub trait PacketProvider {
    fn write_next_packet<W>(size_hint: usize) -> Result<bool>
    where
        W: Write,
        W: Seek;
}
