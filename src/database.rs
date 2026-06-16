use crate::{constants::PAGE_SIZE, error::DbError, pager::Pager};

pub struct Database {
    pager: Pager,
}

impl Database {
    pub fn open(path: &str) -> Result<Self, DbError> {
        let pager = Pager::new(path)?;

        Ok(Self { pager })
    }

    pub fn read_page(&mut self, page_id: u64) -> Result<[u8; PAGE_SIZE], DbError> {
        self.pager.read_page(page_id)
    }
}
