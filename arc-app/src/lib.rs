arc_standard::use_imports!();

use arc_standard::{Actors, Admin, ContractData, Guilds, Tokens};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct App {
    admin: Admin,

    actor: Actors,
    guild: Guilds,
    token: Tokens,

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

arc_standard::impl_arc_tokens!(App, token);
arc_standard::impl_arc_guilds!(App, token, guild);
arc_standard::impl_arc_actors!(App, token, actor);

arc_standard::impl_nft_contract_data!(App, metadata);
