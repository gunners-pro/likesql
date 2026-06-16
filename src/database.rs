use crate::error::DbError;
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
};

const HEADER: &[u8; 8] = b"LIKESQL1";
const PAGE_SIZE: usize = 4096;

pub struct Database {
    file: File,
}

impl Database {
    pub fn open(path: &str) -> Result<Self, DbError> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let metadata = file.metadata()?;

        if metadata.len() == 0 {
            let mut page = [0u8; PAGE_SIZE];
            page[..HEADER.len()].copy_from_slice(HEADER);
            file.write_all(&page)?;
        }

        file.seek(io::SeekFrom::Start(0))?;

        let mut buf_header = [0u8; HEADER.len()];
        file.read_exact(&mut buf_header)?;
        if &buf_header != HEADER {
            return Err(DbError::InvalidHeader);
        }

        Ok(Self { file })
    }
}
