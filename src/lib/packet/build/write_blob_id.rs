use std::io::{Result, Seek, SeekFrom, Write};

use crate::lib::blob::BlobId;

pub struct WriteBlobId {
    stream_position: u64,
}

impl WriteBlobId {
    pub(super) fn new(stream_position: u64) -> WriteBlobId {
        WriteBlobId { stream_position }
    }

    pub fn write_blob_id<W>(self, mut writer: W, id: BlobId) -> Result<()>
    where
        W: Write,
        W: Seek,
    {
        let pos = writer.stream_position()?;
        writer.seek(SeekFrom::Start(self.stream_position))?;
        writer.write_all(id.as_ref())?;
        writer.seek(SeekFrom::Start(pos))?;
        Ok(())
    }
}
