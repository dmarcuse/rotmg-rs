use super::*;

impl FromPacketBytes<'_> for () {
    type Output = ();

    fn from_packet(_: &mut PacketReader<'_>) -> Result<(), Box<PacketFormatError>> {
        Ok(())
    }
}

impl ToPacketBytes<()> for () {
    fn to_packet(_: (), _: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        Ok(())
    }
}
