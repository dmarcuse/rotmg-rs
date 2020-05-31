//! Basic parser for AVM2 bytecode

#![allow(dead_code)]

#[macro_use]
pub mod macros;

pub mod abcfile;
pub mod class;
pub mod constants;
pub mod metadata;
pub mod methods;
pub mod primitives;
pub mod traits;

use std::string::FromUtf8Error;

/// An error parsing an AVM2 type
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// Not enough bytes remained in the buffer to deserialize this type
    #[error(
        "Not enough bytes remaining in buffer: need {needed} bytes, {remaining} bytes remaining"
    )]
    InsufficientBytes { remaining: usize, needed: usize },

    /// Invalid UTF-8 while parsing a string
    #[error("Invalid UTF-8: {0}")]
    Utf8Error(#[from] FromUtf8Error),

    #[error("Flag value {value} ({value:#x}) is invalid for type {typ}")]
    InvalidFlag { value: u8, typ: &'static str },
}

pub struct Reader<'a> {
    remaining: &'a [u8],
}

impl<'a> Reader<'a> {
    /// Create a new reader with the given bytes.
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { remaining: bytes }
    }

    /// Attempt to take the next `n` bytes from this reader.
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], ParseError> {
        if n > self.remaining.len() {
            Err(ParseError::InsufficientBytes {
                remaining: self.remaining.len(),
                needed: n,
            })
        } else {
            let (taken, remaining) = self.remaining.split_at(n);
            self.remaining = remaining;
            Ok(taken)
        }
    }

    /// Peek at the bytes remaining in this reader.
    pub fn peek(&self) -> &'a [u8] {
        self.remaining
    }

    /// Take all bytes remaining in the buffer.
    pub fn take_all(&mut self) -> &'a [u8] {
        std::mem::take(&mut self.remaining)
    }
}

/// A trait defining functionality for parsing an AVM2 type
pub trait Parse: Sized {
    /// Parse this type from the provided bytes
    fn parse_avm2(input: &mut Reader) -> Result<Self, ParseError>;
}
