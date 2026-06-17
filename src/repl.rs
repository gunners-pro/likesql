use crate::{database::Database, error::DbError};
use std::io::{self, Write};

enum Command {
    Help,
    Exit,
    Pages,
    Alloc,
    Read(u64),
}

pub struct Repl {
    db: Database,
}

impl Repl {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn run(&mut self) -> Result<(), DbError> {
        loop {
            print!("LikeSQL:> ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            let command = match Self::parse_command(input) {
                Ok(cmd) => cmd,
                Err(msg) => {
                    println!("Error: {msg}");
                    continue;
                }
            };

            match command {
                Command::Help => {
                    println!("Commands:");
                    println!("  !help");
                    println!("  !exit");
                    println!("  !pages");
                    println!("  !alloc");
                    println!("  !read <page_id>");
                }
                Command::Exit => {
                    println!("Bye!");
                    break;
                }
                Command::Pages => {
                    let total_pages = match self.db.page_count() {
                        Ok(t) => t,
                        Err(msg) => {
                            println!("Error: {msg:?}");
                            continue;
                        }
                    };
                    println!("Total pages: {total_pages}");
                }
                Command::Alloc => {
                    let id = match self.db.allocate_page() {
                        Ok(id) => id,
                        Err(msg) => {
                            println!("Error: {msg:?}");
                            continue;
                        }
                    };
                    println!("Alloc id:{id} page");
                }
                Command::Read(page_id) => {
                    let page = match self.db.read_page(page_id) {
                        Ok(page) => page,
                        Err(msg) => {
                            println!("Error: {msg:?}");
                            continue;
                        }
                    };
                    println!("Read page {page:?}");
                }
            }
        }
        Ok(())
    }

    fn parse_command(input: &str) -> Result<Command, &'static str> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["!help"] => Ok(Command::Help),

            ["!exit"] => Ok(Command::Exit),

            ["!pages"] => Ok(Command::Pages),

            ["!alloc"] => Ok(Command::Alloc),

            ["!read", page] => {
                let page_id = page.parse::<u64>().map_err(|_| "Invalid page id")?;

                Ok(Command::Read(page_id))
            }

            _ => Err("Unknown command"),
        }
    }
}
