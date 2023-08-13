use crate::file::ReadBlob;
use crate::BlobId;
use std::io;
use std::io::Read;

pub trait Db<Blob: Read> {
    fn import_blob<R>(&self, reader: &mut R) -> io::Result<BlobId>
    where
        R: Read;

    fn read_blob(&self, blob_id: BlobId) -> io::Result<ReadBlob<Blob>>;
}
