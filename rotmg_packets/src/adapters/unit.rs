use super::*;

impl FromPacketBytes for () {
    type Output = ();

    fn from_packet(_: &mut PacketReader) -> Result<(), Box<PacketFormatError>> {
        Ok(())
    }
}

impl ToPacketBytes<()> for () {
    fn to_packet(_: (), _: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        Ok(())
    }
}
