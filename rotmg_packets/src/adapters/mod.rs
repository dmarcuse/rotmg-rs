mod option;
mod primitives;
mod str;
mod vec;

use crate::raw::RawPacket;
use std::marker::PhantomData;
use std::str::Utf8Error;

/// An error reading or writing a packet.
#[derive(Debug, Clone, thiserror::Error)]
pub enum PacketFormatError {
    #[error("Expected at least {0} more bytes")]
    UnexpectedEnd(usize),

    #[error("Invalid UTF-8: {0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("Field too large: cannot convert {length} to {repr}")]
    FieldTooLarge { length: usize, repr: &'static str },
}

impl PacketFormatError {
    fn too_large<T>(length: usize) -> Self {
        Self::FieldTooLarge {
            length,
            repr: std::any::type_name::<T>(),
        }
    }
}

/// A simple interface for reading bytes from a raw packet.
pub struct PacketReader<'a> {
    remaining: &'a [u8],
}

impl<'a> PacketReader<'a> {
    /// Create a new reader for the given packet.
    pub fn new(packet: &'a RawPacket) -> Self {
        PacketReader {
            remaining: packet.payload(),
        }
    }

    /// Check whether there are any unparsed bytes remaining.
    pub fn is_empty(&self) -> bool {
        self.remaining.is_empty()
    }

    /// Get the number of remaining unparsed bytes.
    pub fn len(&self) -> usize {
        self.remaining.len()
    }

    /// Attempt to take the next `n` bytes from this reader, returning an error
    /// if there aren't enough remaining.
    pub fn take(&mut self, n: usize) -> Result<&'a [u8], Box<PacketFormatError>> {
        if self.remaining.len() < n {
            Err(Box::new(PacketFormatError::UnexpectedEnd(n)))
        } else {
            let (taken, remaining) = self.remaining.split_at(n);
            self.remaining = remaining;
            Ok(taken)
        }
    }
}

/// Data that can be read from a packet.
///
/// Note that the type this is implemented on need not match the actual returned
/// type.
pub trait FromPacketBytes<'a> {
    type Output;

    /// Read data from the given packet.
    fn from_packet(reader: &mut PacketReader<'a>) -> Result<Self::Output, Box<PacketFormatError>>;
}

/// Data that can be written to a packet.
pub trait ToPacketBytes<T> {
    //noinspection RsSelfConvention
    /// Write data to the given packet.
    fn to_packet(value: T, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>>;
}

/// A dummy type indicating that a dynamically sized type is prefixed with a
/// length field.
pub struct LengthPrefixed<N, T>(PhantomData<N>, PhantomData<T>);
