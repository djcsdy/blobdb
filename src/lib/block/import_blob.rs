use std::io;
use std::io::Read;

use crate::lib::blob_id::BlobId;
use crate::lib::block::block::Block;
use crate::lib::db::DbId;
use crate::lib::packet::{BlobDataPacket, ImportBlobDataPackets};

impl Block {
    pub fn import_blob<R: Read>(db_id: DbId, reader: R) -> ImportBlobDataBlocks<R> {
        ImportBlobDataBlocks {
            db_id,
            packets: BlobDataPacket::import_blob(reader),
        }
    }
}

pub struct ImportBlobDataBlocks<R: Read> {
    db_id: DbId,
    packets: ImportBlobDataPackets<R>,
}

impl<R: Read> ImportBlobDataBlocks<R> {
    pub fn end_and_digest_id(self) -> BlobId {
        self.packets.end_and_digest_id()
    }
}

impl<R: Read> Iterator for ImportBlobDataBlocks<R> {
    type Item = io::Result<Block>;

    fn next(&mut self) -> Option<Self::Item> {
        self.packets
            .next()
            .map(|result| result.map(|packet| Block::new(self.db_id, vec![packet])))
    }
}
