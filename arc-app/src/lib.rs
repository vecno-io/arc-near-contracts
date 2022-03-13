arc_standard::use_imports!();

use arc_standard::{Actors, Admin, Guilds, Metadata, Tokens};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct App {
    admin: Admin,

    actor: Actors,
    guild: Guilds,
    token: Tokens,

    metadata: LazyOption<Metadata>,
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

arc_standard::impl_meta!(App, metadata);

arc_standard::impl_nft_tokens!(App, token);
arc_standard::impl_arc_guilds!(App, token, guild);
arc_standard::impl_arc_actors!(App, token, actor);

// TODO: move to contract implementaion
//create the actor and store it
// let actor_id = format!(
//     "{}:Actor:{:06}",
//     asset_data.symbol,
//     self.actors.data_for_id.len()
// );

// TODO: move to contract implementaion
//create the guild and store it
// let guild_id = format!(
//     "{}:Guild:{:06}",
//     asset_data.symbol,
//     self.guilddata_by_id.len()
// );
