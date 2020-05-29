use super::*;
use num_traits::{FromPrimitive, ToPrimitive, Unsigned};

impl<N, T> FromPacketBytes for WithLen<N, Vec<T>>
where
    T: FromPacketBytes,
    N: FromPacketBytes,
    N::Output: ToPrimitive + Unsigned + Display,
{
    type Output = Vec<T::Output>;

    fn from_packet(reader: &mut PacketReader) -> Result<Self::Output, Box<PacketFormatError>> {
        let len = N::from_packet(reader)?;
        let len = len
            .to_usize()
            .ok_or_else(|| PacketFormatError::too_large::<N>(&len))?;
        (0..len)
            .map(|_| T::from_packet(reader))
            .collect::<Result<Vec<_>, _>>()
    }
}

impl<N, I, T> ToPacketBytes<I> for WithLen<N, Vec<T>>
where
    N: ToPacketBytes<N> + FromPrimitive + Unsigned + Display,
    I: IntoIterator,
    T: ToPacketBytes<I::Item>,
    I::IntoIter: ExactSizeIterator,
{
    fn to_packet(value: I, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        let values = value.into_iter();
        let len = N::from_usize(values.len())
            .ok_or_else(|| PacketFormatError::too_large::<N>(&values.len()))?;
        N::to_packet(len, packet)?;
        for value in values {
            T::to_packet(value, packet)?;
        }
        Ok(())
    }
}

impl FromPacketBytes for CaptureRemaining<Vec<u8>> {
    type Output = Vec<u8>;

    fn from_packet(reader: &mut PacketReader) -> Result<Self::Output, Box<PacketFormatError>> {
        Ok(reader.take_all().to_vec())
    }
}

impl<T: AsRef<[u8]>> ToPacketBytes<T> for CaptureRemaining<Vec<u8>> {
    fn to_packet(value: T, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        packet.extend_from_slice(value.as_ref());
        Ok(())
    }
}
