use crate::lib::blob_id::BlobId;
use crate::lib::block::Block;
use crate::lib::db_id::DbId;
use crate::lib::packet::BlobDataPacket;
use std::io::{Error, ErrorKind, Read, Result};

// TODO -> Iterator<Result<Block>>
pub fn read_into_block<R>(reader: &mut R) -> Result<Block>
where
    R: Read,
{
    let mut buf = [0 as u8; 4004];
    let mut pos = 0;
    loop {
        match reader.read(&mut buf[pos..]) {
            Ok(0) => break,
            Ok(count) => pos += count,
            Err(error) => match error.kind() {
                ErrorKind::Interrupted => continue,
                _ => return Err(error),
            },
        }
    }

    Ok(Block::new(
        DbId(todo!()),
        [BlobDataPacket::new(BlobId(todo!()), todo!(), buf.into())],
    ))
}
