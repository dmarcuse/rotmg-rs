//! Raw, unparsed ROTMG packets.
//!
//! Raw packets represent packets that have been framed and decrypted so that
//! the binary payload and ID can be accessed, but aren't necessarily parsed.
//! These are primarily intended as an intermediary form, for cases where a
//! packet may not need to be parsed, or parsing may not be possible.
//!
//! The `RawPacket` type is represented as `[u8]` - an unsized byte slice.
//! Consequently, it should generally be used behind a layer of indirection -
//! such as a reference or `Box` - and works similarly to types like `str`. An
//! unowned packet could be represented as `&RawPacket` or `&mut RawPacket`, and
//! an owned packet could be represented as `Box<RawPacket>`.

use crate::adapters::{PacketFormatError, PacketReader};
use crate::structured::packets::AnyPacket;
use crate::PacketMappings;
use std::convert::{AsMut, TryInto};
use std::fmt::{self, Debug, Formatter};

/// An error wrapping bytes as a packet
#[derive(Debug, Clone, thiserror::Error)]
pub enum InvalidPacket {
    /// The header is too short (<5 bytes).
    #[error("Packet header too short - need at least 5 bytes: {0:x?}")]
    HeaderTooShort(Box<[u8]>),

    /// The length encoded in the packet header doesn't match the actual length
    /// of the packet.
    #[error("Invalid packet header - encoded length is {encoded_len} but actual length is {actual_len}: {data:x?}")]
    HeaderInvalid {
        /// The length that was encoded in the packet data.
        encoded_len: u32,

        /// The actual length of the packet.
        actual_len: usize,

        /// The complete contents of the offending packet.
        data: Box<[u8]>,
    },
}

/// A raw ROTMG packet.
///
/// Since this type is implemented as a newtype over `[u8]` it is unsized and
/// should be used via a reference, box, or other form of indirection.
#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct RawPacket([u8]);

impl RawPacket {
    fn validate_header(bytes: &[u8]) -> Result<(), InvalidPacket> {
        if bytes.len() < 5 {
            return Err(InvalidPacket::HeaderTooShort(
                bytes.to_vec().into_boxed_slice(),
            ));
        }

        let encoded_len = u32::from_be_bytes(bytes[..4].try_into().unwrap());
        if encoded_len as usize != bytes.len() {
            return Err(InvalidPacket::HeaderInvalid {
                encoded_len,
                actual_len: bytes.len(),
                data: bytes.to_vec().into_boxed_slice(),
            });
        }

        Ok(())
    }

    /// Attempt to convert a slice of bytes to a `RawPacket` reference.
    pub fn from_slice(bytes: &[u8]) -> Result<&Self, InvalidPacket> {
        Self::validate_header(bytes)?;
        // SAFETY: safe because [u8] and RawPacket have the same layout
        Ok(unsafe { &*(bytes as *const [u8] as *const RawPacket) })
    }

    /// Attempt to convert a mutable slice of bytes to a `RawPacket` reference.
    pub fn from_mut(bytes: &mut [u8]) -> Result<&mut Self, InvalidPacket> {
        Self::validate_header(bytes)?;
        // SAFETY: safe because [u8] and RawPacket have the same layout
        Ok(unsafe { &mut *(bytes as *mut [u8] as *mut RawPacket) })
    }

    /// Attempt to convert a boxed slice of bytes to a boxed `RawPacket`.
    pub fn from_box(bytes: Box<[u8]>) -> Result<Box<Self>, InvalidPacket> {
        Self::validate_header(&bytes)?;
        // SAFETY: safe because [u8] and RawPacket have the same layout
        Ok(unsafe { std::mem::transmute(bytes) })
    }

    /// Get the contents of this packet, including the header.
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get the contents of this packet, excluding the header.
    pub fn payload(&self) -> &[u8] {
        &self.0[5..]
    }

    /// Get a mutable reference to the contents of this packet, excluding the
    /// header.
    pub fn payload_mut(&mut self) -> &mut [u8] {
        &mut self.0[5..]
    }

    /// Get the raw ID of this packet.
    pub fn id(&self) -> u8 {
        self.0[4]
    }

    /// Attempt to parse this raw packet into a structured packet using the
    /// given mappings.
    ///
    /// On success, the packet will be returned, along with any remaining bytes
    /// that weren't captured (leftover bytes usually indicates out-of-date or
    /// incorrect packet definitions). If the packet ID isn't known or there's
    /// an error parsing the packet, the error will be returned instead.
    #[allow(clippy::type_complexity)]
    pub fn parse(
        &self,
        mappings: &PacketMappings,
    ) -> Result<(Box<dyn AnyPacket>, &[u8]), Box<PacketFormatError>> {
        let typ = mappings
            .to_internal(self.id())
            .ok_or_else(|| Box::new(PacketFormatError::UnmappedID(self.id())))?;

        let mut reader = PacketReader::new(self);
        let parsed = typ.parse_bytes(&mut reader)?;
        Ok((parsed, reader.take_all()))
    }
}

impl ToOwned for RawPacket {
    type Owned = Box<RawPacket>;

    fn to_owned(&self) -> Self::Owned {
        // SAFETY: safe because [u8] and RawPacket have the same layout
        unsafe { std::mem::transmute(self.bytes().to_vec().into_boxed_slice()) }
    }
}

impl Clone for Box<RawPacket> {
    fn clone(&self) -> Self {
        let bytes = self.bytes().to_vec().into_boxed_slice();
        RawPacket::from_box(bytes).unwrap()
    }
}

impl Debug for RawPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RawPacket").field(&self.bytes()).finish()
    }
}

impl AsMut<RawPacket> for RawPacket {
    fn as_mut(&mut self) -> &mut RawPacket {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::RawPacket;

    #[test]
    fn test_raw_packet_parsing() {
        let data = &[0, 0, 0, 6, 5, 6];
        let pkt = RawPacket::from_slice(&data[..]).unwrap();
        assert_eq!(data, pkt.bytes());
        let pkt = pkt.to_owned();
        assert_eq!(data, pkt.bytes());
        let boxed = RawPacket::from_box(data.to_vec().into_boxed_slice()).unwrap();
        assert_eq!(data, boxed.bytes());
        assert_eq!(pkt, boxed);
    }
}
