/// A raw ROTMG network packet
///
/// Packets consist of the following three parts, from beginning to end:
/// - Total packet length (4 bytes, big endian)
/// - Packet ID (1 byte)
/// - Packet contents (all remaining bytes)
// TODO: investigate using smallvec here to reduce heap usage
#[derive(PartialEq, Eq)]
pub struct RawPacket(Box<[u8]>);

impl RawPacket {
    /// Create a new `RawPacket` from the given bytes.
    ///
    /// This function will panic if the provided data is less than 5 bytes long.
    pub fn new(bytes: Box<[u8]>) -> Self {
        assert!(
            bytes.len() >= 5,
            "invalid packet length: {} < 5",
            bytes.len()
        );
        Self(bytes)
    }

    /// Unwrap this packet into the underlying bytes
    pub fn into_bytes(self) -> Box<[u8]> {
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

    /// Get the contents of this packet, excluding the length and packet ID
    pub fn contents(&self) -> &[u8] {
        &self.0[5..]
    }
}
