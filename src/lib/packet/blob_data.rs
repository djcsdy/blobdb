use crate::lib::blob::{BlobId, BLOB_ID_SIZE};
use crate::lib::io::ReadExt;
use crate::lib::packet::packetizer::{Packetized, Packetizer, PacketizerPostUpdater};
use crate::lib::packet::raw::{MAX_PAYLOAD_SIZE, PAYLOAD_OFFSET};
use crate::lib::packet::{Packet, RawPacket};
use arrayref::array_ref;
use byteorder::{ByteOrder, LittleEndian};
use sha2::{Digest, Sha256};
use std::convert::TryInto;
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
        let mut payload_bytes = Vec::with_capacity(data.len() + BLOB_DATA_OFFSET);
        payload_bytes[BLOB_ID_OFFSET..BLOB_ID_SIZE].copy_from_slice(&blob_id.0);
        LittleEndian::write_u64(&mut payload_bytes[OFFSET_OFFSET..OFFSET_END], offset);
        payload_bytes[BLOB_DATA_OFFSET..].copy_from_slice(&data);

        BlobDataPacket(RawPacket::new(BLOB_DATA_PACKET_TYPE_ID, &payload_bytes))
    }

    fn new_anonymous(offset: u64, data: Vec<u8>) -> BlobDataPacket {
        BlobDataPacket::new(BlobId::anonymous(), offset, data)
    }

    pub fn size(&self) -> usize {
        self.0.size()
    }

    pub fn blob_id(&self) -> BlobId {
        BlobId(*array_ref!(self.0.payload(), BLOB_ID_OFFSET, BLOB_ID_SIZE))
    }

    pub fn with_blob_id(&self, blob_id: BlobId) -> BlobDataPacket {
        BlobDataPacket::new(blob_id, self.offset(), self.data().into())
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

impl<R: Read> Packetizer<(), ImportBlobDataPacketsPostUpdater> for ImportBlobDataPackets<R> {
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
            packet: BlobDataPacket::new_anonymous(self.offset.try_into().unwrap(), buf).into(),
            post_update: (),
        }
    }

    fn into_post_updater(self) -> ImportBlobDataPacketsPostUpdater {
        let (_, digest) = self.tee_reader.into_inner();
        ImportBlobDataPacketsPostUpdater {
            blob_id: BlobId(digest.finalize().into()),
        }
    }
}

pub struct ImportBlobDataPacketsPostUpdater {
    blob_id: BlobId,
}

impl PacketizerPostUpdater<()> for ImportBlobDataPacketsPostUpdater {
    fn apply_post_update(&mut self, packet: Packet, _: ()) -> Packet {
        match packet {
            Packet::BlobData(packet) => packet.with_blob_id(self.blob_id).into(),
            _ => panic!(),
        }
    }
}
