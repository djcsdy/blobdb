use crate::lib::block::Block;
use crate::lib::db::db_id::DbId;
use crate::lib::file::path::{lock_file_path, root_file_path, ROOT_FILE_NAME};
use fs2::FileExt;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

pub struct Db {
    id: DbId,
    path: Box<Path>,
    lock: Option<File>,
}

impl Db {
    pub fn open(path: &Path) -> io::Result<Db> {
        if fs::metadata(path)
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
        {
            Db::open_dir(path)
        } else if path.file_name() == Some(OsStr::new(ROOT_FILE_NAME)) {
            path.parent()
                .ok_or_else(|| io::Error::from(ErrorKind::NotFound))
                .and_then(Db::open_dir)
        } else {
            Err(io::Error::from(ErrorKind::NotFound))
        }
    }

    fn open_dir(path: &Path) -> io::Result<Db> {
        let lock = OpenOptions::new()
            .create(true)
            .write(true)
            .open(lock_file_path(path))?;
        lock.try_lock_exclusive()?;

        let mut root_file = File::open(root_file_path(path))?;
        let block = Block::read(&mut root_file)?;
        let end = Block::read(&mut root_file);

        let id = block.db_id();

        if block.into_packets().len() != 0
            || end
                .err()
                .map(|err| err.kind() != ErrorKind::UnexpectedEof)
                .unwrap_or(false)
        {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        Ok(Db {
            id,
            path: path.into(),
            lock: Some(lock),
        })
    }

    pub fn id(&self) -> DbId {
        self.id
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        self.lock = None;
        fs::remove_file(lock_file_path(&self.path)).unwrap_or_default();
    }
}
