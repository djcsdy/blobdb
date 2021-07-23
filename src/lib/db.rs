use crate::lib::block::Block;
use crate::lib::db_id::DbId;
use crate::lib::packet::Packet;
use fs2::FileExt;
use itertools::Itertools;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Error, ErrorKind, Seek};
use std::path::Path;
use std::{fs, io};

pub struct Db {
    pub id: DbId,
    pub path: Box<Path>,
    lock: File,
}

impl Db {
    pub fn open(path: &Path) -> io::Result<Db> {
        if fs::metadata(path)?.is_dir() {
            Db::open_dir(path)
        } else if path.file_name() == Some(OsStr::new("BlobDB")) {
            path.parent()
                .ok_or(io::Error::from(ErrorKind::NotFound))
                .and_then(|path| Db::open_dir(path))
        } else {
            Err(io::Error::from(ErrorKind::NotFound))
        }
    }

    fn open_dir(path: &Path) -> io::Result<Db> {
        let lock = File::open(path.join("BlobDB.lock"))?;
        lock.lock_exclusive()?;

        let mut root_file = File::open(path.join("BlobDB"))?;
        let block = Block::read(&mut root_file)?;
        let end = Block::read(&mut root_file);

        if !block.packets.is_empty()
            || end
                .err()
                .map(|err| err.kind() != ErrorKind::UnexpectedEof)
                .unwrap_or(false)
        {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        let id = block.db_id;

        Ok(Db {
            id,
            path: path.into(),
            lock,
        })
    }
}
