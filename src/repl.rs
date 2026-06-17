use crate::{constants::PAGE_SIZE, database::Database, error::DbError, page::Page};
use std::io::{self, Write};

enum Command {
    Help,
    Exit,
    Pages,
    Alloc,
    Read(u64),
    Write(u64, String),
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
                Command::Write(page_id, payload) => {
                    let mut page = [0u8; PAGE_SIZE];
                    let payload = payload.as_bytes();

                    if page_id == 0 {
                        println!("Error: Cannot write to page 0.");
                        continue;
                    }

                    if payload.len() > PAGE_SIZE {
                        println!("Error: Payload too large for page sizes.");
                        continue;
                    }

                    page[..payload.len()].copy_from_slice(payload);
                    match self.db.write_page(page_id, &Page::new(page)) {
                        Ok(_) => println!("Page written successfully"),
                        Err(msg) => {
                            println!("Error: {msg:?}");
                            continue;
                        }
                    };
                }
            }
        }
        Ok(())
    }

    fn parse_command(input: &str) -> Result<Command, &'static str> {
        let parts: Vec<&str> = input.splitn(3, ' ').collect();

        match parts.as_slice() {
            ["!help"] => Ok(Command::Help),

            ["!exit"] => Ok(Command::Exit),

            ["!pages"] => Ok(Command::Pages),

            ["!alloc"] => Ok(Command::Alloc),

            ["!read", page] => {
                let page_id = page.parse::<u64>().map_err(|_| "Invalid page id")?;

                Ok(Command::Read(page_id))
            }
            ["!write", page_id, payload] => {
                let page_id = page_id.parse::<u64>().map_err(|_| "Invalid page id")?;
                Ok(Command::Write(page_id, payload.to_string()))
            }

            _ => Err("Unknown command"),
        }
    }
}
