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

// #[near_bindgen]
// impl ArcApp for ArcActors {
//     #[payable]
//     fn arc_create_guild(
//         &mut self,
//         ceo_id: AccountId,
//         guild_id: GuildId,
//         guild_type: GuildType,
//         guild_data: GuildData,
//         guild_board: GuildBoard,
//         guild_payout: Option<AccountId>,
//     ) {
//         require_min_one_yocto();
//         let storage_usage = env::storage_usage();

//         // TODO Implement checks who can call this?
//         // Make the admin guild vote on creation?
//         // For demo/Testing it is open to all.

//         self.guild.register(
//             &ceo_id,
//             &guild_id,
//             guild_type,
//             guild_data,
//             guild_board,
//             guild_payout,
//             None,
//         );

//         refund_storage_deposit(env::storage_usage() - storage_usage);
//     }

//     #[payable]
//     fn arc_add_guild_member(&mut self, guild_key: GuildId, member_id: AccountId) {
//         require_min_one_yocto();
//         let storage_usage = env::storage_usage();

//         // TODO: FixMe: Temp rush job to meet encode club dealines
//         // Note: Base setup, no verification or reuse of old tokens implemented
//         self.guild.create_member_token(&guild_key, Some(member_id));

//         refund_storage_deposit(env::storage_usage() - storage_usage);
//     }
// }
