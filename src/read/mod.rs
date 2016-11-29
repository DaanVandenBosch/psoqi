pub mod quest;
mod qst;
mod dat;
mod bin;

use std::borrow::Cow;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, ReadError>;

#[derive(Debug)]
pub enum ReadError {
    Io(io::Error),
    Encoding(Cow<'static, str>),
    InvalidData
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::Io(err)
    }
}

impl From<Cow<'static, str>> for ReadError {
    fn from(err: Cow<'static, str>) -> ReadError {
        ReadError::Encoding(err)
    }
}
