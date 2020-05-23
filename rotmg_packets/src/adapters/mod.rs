mod option;
mod primitives;
mod str;
mod vec;

use crate::raw::RawPacket;
use std::fmt::Display;
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
    FieldTooLarge { length: String, repr: &'static str },
}

impl PacketFormatError {
    fn too_large<T>(length: &dyn Display) -> Self {
        Self::FieldTooLarge {
            length: length.to_string(),
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
pub struct WithLen<N, T>(PhantomData<N>, PhantomData<T>);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! roundtrip_tests {
        ( $( $name:ident < $type:ty > ( $init:expr ) ),* $(,)? ) => {
            $(
                #[test]
                fn $name() {
                    let original: <$type as FromPacketBytes>::Output = $init;
                    let mut packet = vec![];
                    <$type as ToPacketBytes<_>>::to_packet(original.clone(), &mut packet).unwrap();
                    let mut reader = PacketReader { remaining: &packet };
                    let parsed = <$type as FromPacketBytes>::from_packet(&mut reader).unwrap();
                    assert_eq!(
                        original,
                        parsed,
                        "expected {:?}, got {:?} with encoded repr {:#x?}",
                        original,
                        parsed,
                        packet
                    );
                }
            )*
        }
    }

    roundtrip_tests! {
        // primitives
        test_roundtrip_bool<bool>(rand::random()),
        test_roundtrip_u8<u8>(rand::random()),
        test_roundtrip_u16<u16>(rand::random()),
        test_roundtrip_u32<u32>(rand::random()),
        test_roundtrip_u64<u64>(rand::random()),
        test_roundtrip_i8<i8>(rand::random()),
        test_roundtrip_i16<i16>(rand::random()),
        test_roundtrip_i32<i32>(rand::random()),
        test_roundtrip_i64<i64>(rand::random()),

        // option
        test_roundtrip_none<Option<i32>>(None),
        test_roundtrip_some_i32<Option<i32>>(Some(rand::random())),

        // str
        test_roundtrip_str_u16<WithLen<u16, &str>>("hello world"),
        test_roundtrip_str_u32<WithLen<u32, &str>>("hello world"),

        // vec
        test_roundtrip_vec_u16<WithLen<u16, Vec<i32>>>(vec![1, 3, -42]),
        test_roundtrip_vec_u32<WithLen<u32, Vec<i64>>>(vec![i64::MAX, 42, 8]),

        // nested dynamically sized types
        test_roundtrip_complex_none<Option<WithLen<u16, Vec<WithLen<u32, &str>>>>>(None),
        test_roundtrip_complex_some<Option<WithLen<u16, Vec<WithLen<u32, &str>>>>>(Some(vec!["hello", "world"])),
    }
}
