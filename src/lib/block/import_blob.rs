use std::io::Read;

use crate::lib::block::block::Block;
use crate::lib::block::blockifier::{Blockified, Blockifier};
use crate::lib::db::DbId;

impl Block {
    pub fn import_blob<R: Read>(db_id: DbId, reader: R) -> ImportBlobDataBlocks<R> {
        ImportBlobDataBlocks { db_id, reader }
    }
}

pub struct ImportBlobDataBlocks<R: Read> {
    db_id: DbId,
    reader: R,
}

impl<R: Read> Blockifier<()> for ImportBlobDataBlocks<R> {
    fn next_block(&mut self) -> Blockified<()> {
        todo!()
    }

    fn apply_post_update(&mut self, block: &mut Block, post_update: ()) {
        todo!()
    }
}
