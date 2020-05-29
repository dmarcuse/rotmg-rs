//! Structured packet definitions, representing parsed packets that can be more
//! easily manipulated.

mod definitions {
    use crate::adapters::*;
    use crate::structured::data::*;

    define_packets! {
        /// Packets sent by the ROTMG server.
        server {
            // arena packets
            ArenaDeath { cost: u32 },
            ImminentArenaWave { current_runtime: u32 },

            // pet packets
            ActivePet { instance_id: u32 },
            DeletePetMessage { pet_id: u32 },
            HatchPetMessage {
                pet_name: WithLen<u16, String>,
                pet_skin: u32,
                item_type: u32,
            },
            PetYard { typ: u32 },

            // uncategorized
            AccountList {
                account_list_id: u32,
                account_ids: WithLen<u16, Vec<WithLen<u16, String>>>,
                lock_action: u32,
            },
            AllyShoot {
                bullet_id: u8,
                owner_id: u32,
                container_type: u16,
                angle: f32,
                bard: bool,
            },
            Aoe {
                pos: WorldPosData,
                radius: f32,
                damage: u16,
                effect: u8,
                duration: f32,
                orig_type: u16,
                color: u32,
                armor_pierce: bool,
            },
            BuyResult {
                // TODO: constants
                result: i32,
                result_string: WithLen<u16, String>,
            },
            ClientStat {
                name: WithLen<u16, String>,
                value: u32,
            },
            CreateSuccess {
                object_id: u32,
                char_id: u32,
            },
            Damage {
                target_id: u32,
                effects: WithLen<u8, Vec<u8>>,
                damage_amount: u16,
                kill: bool,
                armor_pierce: bool,
                bullet_id: u8,
                object_id: u32,
            },
            Death {
                account_id: WithLen<u16, String>,
                char_id: u32,
                killed_by: WithLen<u16, String>,
                zombie_type: u32,
                zombie_id: i32,
            },
            EnemyShoot {
                bullet_id: u8,
                owner_id: u32,
                bullet_type: u8,
                starting_pos: WorldPosData,
                angle: f32,
                damage: u16,
                num_shots: Option<u8>,
                angle_inc: Option<f32>,
            },
            EvolvedPet {
                pet_id: u32,
                initial_skin: u32,
                final_skin: u32,
            },
            Failure {
                // TODO: constants
                error_id: u32,
                error_description: WithLen<u16, String>,
                error_place: WithLen<u16, String>,
                error_connection_id: WithLen<u16, String>
            },
            File {
                filename: WithLen<u16, String>,
                file: WithLen<u32, String>,
            },
            GlobalNotification {
                typ: u32,
                text: WithLen<u16, String>,
            },
            Goto {
                object_id: u32,
                pos: WorldPosData,
            },
            GuildResult {
                success: bool,
                line_builder_json: WithLen<u16, String>,
            },
            InvResult { result: i32 },
            InvitedToGuild {
                name: WithLen<u16, String>,
                guild_name: WithLen<u16, String>,
            },
            KeyInfoResponse {
                name: WithLen<u16, String>,
                description: WithLen<u16, String>,
                creator: WithLen<u16, String>,
            },
            MapInfo {
                width: u32,
                height: i32,
                name: WithLen<u16, String>,
                display_name: WithLen<u16, String>,
                fp: u32,
                background: u32,
                difficulty: u32,
                allow_player_teleport: bool,
                show_displays: bool,
                max_players: u16,
                connection_guid: WithLen<u16, String>,
                game_opened_time: u32,
                client_xml: WithLen<u16, Vec<WithLen<u32, String>>>,
                extra_xml: WithLen<u16, Vec<WithLen<u32, String>>>,
            },
            NameResult {
                success: bool,
                error_text: WithLen<u16, String>,
            },
            NewAbilityMessage {
                typ: i32,
            },
            NewTick {
                tick_id: u32,
                tick_time: u32,
                statuses: WithLen<u16, Vec<ObjectStatusData>>
            },
            Notification {
                object_id: u32,
                message: WithLen<u16, String>,
                color: u32,
            },
            PasswordPrompt { clean_password_status: i32 },
            Pic {
                width: u32,
                height: u32,
                bitmap: CaptureRemaining<Vec<u8>>,
            },
            Ping { serial: u32 },
            PlaySound {
                owner_id: u32,
                sound_id: u8,
            },
            QuestObjId { object_id: u32 },
            QuestRedeemResponse {
                ok: bool,
                message: WithLen<u16, String>,
            },
            RealmHeroesResponse {
                number_of_realm_heroes: u32
            },
            Reconnect {
                name: WithLen<u16, String>,
                host: WithLen<u16, String>,
                stats: WithLen<u16, String>,
                port: u32,
                game_id: u32,
                key_time: u32,
                is_from_arena: bool,
                key: WithLen<u16, Vec<u8>>,
            },
            ReskinUnlock {
                skin_id: u32,
                is_pet_skin: u32,
            },
            ServerPlayerShoot {
                bullet_id: u8,
                owner_id: u32,
                container_type: u32,
                starting_pos: WorldPosData,
                angle: f32,
                damage: u16,
            },
            ShowEffect {
                // TODO: constants
                effect_type: u8,
                target_object_id: u32,
                pos1: WorldPosData,
                pos2: WorldPosData,
                color: u32,
                duration: f32,
            },
            Text {
                name: WithLen<u16, String>,
                object_id: u32,
                num_stars: u32,
                bubble_time: u8,
                recipient: WithLen<u16, String>,
                text: WithLen<u16, String>,
                clean_text: WithLen<u16, String>,
                is_supporter: bool,
                star_bg: u32,
            },
            TradeAccepted {
                my_offer: WithLen<u16, Vec<bool>>,
                your_offer: WithLen<u16, Vec<bool>>,
            },
            TradeChanged { offer: WithLen<u16, Vec<bool>> },
            TradeDone {
                code: u32,
                description: WithLen<u16, String>,
            },
            TradeRequested { name: WithLen<u16, String> },
            TradeStart {
                my_items: WithLen<u16, Vec<TradeItem>>,
                your_name: WithLen<u16, String>,
                your_items: WithLen<u16, Vec<TradeItem>>,
            },
            Update {
                tiles: WithLen<u16, Vec<GroundTileData>>,
                new_objs: WithLen<u16, Vec<ObjectData>>,
                drops: WithLen<u16, Vec<i32>>,
            },
            VerifyEmail { _empty: () }
        },

        /// Packets sent by the ROTMG client.
        client {
            // arena packets
            EnterArena { currency: u32 },
            QuestRedeem {
                quest_id: WithLen<u16, String>,
                item: u32,
                slots: WithLen<u16, Vec<SlotObjectData>>,
            },

            // pet packets
            PetUpgradeRequest {
                pet_trans_type: u8,
                pid1: u32,
                pid2: u32,
                object_id: u32,
                payment_trans_type: u8,
                slots: WithLen<u16, Vec<SlotObjectData>>,
            },
            ReskinPet {
                pet_instance_id: u32,
                picked_new_pet_type: u32,
                slot: SlotObjectData,
            },

            // uncategorized
            AcceptTrade {
                my_offer: WithLen<u16, Vec<bool>>,
                your_offer: WithLen<u16, Vec<bool>>,
            },
            ActivePetUpdateRequest {
                command_type: u8,
                instance_id: u32,
            },
            AoeAck {
                time: u32,
                position: WorldPosData,
            },
            Buy {
                object_id: u32,
                quantity: u32,
            },
            CancelTrade { _empty: () },
            ChangeGuildRank {
                name: WithLen<u16, String>,
                guild_rank: u32,
            },
            ChangePetSkin {
                pet_id: u32,
                skin_type: u32,
                currency: u32,
            },
            ChangeTrade { offer: WithLen<u16, Vec<bool>> },
            CheckCredits { _empty: () },
            ChooseName { name: WithLen<u16, String> },
            Create {
                class_type: u16,
                skin_type: u16,
                is_challenger: bool,
            },
            CreateGuild { name: WithLen<u16, String> },
            EditAccountList {
                account_list_id: u32,
                add: bool,
                object_id: u32,
            },
            EnemyHit {
                time: u32,
                bullet_id: u8,
                target_id: u32,
                kill: bool,
            },
            Escape { empty: () },
            GoToQuestRoom { empty: () },
            GotoAck { time: u32 },
            GroundDamage {
                time: u32,
                position: WorldPosData,
            },
            GuildInvite { name: WithLen<u16, String> },
            GuildRemove { name: WithLen<u16, String> },
            Hello {
                build_version: WithLen<u16, String>,
                game_id: u32,
                guid: WithLen<u16, String>,
                rand1: u32,
                password: WithLen<u16, String>,
                rand2: u32,
                secret: WithLen<u16, String>,
                key_time: u32,
                key: WithLen<u16, Vec<u8>>,
                map_json: WithLen<u32, String>,
                entry_tag: WithLen<u16, String>,
                game_net: WithLen<u16, String>,
                game_net_user_id: WithLen<u16, String>,
                play_platform: WithLen<u16, String>,
                platform_token: WithLen<u16, String>,
                user_token: WithLen<u16, String>,
                unknown: WithLen<u16, String>,
                previous_connection_guid: WithLen<u16, String>,
            },
            InvDrop { slot_object: SlotObjectData },
            InvSwap {
                time: u32,
                position: WorldPosData,
                slot1: SlotObjectData,
                slot2: SlotObjectData,
            },
            JoinGuild { guild_name: WithLen<u16, String> },
            KeyInfoRequest { item_type: u32 },
            Load {
                char_id: u32,
                is_from_arena: bool,
                is_challenger: bool,
            },
            Move {
                tick_id: u32,
                time: u32,
                new_position: WorldPosData,
                records: WithLen<u16, Vec<WorldPosData>>,
            },
            OtherHit {
                time: u32,
                bullet_id: u8,
                object_id: u32,
                target_id: u32,
            },
            PlayerHit {
                bullet_id: u8,
                object_id: u32,
            },
            PlayerShoot {
                time: u32,
                bullet_id: u8,
                container_type: u16,
                starting_pos: WorldPosData,
                angle: f32,
                speed_mult: u16,
                life_mult: u16,
            },
            PlayerText { text: WithLen<u16, String> },
            Pong {
                serial: u32,
                time: u32,
            },
            RequestTrade { name: WithLen<u16, String> },
            ResetDailyQuests { empty: () },
            Reskin { skin_id: u32 },
            SetCondition {
                condition_effect: u8,
                condition_duration: f32,
            },
            ShootAck { time: u32 },
            SquareHit {
                time: u32,
                bullet_id: u8,
                object_id: u32,
            },
            Teleport { object_id: u32 },
            UseItem {
                time: u32,
                slot: SlotObjectData,
                item_use_pos: WorldPosData,
                use_type: u8,
            },
            UsePortal { object_id: u32 },
        },
    }
}

pub use definitions::{client, server, PacketType};
