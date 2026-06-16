use crate::constants::{HEADER, PAGE_SIZE};

pub struct Page {
    data: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new(data: [u8; PAGE_SIZE]) -> Self {
        Self { data }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn header(&self) -> &[u8; HEADER.len()] {
        self.data[..HEADER.len()]
            .try_into()
            .expect("Invalid header length.")
    }
}
