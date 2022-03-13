use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, require, AccountId, Balance, CryptoHash, Gas, PanicOnDefault,
    Promise, PromiseOrValue, PromiseResult,
};

pub mod intern;

pub mod actor;
pub mod approval;
pub mod events;
pub mod guild;
pub mod meta;
pub mod royalty;
pub mod token;

pub use crate::intern::*;

pub use crate::actor::*;
pub use crate::approval::*;
pub use crate::events::*;
pub use crate::guild::*;
pub use crate::meta::*;
pub use crate::royalty::*;
pub use crate::token::*;

#[derive(BorshSerialize)]
pub enum StorageKey {
    ManagerId,
    ContractData,
    GuildsById,
    TokensById,
    GuilddataById,
    TokendataById,
    ActordataById,
    TokensPerGuild,
    TokensPerOwner,
    TokensPerOwnerSet { owner_key: CryptoHash },
}

pub type Admin = LazyOption<GuildKey>;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Actors {
    //keeps track of the token info for a given token key
    pub info_by_id: LookupMap<TokenKey, Token>,
    //keeps track of the tokens data for a given token key
    pub data_for_id: UnorderedMap<TokenKey, ActorData>,
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

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tokens {
    //keeps track of the token info for a given token key
    pub info_by_id: LookupMap<TokenKey, Token>,
    //keeps track of the tokens data for a given token key
    pub data_for_id: UnorderedMap<TokenKey, TokenData>,
    //keeps track of all the tokens for a given account key
    pub list_per_owner: LookupMap<CryptoHash, UnorderedSet<TokenKey>>,
}

impl Actors {
    pub fn new() -> Self {
        let this = Self {
            info_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
        };
        this
    }
}

impl Guilds {
    pub fn new() -> Self {
        let this = Self {
            info_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
            board_for_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
        };
        this
    }
}

impl Tokens {
    pub fn new() -> Self {
        let this = Self {
            info_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
            list_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
        };
        this
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    admin: Admin,
    actors: Actors,
    guilds: Guilds,
    tokens: Tokens,
}

// #[near_bindgen]
// impl Contract {
//     #[init]
//     pub fn new(
//         ceo_id: AccountId,
//         guild_key: GuildKey,
//         guild_data: GuildData,
//         guild_board: GuildBoard,
//     ) -> Self {
//         require!(!env::state_exists(), "Already initialized");
//         guild_data.assert_valid();
//         guild_board.assert_valid();

//         let mut this = Self {
//             admin: Admin::new(
//                 StorageKey::ManagerId.try_to_vec().unwrap(),
//                 Some(&guild_key),
//             ),
//             actors: Actors::new(),
//             guilds: Guilds::new(),
//             tokens: Tokens::new(),
//         };
//         this.arc_register_guild(ceo_id, guild_key, guild_data, guild_board, None);
//         this
//     }

//     #[init]
//     pub fn new_default_guild(ceo_id: AccountId, board: AccountId) -> Self {
//         let mut members = HashMap::new();
//         members.insert(board, 10000);
//         Self::new(
//             ceo_id,
//             GuildKey::from("admin:guild".to_string()),
//             GuildData {
//                 spec: NFT_METADATA_SPEC.to_string(),
//                 tag: "Arc-Core".to_string(),
//                 name: "The Core Guild".to_string(),
//                 icon: None,
//                 icon_hash: None,
//                 media: None,
//                 media_hash: None,
//                 reference: None,
//                 reference_hash: None,
//             },
//             GuildBoard {
//                 size: 1,
//                 share: 5000,
//                 members,
//             },
//         )
//     }
// }
