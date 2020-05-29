use super::*;
use std::convert::TryInto;
use std::mem::size_of;

macro_rules! numeric_impls {
    ( $( $type:ty ),* $(,)? ) => {
        $(
            impl FromPacketBytes for $type {
                type Output = $type;

                fn from_packet(reader: &mut PacketReader) -> Result<$type, Box<PacketFormatError>> {
                    let bytes = reader.take(size_of::<$type>())?;
                    Ok(<$type>::from_be_bytes(bytes.try_into().unwrap()))
                }
            }

            impl<T: Into<$type>> ToPacketBytes<T> for $type {
                fn to_packet(value: T, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
                    let value = value.into();
                    packet.extend_from_slice(&value.to_be_bytes());
                    Ok(())
                }
            }
        )*
    };
}

numeric_impls! {
    u8, u16, u32, u64,
    i8, i16, i32, i64,
    f32, f64
}

impl FromPacketBytes for bool {
    type Output = bool;

    fn from_packet(reader: &mut PacketReader) -> Result<bool, Box<PacketFormatError>> {
        reader.take(1).map(|b| b != [0])
    }
}

impl<T: Into<bool>> ToPacketBytes<T> for bool {
    fn to_packet(value: T, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        packet.push(value.into() as u8);
        Ok(())
    }
}
