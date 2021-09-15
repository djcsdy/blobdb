use crate::lib::block::block::MANY_PACKETS_MAX_SIZE;
use crate::lib::block::block_digest::BLOCK_DIGEST_SIZE;
use crate::lib::block::packet_provider::{PacketProvider, PacketResult};
use crate::lib::block::signature::{BlockArity, BlockSignature, BLOCK_SIGNATURE_SIZE};
use crate::lib::block::ONE_PACKET_MAX_SIZE;
use crate::lib::db::DbId;
use crate::lib::packet::MIN_PACKET_SIZE;
use byteorder::WriteBytesExt;
use std::io::{BufWriter, Result, Seek, SeekFrom, Write};

pub fn build_write_block<W: Write + Seek>(
    writer: &mut BufWriter<W>,
    db_id: &DbId,
    packet_provider: &mut dyn PacketProvider<&mut BufWriter<W>>,
) -> Result<BuildWriteBlockResult> {
    let mut packet = packet_provider.next_packet(ONE_PACKET_MAX_SIZE as u16)?;

    let one_packet = match &packet {
        PacketResult::WritePacket(write_packet) => {
            write_packet.size() == ONE_PACKET_MAX_SIZE as u16
        }
        PacketResult::PacketTooBig => panic!(),
        PacketResult::End => return Ok(BuildWriteBlockResult::Done),
    };

    let signature_stream_position = writer.stream_position()?;
    writer.write_all(&[0; BLOCK_SIGNATURE_SIZE])?;
    writer.write_all(db_id.as_ref())?;
    let digest_stream_position = writer.stream_position()?;
    writer.write_all(&[0; BLOCK_DIGEST_SIZE])?;
    let packet_count_stream_position = writer.stream_position()?;

    if !one_packet {
        writer.write_u8(0)?;
    }

    let mut packets_count: u8 = 0;
    let mut bytes_remaining: u16 = MANY_PACKETS_MAX_SIZE as u16;

    while bytes_remaining > 0 && packets_count < u8::MAX {
        let write_packet = match packet {
            PacketResult::WritePacket(write_packet) => write_packet,
            _ => break,
        };
        let expected_packet_size = write_packet.size();
        let packet_start_stream_position = writer.stream_position()?;
        write_packet.write(writer)?;
        let actual_packet_size = writer.stream_position()? - packet_start_stream_position;
        if actual_packet_size != expected_packet_size as u64 {
            panic!();
        }

        packets_count += 1;
        bytes_remaining -= expected_packet_size;

        if bytes_remaining as usize > MIN_PACKET_SIZE {
            packet = packet_provider.next_packet(bytes_remaining)?;
        } else {
            packet = PacketResult::PacketTooBig;
        }
    }

    writer.write_all(vec![0 as u8; bytes_remaining as usize].as_slice())?;

    writer.seek(SeekFrom::Start(signature_stream_position))?;
    writer.write_all(
        BlockSignature::new(if one_packet {
            BlockArity::OnePacket
        } else {
            BlockArity::ManyPackets
        })
        .as_ref(),
    )?;

    writer.seek(SeekFrom::Start(digest_stream_position))?;
    todo!("Write digest");

    if !one_packet {
        writer.seek(SeekFrom::Start(packet_count_stream_position))?;
        writer.write_u8(packets_count)?;
    }

    Ok(match &packet {
        PacketResult::WritePacket(_) => panic!(),
        PacketResult::PacketTooBig => BuildWriteBlockResult::MoreBlocksRequired,
        PacketResult::End => BuildWriteBlockResult::Done,
    })
}

pub enum BuildWriteBlockResult {
    MoreBlocksRequired,
    Done,
}
