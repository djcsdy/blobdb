use crate::lib::block::block::{MANY_PACKETS_MAX_SIZE, PACKET_COUNT_SIZE};
use crate::lib::block::block_digest::BLOCK_DIGEST_SIZE;
use crate::lib::block::build::BuildWriteBlockResult::MoreBlocksRequired;
use crate::lib::block::packet_provider::PacketProvider;
use crate::lib::block::signature::BLOCK_SIGNATURE_SIZE;
use crate::lib::db::DbId;
use std::io::{Result, Seek, Write};

pub fn build_write_block<W>(
    mut writer: W,
    db_id: &DbId,
    packet_provider: Box<dyn PacketProvider<W>>,
) -> Result<BuildWriteBlockResult>
where
    W: Write,
    W: Seek,
{
    let signature_stream_position = writer.stream_position()?;
    writer.write_all(&[0; BLOCK_SIGNATURE_SIZE])?;
    writer.write_all(db_id.as_ref())?;
    let digest_stream_position = writer.stream_position()?;
    writer.write_all(&[0; BLOCK_DIGEST_SIZE])?;
    let packet_count_stream_position = writer.stream_position()?;
    let one_packet_stream_position = packet_count_stream_position;
    let many_packets_stream_position = packet_count_stream_position + PACKET_COUNT_SIZE as u64;

    let mut packets_count: u8 = 0;
    let mut bytes_remaining = MANY_PACKETS_MAX_SIZE;

    while bytes_remaining > 0 && packets_count < u8::MAX {
        todo!();
    }

    Ok(MoreBlocksRequired)
}

pub enum BuildWriteBlockResult {
    MoreBlocksRequired,
    Done,
}
