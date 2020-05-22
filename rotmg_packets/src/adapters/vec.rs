use super::*;
use std::convert::{TryFrom, TryInto};
use std::num::TryFromIntError;

impl<'a, N, T> FromPacketBytes<'a> for LengthPrefixed<N, Vec<T>>
where
    T: FromPacketBytes<'a>,
    N: FromPacketBytes<'a>,
    N::Output: Into<usize>,
{
    type Output = Vec<T::Output>;

    fn from_packet(reader: &mut PacketReader<'a>) -> Result<Self::Output, Box<PacketFormatError>> {
        let len = N::from_packet(reader)?.into();
        (0..len)
            .map(|_| T::from_packet(reader))
            .collect::<Result<Vec<_>, _>>()
    }
}

impl<N, I, T> ToPacketBytes<I> for LengthPrefixed<N, Vec<T>>
where
    N: ToPacketBytes<N> + TryFrom<usize, Error = TryFromIntError>,
    I: IntoIterator,
    T: ToPacketBytes<I::Item>,
    I::IntoIter: ExactSizeIterator,
{
    fn to_packet(value: I, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        let values = value.into_iter();
        let len = values
            .len()
            .try_into()
            .map_err(|_| PacketFormatError::too_large::<N>(values.len()))?;
        N::to_packet(len, packet)?;
        for value in values {
            T::to_packet(value, packet)?;
        }
        Ok(())
    }
}
