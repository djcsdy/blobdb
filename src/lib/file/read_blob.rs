use crate::lib::block::{Block, Packets};
use crate::lib::packet::Packet;
use crate::lib::{BlobId, DbId};
use itertools::Itertools;
use std::cmp::min;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::{Error, ErrorKind, Read, Result, Write};

pub struct ReadBlob<R: Read> {
    db_id: DbId,
    blob_id: BlobId,
    reader: R,
    packets: Option<Packets>,
    packet: VecDeque<u8>,
    offset: u64,
}

pub fn read_blob<R: Read>(db_id: DbId, blob_id: BlobId, reader: R) -> ReadBlob<R> {
    ReadBlob {
        db_id,
        blob_id,
        reader,
        packets: None,
        packet: VecDeque::new(),
        offset: 0,
    }
}

impl<R: Read> Read for ReadBlob<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_from_packet(buf)
    }
}

impl<R: Read> ReadBlob<R> {
    fn read_from_packet(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }

        if self.packet.len() == 0 {
            match self.next_blob_data() {
                Ok(packet) => self.packet = packet,
                Err(error) => {
                    self.packet = VecDeque::new();
                    return Err(error);
                }
            }
        }

        let length = buf.write(
            self.packet
                .drain(0..min(buf.len(), self.packet.len()))
                .collect_vec()
                .as_slice(),
        )?;
        self.offset += u64::try_from(length).unwrap();
        Ok(length)
    }

    fn next_blob_data(&mut self) -> Result<VecDeque<u8>> {
        loop {
            if let Packet::BlobData(packet) = self.next_packet()? {
                if packet.blob_id() == self.blob_id {
                    return if packet.offset() == self.offset {
                        Ok(VecDeque::from(packet.data().to_vec()))
                    } else {
                        Err(Error::from(ErrorKind::InvalidData))
                    };
                }
            }
        }
    }

    fn next_packet(&mut self) -> Result<Packet> {
        loop {
            if let Some(packet) = self.packets.as_mut().and_then(|packets| packets.next()) {
                return Ok(packet);
            }
            self.packets = Some(self.next_packets()?);
        }
    }

    fn next_packets(&mut self) -> Result<Packets> {
        self.read_block().map(|block| block.into_packets())
    }

    fn read_block(&mut self) -> Result<Block> {
        loop {
            let block = Block::read(&mut self.reader)?;
            if !block.valid() {
                return Err(Error::from(ErrorKind::InvalidData));
            } else if block.db_id() == self.db_id {
                return Ok(block);
            }
        }
    }
}
