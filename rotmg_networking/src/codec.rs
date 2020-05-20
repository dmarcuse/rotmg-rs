use crate::rc4::Rc4;
use futures::io::ErrorKind;
use rotmg_packets::RawPacket;
use std::io;
use tokio::prelude::*;

/// An encoder for writing ROTMG packets.
pub struct Encoder<T> {
    stream: T,
    cipher: Rc4,
}

impl<T: AsyncWrite + Unpin> Encoder<T> {
    pub fn new(stream: T, cipher: Rc4) -> Self {
        Self { stream, cipher }
    }

    /// Write the given packet to this encoder.
    ///
    /// An error leaves the encoder in an undefined state, and future attempts
    /// to write with the same encoder are likely to fail.
    pub async fn send(&mut self, mut packet: impl AsMut<RawPacket>) -> io::Result<()> {
        let packet = packet.as_mut();
        self.cipher.process(packet.payload_mut());
        self.stream.write_all(packet.bytes()).await?;
        Ok(())
    }

    /// Get a reference to the underlying data stream.
    pub fn inner(&self) -> &T {
        &self.stream
    }

    /// Unwrap the underlying stream.
    pub fn into_inner(self) -> T {
        self.stream
    }
}

/// A decoder for reading ROTMG packets.
pub struct Decoder<T> {
    stream: T,
    cipher: Rc4,
    buffer: Vec<u8>,
}

/// An error returned when a `Decoder` attempts to decode an excessively large
/// packet.
///
/// When a packet larger than the limit (`Decoder::MAX_PACKET_SIZE`) is
/// encountered, this error will be returned instead.
#[error("Packet size limit of {limit} bytes was exceeded: {size} bytes specified")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub struct PacketSizeLimitExceeded {
    pub limit: u32,
    pub size: u32,
}

impl<T: AsyncRead + Unpin> Decoder<T> {
    /// Maximum allowed packet size, in bytes.
    ///
    /// Although the layout of packets allows sizes of up to 2^32 bytes, a
    /// reasonable upper bound is set to prevent malicious connections
    /// attempting to exhaust system memory.
    pub const MAX_PACKET_SIZE: u32 = 10 * 1024 * 1024;

    pub fn new(stream: T, cipher: Rc4) -> Self {
        Self {
            stream,
            cipher,
            buffer: Vec::new(),
        }
    }

    /// Read a packet from this decoder.
    ///
    /// A value of `None` indicates that the stream has closed and no further
    /// packets can be read.
    ///
    /// An error leaves the decoder in an undefined state, and future attempts
    /// to read with the same decoder are likely to fail.
    pub async fn recv(&mut self) -> io::Result<Option<&mut RawPacket>> {
        // receive packet length
        let mut len_buf = [0, 0, 0, 0];
        let len = match self.stream.read_exact(&mut len_buf).await {
            Ok(_) => u32::from_be_bytes(len_buf),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        };

        if len > Self::MAX_PACKET_SIZE {
            return Err(io::Error::new(
                ErrorKind::Other,
                PacketSizeLimitExceeded {
                    limit: Self::MAX_PACKET_SIZE,
                    size: len,
                },
            ));
        }

        // receive rest of packet
        self.buffer.resize(len as usize, 0);
        self.buffer[..4].copy_from_slice(&len_buf);
        match self.stream.read_exact(&mut self.buffer[4..]).await {
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        }

        // decrypt payload and wrap packet
        self.cipher.process(&mut self.buffer[5..]);
        match RawPacket::from_mut(&mut self.buffer) {
            Ok(p) => Ok(Some(p)),
            Err(e) => Err(io::Error::new(ErrorKind::InvalidData, e)),
        }
    }

    /// Get a reference to the underlying stream.
    pub fn inner(&self) -> &T {
        &self.stream
    }

    /// Unwrap the underlying stream.
    pub fn into_inner(self) -> T {
        self.stream
    }
}
