use crate::lib::block::block::Block;
use crate::lib::packet::Packet;
use crate::lib::packet::RawPacket;

pub struct Packets {
    block: Block,
    pos: usize,
    count: u8,
}

impl Packets {
    pub(super) fn new(block: Block) -> Packets {
        Packets {
            count: block.packet_count(),
            block,
            pos: 0,
        }
    }
}

impl Iterator for Packets {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            let mut buffer = &self.block.raw_packets_bytes()[self.pos..];
            let remaining = buffer.len();
            let raw_packet =
                RawPacket::read(&mut buffer).unwrap_or_else(|_| RawPacket::new_invalid());
            self.pos += buffer.len() - remaining;

            Some(Packet::from(raw_packet))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
}

impl ExactSizeIterator for Packets {
    fn len(&self) -> usize {
        self.count as usize
    }
}
