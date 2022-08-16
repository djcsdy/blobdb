use crate::lib::blob::BlobId;
use std::io::Read;

use crate::lib::block::block::{Block, MANY_PACKETS_MAX_SIZE};
use crate::lib::block::blockifier::{Blockified, Blockifier, BlockifierFinalizer};
use crate::lib::block::ONE_PACKET_MAX_SIZE;
use crate::lib::db::DbId;
use crate::lib::packet::{
    BlobDataPacket, ImportBlobDataPackets, ImportBlobDataPacketsFinalizer, Packetized, Packetizer,
    PacketizerFinalizer, MIN_PACKET_SIZE,
};

impl Block {
    pub fn import_blob<R: Read>(db_id: DbId, reader: R) -> ImportBlobDataBlocks<R> {
        ImportBlobDataBlocks {
            db_id,
            packetizer: BlobDataPacket::import_blob(reader),
        }
    }
}

pub struct ImportBlobDataBlocks<R: Read> {
    db_id: DbId,
    packetizer: ImportBlobDataPackets<R>,
}

impl<R: Read> Blockifier<(), ImportBlobDataFinalizer> for ImportBlobDataBlocks<R> {
    fn next_block(&mut self) -> Blockified<()> {
        let mut packets = vec![];
        let mut packetized = self.packetizer.next_packet(ONE_PACKET_MAX_SIZE as u16);

        let one_packet = match &packetized {
            Packetized::Packet { packet, .. } => packet.size() as usize == ONE_PACKET_MAX_SIZE,
            Packetized::PacketTooBig => false,
            Packetized::End => false,
        };

        let mut bytes_remaining: u16 = MANY_PACKETS_MAX_SIZE as u16;

        while bytes_remaining > 0 && packets.len() < u8::MAX as usize {
            let packet = match packetized {
                Packetized::Packet { packet, .. } => packet,
                _ => break,
            };

            if packet.size() > bytes_remaining as usize {
                panic!();
            }

            bytes_remaining -= packet.size() as u16;
            packets.push(packet);

            if !one_packet && bytes_remaining as usize > MIN_PACKET_SIZE {
                packetized = self.packetizer.next_packet(bytes_remaining);
            } else {
                packetized = Packetized::PacketTooBig
            }
        }

        if packets.is_empty() {
            match packetized {
                Packetized::End => Blockified::End,
                _ => panic!(),
            }
        } else {
            Blockified::Block {
                block: Block::new(self.db_id, packets),
                finalize_data: (),
            }
        }
    }

    fn into_finalizer(self) -> ImportBlobDataFinalizer {
        ImportBlobDataFinalizer {
            packet_finalizer: self.packetizer.into_finalizer(),
        }
    }
}

pub struct ImportBlobDataFinalizer {
    packet_finalizer: ImportBlobDataPacketsFinalizer,
}

impl ImportBlobDataFinalizer {
    pub fn blob_id(&self) -> BlobId {
        self.packet_finalizer.blob_id()
    }
}

impl BlockifierFinalizer<()> for ImportBlobDataFinalizer {
    fn finalize(&mut self, block: Block, _: ()) -> Block {
        Block::new(
            block.db_id(),
            block
                .into_packets()
                .map(|packet| self.packet_finalizer.finalize(packet, ()))
                .collect(),
        )
    }
}
