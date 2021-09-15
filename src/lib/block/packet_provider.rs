use std::io::{Result, Seek, Write};

pub trait PacketProvider<W: Write + Seek> {
    fn next_packet(&mut self, max_size: u16) -> Result<PacketResult>;
}

pub enum PacketResult {
    WritePacket(WritePacket),
    PacketTooBig,
    End,
}

pub struct WritePacket {
    size: u16,
    write: Box<dyn FnOnce(&mut dyn WriteSeek) -> Result<()>>,
}

impl WritePacket {
    pub fn new(size: u16, write: Box<dyn FnOnce(&mut dyn WriteSeek) -> Result<()>>) -> WritePacket {
        WritePacket { size, write }
    }

    pub fn size(&self) -> u16 {
        self.size
    }

    pub fn write(self, writer: &mut dyn WriteSeek) -> Result<()> {
        (self.write)(writer)
    }
}

pub trait WriteSeek: Write + Seek {}

impl<W: Write + Seek> WriteSeek for W {}
