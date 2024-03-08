use crate::stl::{Triangle, STL};

use super::lexer::Token;
use glam::Vec3;
use logos::Lexer;
use std::io::{Error, ErrorKind};

/// Parses the incoming tokens from the lexer and builds up triangles to then create an STL file.
/// Attribute bytes are not present in the ASCII format.
pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
    triangles: Vec<Triangle>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser with the given token stream.
    pub fn new(lexer: Lexer<'a, Token>) -> Parser<'a> {
        Parser {
            lexer,
            triangles: Vec::new(),
        }
    }

    /// Parses an STL file from the given token stream.
    pub fn parse(mut self) -> Result<STL, Error> {
        self.parse_token(Token::Solid)?;
        loop {
            match self.lexer.next() {
                Some(Ok(Token::Facet)) => {
                    let triangle = self.parse_triangle()?;
                    self.triangles.push(triangle);
                }
                Some(Ok(Token::EndSolid)) => {
                    return Ok(STL {
                        triangles: self.triangles,
                    });
                }
                Some(Ok(token)) => return Err(unexpected_token(token)),
                Some(Err(_)) => return Err(invalid_keyword(self.lexer.slice())),
                None => return Err(unexpected_end()),
            };
        }
    }

    /// Parse out a triangle from the token stream.
    fn parse_triangle(&mut self) -> Result<Triangle, Error> {
        self.parse_token(Token::Normal)?;
        let normal = Vec3 {
            x: self.parse_f32()?,
            y: self.parse_f32()?,
            z: self.parse_f32()?,
        };
        let [v1, v2, v3] = self.parse_loop()?;
        self.parse_token(Token::EndFacet)?;
        Ok(Triangle::new(normal, v1, v2, v3, 0))
    }

    /// Parse out three vectors from the token stream.
    fn parse_loop(&mut self) -> Result<[Vec3; 3], Error> {
        self.parse_token(Token::Outer)?;
        self.parse_token(Token::Loop)?;
        let v1 = self.parse_vertex()?;
        let v2 = self.parse_vertex()?;
        let v3 = self.parse_vertex()?;
        self.parse_token(Token::EndLoop)?;
        Ok([v1, v2, v3])
    }

    /// Parse out a single vertex from the token stream.
    fn parse_vertex(&mut self) -> Result<Vec3, Error> {
        self.parse_token(Token::Vertex)?;
        Ok(Vec3 {
            x: self.parse_f32()?,
            y: self.parse_f32()?,
            z: self.parse_f32()?,
        })
    }

    /// Parse a specific token from the stream, otherwise throw an error.
    fn parse_token(&mut self, t: Token) -> Result<(), Error> {
        match self.lexer.next() {
            Some(Ok(token)) if token == t => Ok(()),
            Some(Ok(token)) => Err(unexpected_token(token)),
            Some(Err(_)) => Err(invalid_keyword(self.lexer.slice())),
            None => Err(unexpected_end()),
        }
    }

    /// Parse an f32 from the token stream, otherwise throw an error.
    fn parse_f32(&mut self) -> Result<f32, Error> {
        match self.lexer.next() {
            Some(Ok(Token::Float(num))) => Ok(num),
            Some(Ok(token)) => Err(unexpected_token(token)),
            Some(Err(_)) => Err(invalid_keyword(self.lexer.slice())),
            None => Err(unexpected_end()),
        }
    }
}

/// Helper for invalid keyword
fn invalid_keyword(s: &str) -> Error {
    Error::new(ErrorKind::InvalidData, format!("Invalid token: {}", s))
}

/// Helper for unexpected end of input
fn unexpected_end() -> Error {
    Error::new(ErrorKind::InvalidData, "Unexpected end of input")
}

/// Helper for unexpected token
fn unexpected_token(token: Token) -> Error {
    Error::new(
        ErrorKind::InvalidData,
        format!("Unexpected token: {:?}", token),
    )
}
