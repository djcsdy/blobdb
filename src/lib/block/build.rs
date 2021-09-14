use crate::lib::block::block::MANY_PACKETS_MAX_SIZE;
use crate::lib::block::block_digest::BLOCK_DIGEST_SIZE;
use crate::lib::block::packet_provider::{LargePacketResult, PacketProvider, PacketResult};
use crate::lib::block::signature::{BlockArity, BlockSignature};
use crate::lib::block::ONE_PACKET_MAX_SIZE;
use crate::lib::db::DbId;
use byteorder::WriteBytesExt;
use std::io::{BufWriter, Result, Seek, Write};

pub fn build_write_block<W: Write + Seek>(
    writer: &mut BufWriter<W>,
    db_id: &DbId,
    packet_provider: &mut dyn PacketProvider<&mut BufWriter<W>>,
) -> Result<BuildWriteBlockResult> {
    let write_large_packet = match packet_provider.next_large_packet()? {
        LargePacketResult::LargePacket(writer) => Some(writer),
        LargePacketResult::TooSmall => None,
        LargePacketResult::End => return Ok(BuildWriteBlockResult::Done),
    };

    let arity = match write_large_packet {
        None => BlockArity::ManyPackets,
        Some(_) => BlockArity::OnePacket,
    };

    writer.write_all(BlockSignature::new(arity).as_ref())?;
    writer.write_all(db_id.as_ref())?;
    let digest_stream_position = writer.stream_position()?;
    writer.write_all(&[0; BLOCK_DIGEST_SIZE])?;

    let packet_count_stream_position = match write_large_packet {
        None => Some(writer.stream_position()?),
        Some(_) => None,
    };
    if write_large_packet.is_none() {
        writer.write_u8(0)?;
    }

    let packets_start_stream_position = writer.stream_position()?;

    match write_large_packet {
        None => {
            let mut packets_count: u8 = 0;
            let mut bytes_remaining = MANY_PACKETS_MAX_SIZE;

            while bytes_remaining > 0 && packets_count < u8::MAX {
                let packet_start_stream_position = writer.stream_position()?;
                match packet_provider.next_packet(bytes_remaining)? {
                    PacketResult::Packet(write_packet) => {
                        write_packet(writer)?;
                        let packet_size = writer.stream_position()? - packet_start_stream_position;
                        if packet_size > bytes_remaining as u64 {
                            panic!()
                        }
                        bytes_remaining -= packet_size as usize;
                        packets_count += 1;
                    }
                    PacketResult::TooBig => {
                        writer.write_all(vec![0 as u8; bytes_remaining].as_slice())?;
                        return Ok(BuildWriteBlockResult::MoreBlocksRequired);
                    }
                    PacketResult::End => {
                        writer.write_all(vec![0 as u8; bytes_remaining].as_slice())?;
                        return Ok(BuildWriteBlockResult::Done);
                    }
                }
            }

            writer.write_all(vec![0 as u8; bytes_remaining].as_slice())?;
        }
        Some(write_packet) => {
            write_packet(writer)?;
            if writer.stream_position()? - packets_start_stream_position
                != ONE_PACKET_MAX_SIZE as u64
            {
                panic!()
            }
        }
    }

    Ok(BuildWriteBlockResult::MoreBlocksRequired)
}

pub enum BuildWriteBlockResult {
    MoreBlocksRequired,
    Done,
}
