use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};
use std::collections::HashMap;

use guild_standard::{GuildId, GuildInfo, Guilds};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    guilds: Guilds,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn initialize(
        id: &GuildId,
        guild: &GuildInfo,
        board_map: &HashMap<AccountId, u16>,
        member_map: &HashMap<AccountId, u128>,
    ) -> Self {
        require!(
            !env::state_exists(),
            "Contract can only be initialized once"
        );

        let mut this = Self {
            guilds: Guilds::new(),
        };
        this.guilds.register(id, guild, board_map, member_map);

        this
    }
}

impl guild_standard::Contract for Contract {}
