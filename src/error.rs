use std::io;

#[derive(Debug)]
pub enum DbError {
    Io(io::Error),
    InvalidHeader,
}

impl From<io::Error> for DbError {
    fn from(err: io::Error) -> Self {
        DbError::Io(err)
    }
}
