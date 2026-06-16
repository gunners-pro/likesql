use crate::{database::Database, error::DbError};

mod constants;
mod database;
mod error;
mod pager;

fn main() -> Result<(), DbError> {
    println!("Server started...");
    let mut db = Database::open("database.db")?;
    let page0 = db.read_page(0)?;

    println!("Database opened successfully! {page0:?}");
    Ok(())
}
