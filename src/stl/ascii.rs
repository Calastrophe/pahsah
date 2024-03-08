use logos::Logos;

use super::STL;
use lexer::Token;
use parser::Parser;
use std::fs::read_to_string;
use std::io;
use std::path::Path;

mod lexer;
mod parser;

/// Parse an STL file in the ASCII format.
pub fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<STL> {
    let contents = read_to_string(path)?;
    parse_string(&contents)
}

/// Parse an STL file in the ASCII format from a given string.
pub fn parse_string(string: &str) -> io::Result<STL> {
    let tokens = Token::lexer(string);
    let parser = Parser::new(tokens);
    parser.parse()
}
