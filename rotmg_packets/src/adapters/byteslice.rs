use super::*;
use num_traits::{FromPrimitive, ToPrimitive, Unsigned};

impl<'a, N> FromPacketBytes<'a> for WithLen<N, &'a [u8]>
where
    N: FromPacketBytes<'a>,
    N::Output: ToPrimitive + Unsigned + Display,
{
    type Output = &'a [u8];

    fn from_packet(reader: &mut PacketReader<'a>) -> Result<Self::Output, Box<PacketFormatError>> {
        let len = N::from_packet(reader)?;
        let len = len
            .to_usize()
            .ok_or_else(|| PacketFormatError::too_large::<N>(&len))?;
        reader.take(len)
    }
}

impl<N> ToPacketBytes<&[u8]> for WithLen<N, &[u8]>
where
    N: ToPacketBytes<N> + FromPrimitive + Unsigned + Display,
{
    fn to_packet(value: &[u8], packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        let len = N::from_usize(value.len())
            .ok_or_else(|| PacketFormatError::too_large::<N>(&value.len()))?;
        N::to_packet(len, packet)?;
        packet.extend_from_slice(value);
        Ok(())
    }
}

impl<'a> FromPacketBytes<'a> for CaptureRemaining<&'a [u8]> {
    type Output = &'a [u8];

    fn from_packet(reader: &mut PacketReader<'a>) -> Result<Self::Output, Box<PacketFormatError>> {
        Ok(reader.take_all())
    }
}

impl ToPacketBytes<&[u8]> for CaptureRemaining<&[u8]> {
    fn to_packet(value: &[u8], packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        packet.extend_from_slice(value);
        Ok(())
    }
}
