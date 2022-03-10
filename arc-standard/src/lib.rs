use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, CryptoHash, PanicOnDefault, Promise,
    PromiseOrValue,
};

mod intern;

pub mod actor;
pub mod events;
pub mod guild;
pub mod meta;
pub mod royalty;
pub mod token;

use crate::intern::*;

pub use crate::actor::*;
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract manager
    pub manager_id: LazyOption<AccountId>,
    //keeps track of the contract data for a contract
    pub contract_data: LazyOption<ContractData>,

    //keeps track of the guild struct for a given guild ID
    pub guilds_by_id: LookupMap<GuildId, Guild>,
    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the guilds guilddata for a given guild ID
    pub guilddata_by_id: UnorderedMap<GuildId, GuildData>,
    //keeps track of the tokens tokendata for a given token ID
    pub tokendata_by_id: UnorderedMap<TokenId, TokenData>,
    //keeps track of the tokens actordata for a given token ID
    pub actordata_by_id: UnorderedMap<TokenId, ActorData>,

    //keeps track of all the guild for a given guild
    pub tokens_per_guild: LookupMap<GuildId, UnorderedSet<TokenId>>,
    //keeps track of all the tokens for a given account
    pub tokens_per_owner: LookupMap<CryptoHash, UnorderedSet<TokenId>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(manager_id: AccountId, contract_data: ContractData) -> Self {
        contract_data.assert_valid();

        //initialize the contract data and return it
        let this = Self {
            manager_id: LazyOption::new(
                StorageKey::ManagerId.try_to_vec().unwrap(),
                Some(&manager_id),
            ),
            contract_data: LazyOption::new(
                StorageKey::ContractData.try_to_vec().unwrap(),
                Some(&contract_data),
            ),

            guilds_by_id: LookupMap::new(StorageKey::GuildsById.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),

            guilddata_by_id: UnorderedMap::new(StorageKey::GuilddataById.try_to_vec().unwrap()),
            tokendata_by_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
            actordata_by_id: UnorderedMap::new(StorageKey::ActordataById.try_to_vec().unwrap()),

            tokens_per_guild: LookupMap::new(StorageKey::TokensPerGuild.try_to_vec().unwrap()),
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
        };
        this
    }
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls new: with default contract data and the owner_id
        Self::new(
            owner_id,
            ContractData {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Story Arc Assets Contract".to_string(),
                symbol: "ARC-A".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
}
