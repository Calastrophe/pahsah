use logos::Logos;

use super::STL;
use lexer::Token;
use parser::Parser;
use pyo3::prelude::*;
use std::fs::read_to_string;
use std::io;

mod lexer;
mod parser;

#[pymodule]
pub fn ascii(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_string, m)?)
}

#[pyfunction]
/// Parse an STL file in the ASCII format.
pub fn parse_file(path: &str) -> io::Result<STL> {
    let contents = read_to_string(path)?;
    parse_string(&contents)
}

#[pyfunction]
/// Parse an STL file in the ASCII format from a given string.
pub fn parse_string(string: &str) -> io::Result<STL> {
    let tokens = Token::lexer(string);
    let parser = Parser::new(tokens);
    parser.parse()
}
