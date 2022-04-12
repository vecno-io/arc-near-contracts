use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, require, AccountId, Balance, PanicOnDefault};
use std::collections::HashMap;

use guild_standard::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    locked_amount: Balance,

    guilds: Guilds,
    votes: Votes,
}

pub fn locked_storage_amount() -> Balance {
    env::storage_byte_cost() * (env::storage_usage() as Balance)
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
            locked_amount: 0,

            guilds: Guilds::new(id),
            votes: Votes::new(),
        };
        this.guilds.register(id, guild, board_map, member_map);

        this
    }

    /// Returns the amount of NEAR that can be spent.
    pub fn get_available_amount(&self) -> U128 {
        U128(env::account_balance() - locked_storage_amount() - self.locked_amount)
    }

    /// Returns the amount of NEAR that is locked for storage.
    pub fn get_locked_storage_amount(&self) -> U128 {
        let locked_storage_amount = env::storage_byte_cost() * (env::storage_usage() as u128);
        U128(locked_storage_amount)
    }
}
