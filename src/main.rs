use crate::{database::Database, error::DbError, repl::Repl};

mod constants;
mod database;
mod error;
mod page;
mod pager;
mod repl;

fn main() -> Result<(), DbError> {
    println!("Server started...");
    let db = Database::open("database.db")?;
    let mut repl = Repl::new(db);

    repl.run()?;
    Ok(())
}
