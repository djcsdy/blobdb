use crate::lib::blob::BlobId;

pub enum FileId {
    Anonymous,
    Blob(BlobId),
}
