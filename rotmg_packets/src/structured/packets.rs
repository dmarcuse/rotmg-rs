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
                pet_name: WithLen<u16, &'a str>,
                pet_skin: u32,
                item_type: u32,
            },
            PetYard { typ: u32 },

            // uncategorized
            AccountList {
                account_list_id: u32,
                account_ids: WithLen<u16, Vec<WithLen<u16, &'a str>>>,
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
                pos: WorldPosData<'a>,
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
                result_string: WithLen<u16, &'a str>,
            },
            ClientStat {
                name: WithLen<u16, &'a str>,
                value: u32,
            },
            CreateSuccess {
                object_id: u32,
                char_id: u32,
            },
            Damage {
                target_id: u32,
                effects: WithLen<u8, &'a [u8]>,
                damage_amount: u16,
                kill: bool,
                armor_pierce: bool,
                bullet_id: u8,
                object_id: u32,
            },
            Death {
                account_id: WithLen<u16, &'a str>,
                char_id: u32,
                killed_by: WithLen<u16, &'a str>,
                zombie_type: u32,
                zombie_id: i32,
            },
            EnemyShoot {
                bullet_id: u8,
                owner_id: u32,
                bullet_type: u8,
                starting_pos: WorldPosData<'a>,
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
                error_description: WithLen<u16, &'a str>,
                error_place: WithLen<u16, &'a str>,
                error_connection_id: WithLen<u16, &'a str>
            },
            File {
                filename: WithLen<u16, &'a str>,
                file: WithLen<u32, &'a str>,
            },
            GlobalNotification {
                typ: u32,
                text: WithLen<u16, &'a str>,
            },
            Goto {
                object_id: u32,
                pos: WorldPosData<'a>,
            },
            GuildResult {
                success: bool,
                line_builder_json: WithLen<u16, &'a str>,
            },
            InvResult { result: i32 },
            InvitedToGuild {
                name: WithLen<u16, &'a str>,
                guild_name: WithLen<u16, &'a str>,
            },
            KeyInfoResponse {
                name: WithLen<u16, &'a str>,
                description: WithLen<u16, &'a str>,
                creator: WithLen<u16, &'a str>,
            },
            MapInfo {
                width: u32,
                height: i32,
                name: WithLen<u16, &'a str>,
                display_name: WithLen<u16, &'a str>,
                fp: u32,
                background: u32,
                difficulty: u32,
                allow_player_teleport: bool,
                show_displays: bool,
                max_players: u16,
                connection_guid: WithLen<u16, &'a str>,
                game_opened_time: u32,
                client_xml: WithLen<u16, &'a str>,
                extra_xml: WithLen<u16, &'a str>,
            },
            NameResult {
                success: bool,
                error_text: WithLen<u16, &'a str>,
            },
            NewAbilityMessage {
                typ: i32,
            },
            NewTick {
                tick_id: u32,
                tick_time: u32,
                statuses: WithLen<u16, Vec<ObjectStatusData<'a>>>
            },
            Notification {
                object_id: u32,
                message: WithLen<u16, &'a str>,
                color: u32,
            },
            PasswordPrompt { clean_password_status: i32 },
            Pic {
                width: u32,
                height: u32,
                bitmap: CaptureRemaining<&'a [u8]>,
            },
            Ping { serial: u32 },
            PlaySound {
                owner_id: u32,
                sound_id: u8,
            },
            QuestObjId { object_id: u32 },
            QuestRedeemResponse {
                ok: bool,
                message: WithLen<u16, &'a str>,
            },
            RealmHeroesResponse {
                number_of_realm_heroes: u32
            },
            Reconnect {
                name: WithLen<u16, &'a str>,
                host: WithLen<u16, &'a str>,
                stats: WithLen<u16, &'a str>,
                port: u32,
                game_id: u32,
                key_time: u32,
                is_from_arena: bool,
                key: WithLen<u16, &'a [u8]>,
            },
            ReskinUnlock {
                skin_id: u32,
                is_pet_skin: u32,
            },
            ServerPlayerShoot {
                bullet_id: u8,
                owner_id: u32,
                container_type: u32,
                starting_pos: WorldPosData<'a>,
                angle: f32,
                damage: u16,
            },
            ShowEffect {
                // TODO: constants
                effect_type: u8,
                target_object_id: u32,
                pos1: WorldPosData<'a>,
                pos2: WorldPosData<'a>,
                color: u32,
                duration: f32,
            },
            Text {
                name: WithLen<u16, &'a str>,
                object_id: u32,
                num_stars: u32,
                bubble_time: u8,
                recipient: WithLen<u16, &'a str>,
                text: WithLen<u16, &'a str>,
                clean_text: WithLen<u16, &'a str>,
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
                description: WithLen<u16, &'a str>,
            },
            TradeRequested { name: WithLen<u16, &'a str> },
            TradeStart {
                my_items: WithLen<u16, Vec<TradeItem<'a>>>,
                your_name: WithLen<u16, &'a str>,
                your_items: WithLen<u16, Vec<TradeItem<'a>>>,
            },
            Update {
                tiles: WithLen<u16, Vec<GroundTileData<'a>>>,
                new_objs: WithLen<u16, Vec<ObjectData<'a>>>,
                drops: WithLen<u16, Vec<i32>>,
            },
            VerifyEmail { _empty: () }
        },

        /// Packets sent by the ROTMG client.
        client {
            // arena packets
            EnterArena { currency: u32 },
            QuestRedeem {
                quest_id: WithLen<u16, &'a str>,
                item: u32,
                slots: WithLen<u16, Vec<SlotObjectData<'a>>>,
            },

            // pet packets
            PetUpgradeRequest {
                pet_trans_type: u8,
                pid1: u32,
                pid2: u32,
                object_id: u32,
                payment_trans_type: u8,
                slots: WithLen<u16, Vec<SlotObjectData<'a>>>,
            },
            ReskinPet {
                pet_instance_id: u32,
                picked_new_pet_type: u32,
                slot: SlotObjectData<'a>,
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
                position: WorldPosData<'a>,
            },
            Buy {
                object_id: u32,
                quantity: u32,
            },
            CancelTrade { _empty: () },
            ChangeGuildRank {
                name: WithLen<u16, &'a str>,
                guild_rank: u32,
            },
            ChangePetSkin {
                pet_id: u32,
                skin_type: u32,
                currency: u32,
            },
            ChangeTrade { offer: WithLen<u16, Vec<bool>> },
            CheckCredits { _empty: () },
            ChooseName { name: WithLen<u16, &'a str> },
            Create {
                class_type: u16,
                skin_type: u16,
                is_challenger: bool,
            },
            CreateGuild { name: WithLen<u16, &'a str> },
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
                position: WorldPosData<'a>,
            },
            GuildInvite { name: WithLen<u16, &'a str> },
            GuildRemove { name: WithLen<u16, &'a str> },
            Hello {
                build_version: WithLen<u16, &'a str>,
                game_id: u32,
                guid: WithLen<u16, &'a str>,
                rand1: u32,
                password: WithLen<u16, &'a str>,
                rand2: u32,
                secret: WithLen<u16, &'a str>,
                key_time: u32,
                key: WithLen<u16, &'a [u8]>,
                map_json: WithLen<u32, &'a str>,
                entry_tag: WithLen<u16, &'a str>,
                game_net: WithLen<u16, &'a str>,
                game_net_user_id: WithLen<u16, &'a str>,
                play_platform: WithLen<u16, &'a str>,
                platform_token: WithLen<u16, &'a str>,
                user_token: WithLen<u16, &'a str>,
                unknown: WithLen<u16, &'a str>,
                previous_connection_guid: WithLen<u16, &'a str>,
            },
            InvDrop { slot_object: SlotObjectData<'a> },
            InvSwap {
                time: u32,
                position: WorldPosData<'a>,
                slot1: SlotObjectData<'a>,
                slot2: SlotObjectData<'a>,
            },
            JoinGuild { guild_name: WithLen<u16, &'a str> },
            KeyInfoRequest { item_type: u32 },
            Load {
                char_id: u32,
                is_from_arena: bool,
                is_challenger: bool,
            },
            Move {
                tick_id: u32,
                time: u32,
                new_position: WorldPosData<'a>,
                records: WithLen<u16, Vec<WorldPosData<'a>>>,
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
                starting_pos: WorldPosData<'a>,
                angle: f32,
                speed_mult: u16,
                life_mult: u16,
            },
            PlayerText { text: WithLen<u16, &'a str> },
            Pong {
                serial: u32,
                time: u32,
            },
            RequestTrade { name: WithLen<u16, &'a str> },
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
                slot: SlotObjectData<'a>,
                item_use_pos: WorldPosData<'a>,
                use_type: u8,
            },
            UsePortal { object_id: u32 },
        },
    }
}

pub use definitions::{client, server, PacketType};
