use std::io::Write;

mod error;

pub use error::*;

pub struct Writer<W: Write> {
    sink: W,
}
