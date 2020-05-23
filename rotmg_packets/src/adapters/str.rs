use super::*;
use num_traits::{FromPrimitive, ToPrimitive, Unsigned};
use std::fmt::Display;

impl<'a, N> FromPacketBytes<'a> for WithLen<N, &'a str>
where
    N: FromPacketBytes<'a>,
    N::Output: ToPrimitive + Unsigned + Display,
{
    type Output = &'a str;

    fn from_packet(reader: &mut PacketReader<'a>) -> Result<Self::Output, Box<PacketFormatError>> {
        let len = N::from_packet(reader)?;
        let len = len
            .to_usize()
            .ok_or_else(|| PacketFormatError::too_large::<N>(&len))?;
        let bytes = reader.take(len)?;
        Ok(std::str::from_utf8(bytes).map_err(|e| Box::new(e.into()))?)
    }
}

impl<N, T: AsRef<str>> ToPacketBytes<T> for WithLen<N, &str>
where
    N: ToPacketBytes<N> + FromPrimitive + Unsigned + Display,
{
    fn to_packet(value: T, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        let value = value.as_ref();
        let len = N::from_usize(value.len())
            .ok_or_else(|| PacketFormatError::too_large::<N>(&value.len()))?;
        N::to_packet(len, packet)?;
        packet.extend_from_slice(value.as_bytes());
        Ok(())
    }
}
