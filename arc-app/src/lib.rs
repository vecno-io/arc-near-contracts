use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{near_bindgen, PanicOnDefault};

use arc_standard::{Actors, Admin, ContractData, Guilds, Tokens};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    admin: Admin,

    actors: Actors,
    guilds: Guilds,
    tokens: Tokens,

    metadata: LazyOption<ContractData>,
}

// // TODO Implement API Hooks
// // TODO Implement Private Calls
// pub trait GuildVotes {
//     fn create_vote(&self) -> bool;
// }
// impl GuildVotes for Guilds {
//     fn create_vote(&self) -> bool {
//         false
//     }
// }

arc_standard::impl_arc_guilds!(Contract, guilds);

arc_standard::impl_nft_contract_data!(Contract, metadata);
