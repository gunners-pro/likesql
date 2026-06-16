use crate::{
    constants::{HEADER, PAGE_SIZE},
    error::DbError,
    page::Page,
};
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
};

pub struct Pager {
    file: File,
}

impl Pager {
    pub fn new(path: &str) -> Result<Self, DbError> {
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

    pub fn read_page(&mut self, page_id: u64) -> Result<Page, DbError> {
        let offset = page_id * PAGE_SIZE as u64;

        self.file.seek(io::SeekFrom::Start(offset))?;
        let mut page = Page::new([0u8; PAGE_SIZE]);
        self.file.read_exact(page.as_mut_bytes())?;
        Ok(page)
    }
}
