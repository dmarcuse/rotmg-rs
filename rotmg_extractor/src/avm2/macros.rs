macro_rules! flag_enum {
    (
        $name:ident {
            $(
                $flag:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
        )]
        #[repr(u8)]
        pub enum $name {
            $(
                $flag = $value
            ),*
        }

        impl $name {
            const VALID: [Option<$name>; 256] = {
                let mut arr = [None; 256];

                $(
                    arr[$name::$flag as usize] = Some($name::$flag);
                )*

                arr
            };

            pub fn from_u8(byte: u8) -> Result<Self, $crate::avm2::ParseError> {
                Self::VALID[byte as usize]
                    .ok_or($crate::avm2::ParseError::InvalidFlag {
                        value: byte,
                        typ: std::any::type_name::<Self>(),
                    })
            }
        }

        impl $crate::avm2::Parse for $name {
            fn parse_avm2(input: &mut $crate::avm2::Reader) -> Result<Self, $crate::avm2::ParseError> {
                Self::from_u8(input.take(1)?[0])
            }
        }
    };
}

macro_rules! data_struct {
    (
        $name:ident {
            $(
                $field:ident : $type:ty
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $(
                $field : $type
            ),*
        }

        impl $crate::avm2::Parse for $name {
            fn parse_avm2(buf: &mut $crate::avm2::Reader) -> Result<Self, $crate::avm2::ParseError> {
                $(
                    let $field = $crate::avm2::Parse::parse_avm2(buf)?;
                )*

                Ok(Self {
                    $(
                        $field
                    ),*
                })
            }
        }
    }
}
