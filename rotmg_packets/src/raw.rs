use bytes::{Buf, Bytes};

/// A raw ROTMG network packet
///
/// Packets consist of the following three parts, from beginning to end:
/// - Total packet length (4 bytes, big endian)
/// - Packet ID (1 byte)
/// - Packet contents (all remaining bytes)
// TODO: investigate using smallvec here to reduce heap usage
#[derive(PartialEq, Eq, Clone)]
pub struct RawPacket(Bytes);

impl RawPacket {
    /// Create a new `RawPacket` from the given bytes.
    ///
    /// This function will panic if the provided data is less than 5 bytes long,
    /// or if the length doesn't match the one encoded in the packet data.
    pub fn new(data: Bytes) -> Self {
        assert!(data.len() >= 5, "invalid packet length: {} < 5", data.len());

        assert_eq!(
            data.len(),
            data.bytes().get_u32() as usize,
            "actual packet length doesn't match length encoded in contents"
        );

        Self(data)
    }

    /// Unwrap this packet into the underlying bytes
    pub fn into_bytes(self) -> Bytes {
        self.0
    }

    /// Get the bytes of this packet, including the length and packet ID
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get the ID for this packet
    pub fn packet_id(&self) -> u8 {
        self.0[4]
    }

    /// Get the payload of the packet - contents excluding length and ID header
    pub fn payload(&self) -> &[u8] {
        &self.0[5..]
    }
}
