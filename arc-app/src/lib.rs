arc_standard::use_imports!();

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

arc_standard::impl_nft_core!(Contract, tokens);
// arc_standard::impl_nft_core!(Contract, tokens);
// arc_standard::impl_nft_approve!(Contract, tokens);
// arc_standard::impl_nft_royalties!(Contract, tokens);

arc_standard::impl_nft_contract_data!(Contract, metadata);

arc_standard::impl_arc_guilds!(Contract, guilds);
