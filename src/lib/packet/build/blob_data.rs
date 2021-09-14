use std::io;
use std::io::{Cursor, Seek, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::lib::blob::BlobId;
use crate::lib::packet::blob_data::{BLOB_DATA_OFFSET, BLOB_DATA_PACKET_TYPE_ID, MAX_DATA_SIZE};
use crate::lib::packet::build::raw::build_write_raw_packet;
use crate::lib::packet::build::write_blob_id::WriteBlobId;

pub fn build_write_blob_data<W>(writer: W, packet: DraftBlobDataPacket) -> io::Result<WriteBlobId>
where
    W: Write,
    W: Seek,
{
    if packet.data.len() > MAX_DATA_SIZE {
        panic!();
    }
    build_write_raw_packet(writer, BLOB_DATA_PACKET_TYPE_ID, |writer| {
        let blob_id_position = writer.stream_position()?;
        writer.write_all(BlobId::anonymous().as_ref())?;
        writer.write_u64::<LittleEndian>(packet.offset)?;
        writer.write_all(&packet.data)?;
        Ok(WriteBlobId::new(blob_id_position))
    })
}

pub fn build_blob_data(blob_id: BlobId, packet: DraftBlobDataPacket) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(BLOB_DATA_OFFSET + packet.data.len());
    let mut cursor = Cursor::new(&mut buffer);
    build_write_blob_data(&mut cursor, packet)
        .unwrap()
        .write_blob_id(&mut cursor, blob_id)
        .unwrap();
    buffer
}

pub struct DraftBlobDataPacket {
    pub offset: u64,
    pub data: Vec<u8>,
}
