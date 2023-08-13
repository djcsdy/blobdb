use crate::blob::BlobId;

pub enum FileId {
    Anonymous,
    Blob(BlobId),
}
