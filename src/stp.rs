use std::{io, path::Path};

mod lexer;
mod parser;

#[derive(Debug)]
pub struct STP;

// TODO: Lexer accepts &[u8] -> &str

/// Parses an STP file from the given path.
pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<STP> {
    todo!()
}

/// Parses an STP file given in the form of a string.
pub fn parse_string(s: &str) -> io::Result<STP> {
    todo!()
}
