use crate::lib::blob::BlobId;
use crate::lib::block::{Block, Blockified, Blockifier, BlockifierFinalizer};
use crate::lib::db::DbId;
use crate::lib::file::path::{blob_file_path, temp_dir_path};
use std::io::{Read, Result, Seek, SeekFrom, Write};
use std::path::Path;
use tempfile::{NamedTempFile, PersistError};

pub fn import_blob<R: Read>(db_id: DbId, base_path: &Path, reader: R) -> Result<BlobId> {
    let mut file = NamedTempFile::new_in(temp_dir_path(base_path))?;
    let mut blockifier = Block::import_blob(db_id, reader);
    let mut block_count: u64 = 0;

    while let Blockified::Block { block, .. } = blockifier.next_block() {
        if block_count >= u64::MAX / (Block::SIZE as u64) {
            panic!();
        }

        file.write_all(block.as_ref())?;
        block_count += 1;
    }

    file.seek(SeekFrom::Start(0))?;

    let mut finalizer = blockifier.into_finalizer();
    for _ in 0..block_count {
        let block_offset = file.stream_position()?;
        let updated_block = finalizer.finalize(Block::read(&mut file)?, ());
        file.seek(SeekFrom::Start(block_offset))?;
        file.write_all(updated_block.as_ref())?;
    }

    let blob_id = finalizer.blob_id();

    file.persist_noclobber(blob_file_path(base_path, blob_id))
        .map_err(|PersistError { error, .. }| error)?;

    Ok(blob_id)
}
