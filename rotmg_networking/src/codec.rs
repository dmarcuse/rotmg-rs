use crate::rc4::Rc4;
use bytes::{Buf, BufMut, BytesMut};
use rotmg_packets::RawPacket;
use std::io;
use tokio_util::codec::{Decoder, Encoder};

/// A codec for framing and encrypting/decrypting packets from a ROTMG network
/// connection. This can be used for either the client or the server end of the
/// connection.
pub struct Codec {
    recv_rc4: Rc4,
    send_rc4: Rc4,
}

#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error("IO error")]
    IoError(#[from] io::Error),

    #[error("Packet had invalid length: {0}")]
    InvalidLength(usize),
}

impl Decoder for Codec {
    type Item = RawPacket;
    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            // we can't frame the packet until we have the length
            return Ok(None);
        }

        let length = src.bytes().get_u32() as usize;

        // the smallest legal packet is just a length + ID, 5 bytes
        if length < 5 {
            return Err(CodecError::InvalidLength(length));
        }

        if src.len() < length {
            // we haven't received the full packet yet
            return Ok(None);
        }

        // full packet received, remove from buffer and decrypt contents
        let mut packet = src.split_to(length);
        self.recv_rc4.process(&mut packet[5..]);
        Ok(Some(RawPacket::new(packet.freeze())))
    }
}

impl Encoder<RawPacket> for Codec {
    type Error = CodecError;

    fn encode(&mut self, item: RawPacket, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // encrypt packet payload, then write entire packet to buffer
        let mut packet = item.into_bytes().to_vec();
        self.send_rc4.process(&mut packet[5..]);
        dst.put_slice(&packet);
        Ok(())
    }
}
