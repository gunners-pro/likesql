use crate::database::Database;
use std::io;

mod database;
mod error;

fn main() -> io::Result<()> {
    println!("Server started...");
    let db = Database::open("database.db").unwrap();
    println!("Database opened successfully!");
    Ok(())
}
