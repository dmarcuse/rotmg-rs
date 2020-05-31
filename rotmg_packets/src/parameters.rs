use crate::PacketMappings;

/// Constant data used by the ROTMG protocol.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameters {
    /// Mappings between internal and real packet IDs.
    pub packets: PacketMappings,

    /// Network RC4 keys, in hexadecimal.
    pub rc4: String,

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
