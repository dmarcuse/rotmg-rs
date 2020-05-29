use crate::structured::packets::PacketType;
use bimap::BiHashMap;
use std::collections::HashSet;

/// Mappings of internal `PacketType`s to ROTMG packet IDs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PacketMappings(pub BiHashMap<PacketType, u8>);

impl PacketMappings {
    /// Create a new `PacketMappings` from the given mappings.
    pub fn new(mappings: impl IntoIterator<Item = (PacketType, u8)>) -> Self {
        Self(mappings.into_iter().collect())
    }

    /// Get all internal packet types that don't have an associated mapping.
    pub fn get_unmapped(&self) -> HashSet<PacketType> {
        let mut unmapped = PacketType::VALUES.iter().copied().collect::<HashSet<_>>();
        self.0.left_values().for_each(|t| {
            unmapped.remove(t);
        });
        unmapped
    }

    /// Given an internal `PacketType`, get the associated ROTMG packet ID.
    pub fn to_game(&self, typ: PacketType) -> Option<u8> {
        self.0.get_by_left(&typ).copied()
    }

    /// Given a ROTMG packet ID, get the associated internal `PacketType`.
    pub fn to_internal(&self, id: u8) -> Option<PacketType> {
        self.0.get_by_right(&id).copied()
    }
}
