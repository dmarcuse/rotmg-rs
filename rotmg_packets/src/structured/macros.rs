/// Define adapter implementations for a packet data type.
macro_rules! define_packet_data_adapter {
    ( $name:ident ( @ManualAdapters ) $rest:tt ) => {};
    ( $name:ident { $( $fname:ident : $fty:ty ),* $(,)? }) => {
        impl $crate::adapters::FromPacketBytes for $name {
            type Output = $name;

            fn from_packet(
                reader: &mut $crate::adapters::PacketReader
            ) -> Result<Self::Output, Box<$crate::adapters::PacketFormatError>> {
                $( let $fname = <$fty as $crate::adapters::FromPacketBytes>::from_packet(reader)?; )*
                Ok(Self { $( $fname ),* })
            }
        }

        impl<T: Into<$name>>
            $crate::adapters::ToPacketBytes<T> for $name {
            fn to_packet(
                value: T, packet: &mut Vec<u8>
            ) -> Result<(), Box<$crate::adapters::PacketFormatError>> {
                let $name { $( $fname ),* } = value.into();
                $( <$fty as $crate::adapters::ToPacketBytes<_>>::to_packet($fname, packet)?; )*
                Ok(())
            }
        }
    }
}

/// Define packet data types and associated adapters.
macro_rules! define_packet_data {
    (
        $(
            $( #[ $attrs:meta ] )*
            $name:ident $( ( @ $arg:ident ) )?
            {
                $(
                    $( #[ $fattrs:meta ] )*
                    $fname:ident : $fty:ty
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        $(
            $( #[ $attrs ] )*
            #[derive(Debug, Clone, PartialEq, Default)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[allow(missing_docs)]
            pub struct $name {
                $(
                    $( #[ $fattrs ] )*
                    pub $fname : <$fty as $crate::adapters::FromPacketBytes>::Output
                ),*
            }

            define_packet_data_adapter! {
                $name $( ( @ $arg ) )* { $( $fname : $fty ),* }
            }
        )*
    };
}

macro_rules! is_string {
    ( i32 ) => {
        false
    };
    ( String ) => {
        true
    };
}

/// Helper macro for defining stat types contained in `StatData` instances.
macro_rules! define_stat_types {
    ( $( $name:ident : $type:ident = $value:literal ),* $(,)? ) => {
        /// The type of a stat specified by a `StatData`.
        ///
        /// Most stat types are associated with integer data, but some are
        /// associated with strings - such as `StatType::NAME_STAT`. This can be
        /// checked with `StatType::is_string`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[repr(u8)]
        #[allow(non_camel_case_types, missing_docs)]
        pub enum StatType {
            $( $name = $value ),*
        }

        impl StatType {
            const VALID_TYPES: [Option<StatType>; 256] = {
                let mut array = [ None; 256 ];
                $( array[$value] = Some(StatType::$name); )*
                array
            };

            const STRING_TYPES: [bool; 256] = {
                let mut array = [ false; 256 ];
                $( array[$value] = is_string!($type); )*
                array
            };

            /// Convert a byte to a `StatType`, returning `None` on invalid
            /// types.
            pub const fn from_byte(byte: u8) -> Option<Self> {
                Self::VALID_TYPES[byte as usize]
            }

            /// Convert this `StatType` to a byte.
            pub const fn to_byte(self) -> u8 {
                self as u8
            }

            /// Check whether the given stat type is a string.
            pub const fn is_string(self) -> bool {
                Self::STRING_TYPES[self as u8 as usize]
            }
        }
    };
}

/// Define packet types
macro_rules! define_packets {
    (
        $(
            $( #[ $mattrs:meta ] )*
            $module:ident {
                $(
                    $( #[ $attrs:meta ] )*
                    $name:ident $( ( @ $arg:ident ) )? $body:tt
                ),* $(,)?
            }
        ),* $(,)?

    ) => {
        // define the packet structures and adapters
        $(
            $( #[ $mattrs ] )*
            pub mod $module {
                #![allow(unused_imports)]
                use super::*;

                define_packet_data! {
                    $(
                        $( #[ $attrs ] )*
                        $name $( ( @ $arg ) )? $body
                    ),*
                }

                // packet trait implementations
                $(
                    impl sealed::Sealed for $name {}

                    impl StructuredPacket for $name {
                        const TYPE: PacketType = PacketType::$name;
                    }

                    impl AnyPacket for $name {
                        fn packet_type(&self) -> PacketType {
                            PacketType::$name
                        }

                        fn into_raw(self: Box<Self>) -> Result<Box<RawPacket>, Box<PacketFormatError>> {
                            let mut packet = vec![0u8; 4];
                            packet.push(PacketType::$name as u8);
                            $name::to_packet(*self, &mut packet)?;

                            let len = (packet.len() as u32).to_be_bytes();
                            packet[..4].copy_from_slice(&len);

                            Ok(RawPacket::from_box(packet.into_boxed_slice()).unwrap())
                        }
                    }
                )*
            }
        )*

        // define the PacketType enum
        /// A ROTMG packet type.
        ///
        /// This represents an internal equivalent to ROTMG packet IDs. However,
        /// while they're represented as a byte, they aren't equal to the actual
        /// ROTMG packet IDs. Instead, a `PacketMappings` instance should be
        /// used to convert between the two.
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[allow(missing_docs)]
        pub enum PacketType {
            $( $(
                $name ,
            )* )*
        }

        impl PacketType {
            /// All values of the `PacketType` enum.
            pub const VALUES: &'static [PacketType] = &[
                $( $( Self :: $name, )* )*
            ];

            pub(crate) fn parse_bytes(
                self,
                reader: &mut PacketReader
            ) -> Result<Box<dyn AnyPacket>, Box<PacketFormatError>> {
                match self {
                    $( $(
                        PacketType::$name => {
                            $module::$name::from_packet(reader).map(|p| -> Box<dyn AnyPacket> {
                                Box::new(p)
                            })
                        },
                    )* )*
                }
            }
        }
    };
}
