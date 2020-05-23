use super::*;
use num_traits::{FromPrimitive, ToPrimitive, Unsigned};

impl<'a, N, T> FromPacketBytes<'a> for WithLen<N, Vec<T>>
where
    T: FromPacketBytes<'a>,
    N: FromPacketBytes<'a>,
    N::Output: ToPrimitive + Unsigned + Display,
{
    type Output = Vec<T::Output>;

    fn from_packet(reader: &mut PacketReader<'a>) -> Result<Self::Output, Box<PacketFormatError>> {
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
