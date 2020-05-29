//! Data types used in packets.

use crate::adapters::*;

// Define trivial types
define_packet_data! {
    GroundTileData {
        x: i16,
        y: i16,
        tile_type: u16,
    },
    MoveRecord {
        time: u32,
        x: f32,
        y: f32,
    },
    ObjectData {
        object_type: u16,
        status: ObjectStatusData,
    },
    ObjectStatusData {
        object_id: u32,
        pos: WorldPosData,
        stats: WithLen<u16, Vec<StatData>>,
    },
    SlotObjectData {
        object_id: u32,
        slot_id: u8,
        object_type: u32,
    },
    TradeItem {
        item: u32,
        slot_type: u32,
        tradeable: bool,
        included: bool,
    },
    WorldPosData {
        x: f32,
        y: f32,
    },
}

define_stat_types! {
    MAX_HP_STAT:i32 = 0,
    HP_STAT: i32 = 1,
    SIZE_STAT: i32 = 2,
    MAX_MP_STAT: i32 = 3,
    MP_STAT: i32 = 4,
    NEXT_LEVEL_EXP_STAT: i32 = 5,
    EXP_STAT: i32 = 6,
    LEVEL_STAT: i32 = 7,
    ATTACK_STAT: i32 = 20,
    DEFENSE_STAT: i32 = 21,
    SPEED_STAT: i32 = 22,
    INVENTORY_0_STAT: i32 = 8,
    INVENTORY_1_STAT: i32 = 9,
    INVENTORY_2_STAT: i32 = 10,
    INVENTORY_3_STAT: i32 = 11,
    INVENTORY_4_STAT: i32 = 12,
    INVENTORY_5_STAT: i32 = 13,
    INVENTORY_6_STAT: i32 = 14,
    INVENTORY_7_STAT: i32 = 15,
    INVENTORY_8_STAT: i32 = 16,
    INVENTORY_9_STAT: i32 = 17,
    INVENTORY_10_STAT: i32 = 18,
    INVENTORY_11_STAT: i32 = 19,
    VITALITY_STAT: i32 = 26,
    WISDOM_STAT: i32 = 27,
    DEXTERITY_STAT: i32 = 28,
    CONDITION_STAT: i32 = 29,
    NUM_STARS_STAT: i32 = 30,
    NAME_STAT: String = 31,
    TEX1_STAT: i32 = 32,
    TEX2_STAT: i32 = 33,
    MERCHANDISE_TYPE_STAT: i32 = 34,
    CREDITS_STAT: i32 = 35,
    MERCHANDISE_PRICE_STAT: i32 = 36,
    ACTIVE_STAT: i32 = 37,
    ACCOUNT_ID_STAT: String = 38,
    FAME_STAT: i32 = 39,
    MERCHANDISE_CURRENCY_STAT: i32 = 40,
    CONNECT_STAT: i32 = 41,
    MERCHANDISE_COUNT_STAT: i32 = 42,
    MERCHANDISE_MINS_LEFT_STAT: i32 = 43,
    MERCHANDISE_DISCOUNT_STAT: i32 = 44,
    MERCHANDISE_RANK_REQ_STAT: i32 = 45,
    MAX_HP_BOOST_STAT: i32 = 46,
    MAX_MP_BOOST_STAT: i32 = 47,
    ATTACK_BOOST_STAT: i32 = 48,
    DEFENSE_BOOST_STAT: i32 = 49,
    SPEED_BOOST_STAT: i32 = 50,
    VITALITY_BOOST_STAT: i32 = 51,
    WISDOM_BOOST_STAT: i32 = 52,
    DEXTERITY_BOOST_STAT: i32 = 53,
    OWNER_ACCOUNT_ID_STAT: String = 54,
    RANK_REQUIRED_STAT: i32 = 55,
    NAME_CHOSEN_STAT: i32 = 56,
    CURR_FAME_STAT: i32 = 57,
    NEXT_CLASS_QUEST_FAME_STAT: i32 = 58,
    LEGENDARY_RANK_STAT: i32 = 59,
    SINK_LEVEL_STAT: i32 = 60,
    ALT_TEXTURE_STAT: i32 = 61,
    GUILD_NAME_STAT: String = 62,
    GUILD_RANK_STAT: i32 = 63,
    BREATH_STAT: i32 = 64,
    XP_BOOSTED_STAT: i32 = 65,
    XP_TIMER_STAT: i32 = 66,
    LD_TIMER_STAT: i32 = 67,
    LT_TIMER_STAT: i32 = 68,
    HEALTH_POTION_STACK_STAT: i32 = 69,
    MAGIC_POTION_STACK_STAT: i32 = 70,
    BACKPACK_0_STAT: i32 = 71,
    BACKPACK_1_STAT: i32 = 72,
    BACKPACK_2_STAT: i32 = 73,
    BACKPACK_3_STAT: i32 = 74,
    BACKPACK_4_STAT: i32 = 75,
    BACKPACK_5_STAT: i32 = 76,
    BACKPACK_6_STAT: i32 = 77,
    BACKPACK_7_STAT: i32 = 78,
    HASBACKPACK_STAT: i32 = 79,
    TEXTURE_STAT: i32 = 80,
    PET_INSTANCEID_STAT: i32 = 81,
    PET_NAME_STAT: String = 82,
    PET_TYPE_STAT: i32 = 83,
    PET_RARITY_STAT: i32 = 84,
    PET_MAXABILITYPOWER_STAT: i32 = 85,
    PET_FAMILY_STAT: i32 = 86,
    PET_FIRSTABILITY_POINT_STAT: i32 = 87,
    PET_SECONDABILITY_POINT_STAT: i32 = 88,
    PET_THIRDABILITY_POINT_STAT: i32 = 89,
    PET_FIRSTABILITY_POWER_STAT: i32 = 90,
    PET_SECONDABILITY_POWER_STAT: i32 = 91,
    PET_THIRDABILITY_POWER_STAT: i32 = 92,
    PET_FIRSTABILITY_TYPE_STAT: i32 = 93,
    PET_SECONDABILITY_TYPE_STAT: i32 = 94,
    PET_THIRDABILITY_TYPE_STAT: i32 = 95,
    NEW_CON_STAT: i32 = 96,
    FORTUNE_TOKEN_STAT: i32 = 97,
    SUPPORTER_POINTS_STAT: i32 = 98,
    SUPPORTER_STAT: i32 = 99,
    CHALLENGER_STARBG_STAT: i32 = 100,
    PROJECTILE_SPEED_MULT: i32 = 102,
    PROJECTILE_LIFE_MULT: i32 = 103,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatData {
    String(StatType, String),
    Integer(StatType, i32),
}

impl FromPacketBytes for StatData {
    type Output = StatData;

    fn from_packet(reader: &mut PacketReader) -> Result<Self::Output, Box<PacketFormatError>> {
        let typ = u8::from_packet(reader)?;
        let typ = StatType::from_byte(typ)
            .ok_or_else(|| Box::new(PacketFormatError::UnknownStatType(typ)))?;

        if typ.is_string() {
            <WithLen<u16, String>>::from_packet(reader).map(|s| StatData::String(typ, s))
        } else {
            i32::from_packet(reader).map(|i| StatData::Integer(typ, i))
        }
    }
}

impl<T: Into<StatData>> ToPacketBytes<T> for StatData {
    fn to_packet(value: T, packet: &mut Vec<u8>) -> Result<(), Box<PacketFormatError>> {
        let value = value.into();
        match value {
            StatData::Integer(typ, i) => {
                u8::to_packet(typ.to_byte(), packet)?;
                i32::to_packet(i, packet)?;
                Ok(())
            }
            StatData::String(typ, s) => {
                u8::to_packet(typ.to_byte(), packet)?;
                <WithLen<u16, String>>::to_packet(s, packet)?;
                Ok(())
            }
        }
    }
}
