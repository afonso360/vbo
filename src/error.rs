use core::fmt;
use crate::ChannelName;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    TimeFormatError(time::error::Format),
    DuplicateChannel(ChannelName),

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "IOError({})", e),
            Error::TimeFormatError(e) => write!(f, "TimeFormatError({})", e),
            Error::DuplicateChannel(n) => write!(f, "DuplicateChannel({:?})", n),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}
impl From<time::error::Format> for Error {
    fn from(e: time::error::Format) -> Self {
        Error::TimeFormatError(e)
    }
}
