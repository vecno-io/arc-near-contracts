use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, require, AccountId, Balance, CryptoHash, Gas, PanicOnDefault,
    Promise, PromiseOrValue, PromiseResult,
};

mod intern;

pub mod actor;
pub mod approval;
pub mod events;
pub mod guild;
pub mod meta;
pub mod royalty;
pub mod token;

use crate::intern::*;

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

pub type Admin = LazyOption<TokenKey>;

// TODO Replace CryptoHash > AccountKey
// Map account sting on to a [64]bytes array.
pub type AccountKey = [u8; 64];

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Actors {
    //keeps track of the token struct for a given token ID
    pub info_by_id: LookupMap<TokenKey, Token>,
    //keeps track of the tokens actordata for a given token ID
    pub data_for_id: UnorderedMap<TokenKey, ActorData>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guilds {
    //keeps track of the guild struct for a given guild ID
    pub info_by_id: LookupMap<GuildKey, Guild>,
    //keeps track of the guilds guilddata for a given guild ID
    pub data_for_id: UnorderedMap<GuildKey, GuildData>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tokens {
    //keeps track of the token struct for a given token ID
    pub info_by_id: LookupMap<TokenKey, Token>,
    //keeps track of the tokens tokendata for a given token ID
    pub data_for_id: UnorderedMap<TokenKey, TokenData>,
    //keeps track of all the tokens for a given account
    pub list_per_owner: LookupMap<CryptoHash, UnorderedSet<TokenKey>>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    admin: Admin,
    actors: Actors,
    guilds: Guilds,
    tokens: Tokens,
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
impl Contract {
    #[init]
    pub fn new(manager_id: AccountId, guild_data: GuildData) -> Self {
        require!(!env::state_exists(), "Already initialized");
        guild_data.assert_valid();

        let token_id = "todo_id".to_string();
        // ToDo: Register the managing guild on to slot 0

        let this = Self {
            admin: Admin::new(StorageKey::ManagerId.try_to_vec().unwrap(), Some(&token_id)),
            actors: Actors::new(),
            guilds: Guilds::new(),
            tokens: Tokens::new(),
        };
        this
    }

    #[init]
    pub fn new_default_meta(manager_id: AccountId) -> Self {
        Self::new(
            manager_id,
            GuildData {
                spec: NFT_METADATA_SPEC.to_string(),
                tag: "Arc-Core".to_string(),
                name: "The Core Guild".to_string(),
                icon: None,
                icon_hash: None,
                media: None,
                media_hash: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
}
