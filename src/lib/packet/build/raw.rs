use crate::lib::packet::type_id_and_length::combine_type_id_and_length;
use byteorder::{LittleEndian, WriteBytesExt};
use std::convert::TryInto;
use std::io::{Result, Seek, SeekFrom, Write};

pub fn build_write_raw_packet<W, P, R>(mut writer: W, type_id: u8, write_payload: P) -> Result<R>
where
    W: Write,
    W: Seek,
    P: FnOnce(&mut W) -> Result<R>,
{
    let type_id_and_length_stream_position = writer.stream_position()?;
    writer.write_u16::<LittleEndian>(0)?;
    let payload_start_stream_position = writer.stream_position()?;
    let result = write_payload(&mut writer)?;
    let end_stream_position = writer.stream_position()?;
    if end_stream_position < payload_start_stream_position {
        panic!()
    }
    let length: usize = (end_stream_position - payload_start_stream_position)
        .try_into()
        .unwrap();
    writer.seek(SeekFrom::Start(type_id_and_length_stream_position))?;
    writer.write_u16::<LittleEndian>(combine_type_id_and_length(type_id, length))?;
    writer.seek(SeekFrom::Start(end_stream_position))?;
    Ok(result)
}
