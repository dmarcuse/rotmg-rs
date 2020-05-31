use crate::PacketMappings;

/// Basic constants used by the ROTMG client.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BasicParameters {
    /// Game build version.
    pub version: String,

    /// ROTMG protocol port.
    pub port: u16,

    /// Game ID for the tutorial.
    pub tutorial_game_id: i32,

    /// Game ID for the nexus.
    pub nexus_game_id: i32,

    /// Game ID for a random realm.
    pub random_game_id: i32,
}

/// Constant data used by the ROTMG protocol.
///
/// This type combines `BasicParameters` with networking data to provide an
/// all-in-one structure for full network protocol support.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameters {
    /// Mappings between internal and real packet IDs.
    pub packets: PacketMappings,

    /// Network RC4 keys, in hexadecimal.
    pub rc4: String,

    /// Other basic parameters.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub basic: BasicParameters,
}
