use std::io::Read;

use crate::lib::block::block::{Block, MANY_PACKETS_MAX_SIZE};
use crate::lib::block::blockifier::{Blockified, Blockifier, BlockifierPostUpdater};
use crate::lib::block::ONE_PACKET_MAX_SIZE;
use crate::lib::db::DbId;
use crate::lib::packet::{
    BlobDataPacket, ImportBlobDataPackets, ImportBlobDataPacketsPostUpdater, Packetized,
    Packetizer, PacketizerPostUpdater, MIN_PACKET_SIZE,
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

impl<R: Read> Blockifier<(), ImportBlobDataPostUpdater> for ImportBlobDataBlocks<R> {
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
                post_update: (),
            }
        }
    }

    fn into_post_updater(self) -> ImportBlobDataPostUpdater {
        ImportBlobDataPostUpdater {
            packet_post_updater: self.packetizer.into_post_updater(),
        }
    }
}

pub struct ImportBlobDataPostUpdater {
    packet_post_updater: ImportBlobDataPacketsPostUpdater,
}

impl BlockifierPostUpdater<()> for ImportBlobDataPostUpdater {
    fn apply_post_update(&mut self, block: Block, _: ()) -> Block {
        Block::new(
            block.db_id(),
            block
                .into_packets()
                .map(|packet| self.packet_post_updater.apply_post_update(packet, ()))
                .collect(),
        )
    }
}
