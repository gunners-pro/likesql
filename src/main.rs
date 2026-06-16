use crate::{database::Database, error::DbError};

mod constants;
mod database;
mod error;
mod page;
mod pager;

fn main() -> Result<(), DbError> {
    println!("Server started...");
    let mut db = Database::open("database.db")?;

    println!("Database opened successfully!");
    Ok(())
}
