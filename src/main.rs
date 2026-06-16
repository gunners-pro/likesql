use crate::{constants::HEADER, database::Database, error::DbError};

mod constants;
mod database;
mod error;
mod page;
mod pager;

fn main() -> Result<(), DbError> {
    println!("Server started...");
    let mut db = Database::open("database.db")?;
    let page0 = db.read_page(0)?;

    assert_eq!(page0.header(), HEADER);

    println!("Database opened successfully!");
    Ok(())
}
