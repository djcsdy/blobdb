use std::io::{ErrorKind, Read, Result};

pub trait ReadExt: Read {
    fn read_at_most(&mut self, buf: &mut [u8]) -> Result<usize>;
}

impl<R: Read + ?Sized> ReadExt for R {
    fn read_at_most(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut pos = 0;
        loop {
            match self.read(&mut buf[pos..]) {
                Ok(0) => break,
                Ok(count) => pos += count,
                Err(error) => match error.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => return Err(error),
                },
            }
        }
        Ok(pos)
    }
}
