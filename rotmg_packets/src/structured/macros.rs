/// Define adapter implementations for a packet data type.
macro_rules! define_packet_data_adapter {
    ( $name:ident ( @ManualAdapters ) $rest:tt ) => {};
    ( $name:ident { $( $fname:ident : $fty:ty ),* $(,)? }) => {
        impl<'a> $crate::adapters::FromPacketBytes<'a> for $name <'a> {
            type Output = $name <'a>;

            fn from_packet(
                reader: &mut $crate::adapters::PacketReader<'a>
            ) -> Result<Self::Output, Box<$crate::adapters::PacketFormatError>> {
                $( let $fname = <$fty as $crate::adapters::FromPacketBytes<'a>>::from_packet(reader)?; )*
                Ok(Self { $( $fname ),* })
            }
        }

        impl<'a, T: Into<$name<'a>>>
            $crate::adapters::ToPacketBytes<T> for $name <'a> {
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

/// Define a packet data type and associated adapters.
macro_rules! define_packet_data {
    (
        $(
            $( #[ $attrs:meta ] )*
            $name:ident
                $( ( @ $arg:ident ) )?
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
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name <'a> {
                $(
                    $( #[ $fattrs ] )*
                    pub $fname : <$fty as $crate::adapters::FromPacketBytes<'a>>::Output
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u8)]
        #[allow(non_camel_case_types)]
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
            pub fn is_string(self) -> bool {
                Self::STRING_TYPES[self as u8 as usize]
            }
        }
    };
}
