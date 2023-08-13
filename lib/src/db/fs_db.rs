use crate::block::Block;
use crate::file::path::{blob_file_path, lock_file_path, root_file_path, ROOT_FILE_NAME};
use crate::file::{import_blob, read_blob, ReadBlob};
use crate::{BlobId, Db, DbId};
use fs2::FileExt;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read};
use std::path::Path;
use std::{fs, io};

pub struct FsDb {
    id: DbId,
    path: Box<Path>,
    lock: Option<File>,
}

impl FsDb {
    pub fn open(path: &Path) -> io::Result<FsDb> {
        if fs::metadata(path)
            .map(|metadata| metadata.is_dir())
            .unwrap_or(false)
        {
            FsDb::open_dir(path)
        } else if path.file_name() == Some(OsStr::new(ROOT_FILE_NAME)) {
            path.parent()
                .ok_or_else(|| io::Error::from(ErrorKind::NotFound))
                .and_then(FsDb::open_dir)
        } else {
            Err(io::Error::from(ErrorKind::NotFound))
        }
    }

    fn open_dir(path: &Path) -> io::Result<FsDb> {
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

        Ok(FsDb {
            id,
            path: path.into(),
            lock: Some(lock),
        })
    }
}

impl Db<File> for FsDb {
    fn import_blob<R>(&self, reader: &mut R) -> io::Result<BlobId>
    where
        R: Read,
    {
        import_blob(self.id, &self.path, reader)
    }

    fn read_blob(&self, blob_id: BlobId) -> io::Result<ReadBlob<File>> {
        File::open(blob_file_path(&self.path, blob_id))
            .map(|file| read_blob(self.id, blob_id, file))
    }
}

impl Drop for FsDb {
    fn drop(&mut self) {
        self.lock = None;
        fs::remove_file(lock_file_path(&self.path)).unwrap_or_default();
    }
}
