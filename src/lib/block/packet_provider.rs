use std::io::{Result, Seek, Write};

pub trait PacketProvider<W>
where
    W: Write,
    W: Seek,
{
    fn write_next_packet(&mut self, max_size: usize) -> Result<PacketProviderResult>;
}

pub enum PacketProviderResult {
    Next,
    TooBig,
    End,
}
