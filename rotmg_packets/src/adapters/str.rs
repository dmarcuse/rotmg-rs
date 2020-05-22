use super::*;
use std::convert::{TryFrom, TryInto};
use std::num::TryFromIntError;

impl<'a, N> FromPacketBytes<'a> for LengthPrefixed<N, &'a str>
where
    N: FromPacketBytes<'a>,
    N::Output: Into<usize>,
{
    type Output = &'a str;

    fn from_packet(reader: &mut PacketReader<'a>) -> Result<Self::Output, Box<PacketFormatError>> {
        let len = N::from_packet(reader)?;
        let bytes = reader.take(len.into())?;
        Ok(std::str::from_utf8(bytes).map_err(|e| Box::new(e.into()))?)
    }
}

impl<N, T: AsRef<str>> ToPacketBytes<T> for LengthPrefixed<N, &str>
where
    N: ToPacketBytes<N> + TryFrom<usize, Error = TryFromIntError>,
{
    fn to_packet(value: T, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        let value = value.as_ref();
        let len = value
            .len()
            .try_into()
            .map_err(|_| PacketFormatError::too_large::<N>(value.len()))?;
        N::to_packet(len, packet)?;
        packet.extend_from_slice(value.as_bytes());
        Ok(())
    }
}
