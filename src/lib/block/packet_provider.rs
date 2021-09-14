use std::io::{Result, Seek, Write};

pub trait PacketProvider<W: Write + Seek> {
    fn next_large_packet(&mut self) -> Result<LargePacketResult>;
    fn next_packet(&mut self, max_size: usize) -> Result<PacketResult>;
}

pub enum LargePacketResult {
    LargePacket(WritePacket),
    TooSmall,
    End,
}

pub enum PacketResult {
    Packet(WritePacket),
    TooBig,
    End,
}

pub type WritePacket = Box<dyn FnOnce(&mut dyn WriteSeek) -> Result<()>>;

pub trait WriteSeek: Write + Seek {}

impl<W: Write + Seek> WriteSeek for W {}
