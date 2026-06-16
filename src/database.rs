use crate::{error::DbError, page::Page, pager::Pager};

pub struct Database {
    pager: Pager,
}

impl Database {
    pub fn open(path: &str) -> Result<Self, DbError> {
        let pager = Pager::new(path)?;

        Ok(Self { pager })
    }

    pub fn read_page(&mut self, page_id: u64) -> Result<Page, DbError> {
        self.pager.read_page(page_id)
    }

    pub fn write_page(&mut self, page_id: u64, page: &Page) -> Result<(), DbError> {
        self.pager.write_page(page_id, page)
    }
}
