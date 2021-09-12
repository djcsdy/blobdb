use crate::lib::blob::BlobId;
use crate::lib::packet::BlobDataPacket;

pub enum FileBuilderCommand {
    BlobData(BlobDataPacket),
    BlobId(BlobId),
}
