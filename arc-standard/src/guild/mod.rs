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
    pub info_by_id: LookupMap<GuildKey, Guild>,
    //keeps track of the guilds data for a given guild key
    pub data_for_id: UnorderedMap<GuildKey, GuildData>,
    //keeps track of the guilds board for a given guild key
    pub board_for_id: UnorderedMap<GuildKey, GuildBoard>,
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
        ceo_id: AccountId,
        type_id: GuildType,
        guild_key: GuildKey,
        guild_data: GuildData,
        guild_board: GuildBoard,
        memo: Option<String>,
    ) {
        guild_data.assert_valid();
        guild_board.assert_valid();

        // TODO: Storage cost is now the contracts task
        // assert_min_one_yocto();
        // let storage_usage = env::storage_usage();

        let guild = Guild {
            ceo_id: ceo_id,
            type_id: type_id,
        };
        require!(
            self.info_by_id.insert(&guild_key, &guild).is_none(),
            "A guild with the provided id already exits"
        );
        self.data_for_id.insert(&guild_key, &guild_data);
        self.board_for_id.insert(&guild_key, &guild_board);

        // TODO: Implement events for guild registration
        if let Some(message) = memo {
            env::log_str(&message);
        }

        // TODO Implement tracking per member? (board, owners)
        //self.add_to_member(&guild_id, &member_id);

        // TODO: Storage cost is now the contracts task
        //refund_storage_deposit(env::storage_usage() - storage_usage);
    }
}
