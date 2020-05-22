use super::*;

impl<'a, T: FromPacketBytes<'a>> FromPacketBytes<'a> for Option<T> {
    type Output = Option<T::Output>;

    fn from_packet(reader: &mut PacketReader<'a>) -> Result<Self::Output, Box<PacketFormatError>> {
        if reader.is_empty() {
            Ok(None)
        } else {
            T::from_packet(reader).map(Some)
        }
    }
}

impl<T: ToPacketBytes<U>, U> ToPacketBytes<Option<U>> for Option<T> {
    fn to_packet(value: Option<U>, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        if let Some(value) = value {
            T::to_packet(value, packet)
        } else {
            Ok(())
        }
    }
}
