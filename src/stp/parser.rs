use super::{lexer::Token, STP};
use logos::Lexer;
use std::io::{Error, ErrorKind};

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, Token>) -> Parser<'a> {
        Parser { lexer }
    }

    pub fn parse() -> Result<STP, Error> {
        todo!()
    }
}
