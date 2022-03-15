use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{env, require, AccountId};

pub mod api;
pub mod data;

pub use self::api::*;
pub use self::data::*;

#[derive(BorshSerialize)]
pub enum StorageKey {
    GuildInfoById,
    GuildDataForId,
    GuildBoardForId,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guilds {
    //keeps track of the guild info for a given guild key
    pub info_by_id: LookupMap<GuildId, Guild>,
    //keeps track of the guilds data for a given guild key
    pub data_for_id: UnorderedMap<GuildId, GuildData>,
    //keeps track of the guilds board for a given guild key
    pub board_for_id: UnorderedMap<GuildId, GuildBoard>,
}

impl Guilds {
    pub fn new() -> Self {
        let this = Self {
            info_by_id: LookupMap::new(StorageKey::GuildInfoById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::GuildDataForId.try_to_vec().unwrap()),
            board_for_id: UnorderedMap::new(StorageKey::GuildBoardForId.try_to_vec().unwrap()),
        };
        this
    }

    pub fn register(
        &mut self,
        ceo_id: &AccountId,
        guild_id: &GuildId,
        guild_type: GuildType,
        guild_data: GuildData,
        guild_board: GuildBoard,
        memo: Option<String>,
    ) {
        guild_data.assert_valid();
        guild_board.assert_valid();

        let guild = Guild {
            ceo_id: ceo_id.clone(),
            type_id: guild_type.clone(),
        };
        require!(
            self.info_by_id.insert(&guild_id, &guild).is_none(),
            "A guild with the provided id already exits"
        );
        self.data_for_id.insert(&guild_id, &guild_data);
        self.board_for_id.insert(&guild_id, &guild_board);

        // TODO Implement tracking per member? (board, owners)
        //self.add_to_member(&guild_id, &member_id);

        let arc_register_log: ArcEventLog = ArcEventLog {
            module: EVENT_ARC_STANDARD_GUILD.to_string(),
            version: EVENT_ARC_METADATA_SPEC.to_string(),
            event: ArcEventVariant::ArcRegister(vec![ArcRegisterLog {
                user_id: ceo_id.to_string(),
                keys_list: vec![guild_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&arc_register_log.to_string());
    }
}
