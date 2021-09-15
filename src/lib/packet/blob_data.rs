use crate::lib::blob::{BlobId, BLOB_ID_SIZE};
use crate::lib::io::ReadExt;
use crate::lib::packet::build::{build_blob_data, DraftBlobDataPacket};
use crate::lib::packet::packetizer::{Packetized, Packetizer};
use crate::lib::packet::raw::{MAX_PAYLOAD_SIZE, PAYLOAD_OFFSET};
use crate::lib::packet::{Packet, RawPacket};
use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};
use sha2::{Digest, Sha256};
use std::io::Read;
use tee_readwrite::TeeReader;

pub const BLOB_DATA_PACKET_TYPE_ID: u8 = 1;

pub const BLOB_DATA_PACKET_OVERHEAD: usize = PAYLOAD_OFFSET + BLOB_DATA_OFFSET;

pub const OFFSET_SIZE: usize = 8;
pub const MAX_DATA_SIZE: usize = MAX_PAYLOAD_SIZE - BLOB_DATA_OFFSET;

pub const BLOB_ID_OFFSET: usize = 0;
pub const BLOB_ID_END: usize = BLOB_ID_OFFSET + BLOB_ID_SIZE;
pub const OFFSET_OFFSET: usize = BLOB_ID_END;
pub const OFFSET_END: usize = OFFSET_OFFSET + OFFSET_SIZE;
pub const BLOB_DATA_OFFSET: usize = OFFSET_END;

pub struct BlobDataPacket(pub(super) RawPacket);

impl AsRef<[u8]> for BlobDataPacket {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl BlobDataPacket {
    pub fn new(blob_id: BlobId, offset: u64, data: Vec<u8>) -> BlobDataPacket {
        BlobDataPacket(RawPacket::new(build_blob_data(
            blob_id,
            DraftBlobDataPacket { offset, data },
        )))
    }

    fn new_anonymous(offset: u64, data: Vec<u8>) -> BlobDataPacket {
        BlobDataPacket::new(BlobId::anonymous(), offset, data)
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    pub fn blob_id(&self) -> BlobId {
        BlobId(array_ref!(self.0.payload(), BLOB_ID_OFFSET, BLOB_ID_SIZE).clone())
    }

    pub fn offset(&self) -> u64 {
        LittleEndian::read_u64(array_ref!(self.0.payload(), OFFSET_OFFSET, OFFSET_SIZE))
    }

    pub fn data(&self) -> &[u8] {
        &self.0.payload()[BLOB_DATA_OFFSET..]
    }

    pub fn import_blob<R: Read>(reader: R) -> ImportBlobDataPackets<R> {
        ImportBlobDataPackets {
            tee_reader: TeeReader::new(reader, Sha256::new(), false),
            offset: 0,
            end: false,
        }
    }
}

pub struct ImportBlobDataPackets<R: Read> {
    tee_reader: TeeReader<R, Sha256>,
    offset: usize,
    end: bool,
}

impl<R: Read> Packetizer<()> for ImportBlobDataPackets<R> {
    fn next_packet(&mut self, max_size: u16) -> Packetized<()> {
        if self.end {
            return Packetized::End;
        } else if (max_size as usize) <= BLOB_DATA_PACKET_OVERHEAD {
            return Packetized::PacketTooBig;
        }

        let max_data_size = max_size as usize - BLOB_DATA_PACKET_OVERHEAD;

        let mut buf = vec![0; max_data_size];
        self.offset += self.tee_reader.read_at_most(&mut buf).unwrap(); // TODO handle

        Packetized::Packet {
            packet: RawPacket::new(buf).into(),
            post_update: (),
        }
    }

    fn apply_post_update(&mut self, packet: &mut Packet, _: ()) {
        match packet {
            Packet::BlobData(packet) => todo!("set blob_id"),
            _ => panic!(),
        }
    }
}

// impl<R: Read> ImportBlobDataPackets<R> {
//     pub fn end_and_digest_id(self) -> BlobId {
//         let (_, digest) = self.tee_reader.into_inner();
//         BlobId(digest.finalize().into())
//     }
// }
