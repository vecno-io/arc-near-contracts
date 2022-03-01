use arc_shared::{ActorData, ContractData, Payout, TokenData, TokenId, NFT_METADATA_SPEC};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::intern::*;

pub use crate::actor::*;
pub use crate::approval::*;
pub use crate::events::*;
pub use crate::royalty::*;
pub use crate::token::*;

mod intern;

mod actor;
mod approval;
mod events;
mod royalty;
mod token;

#[derive(BorshSerialize)]
pub enum StorageKey {
    GuildCount,
    ContractData,
    TokensById,
    TokensByOwner,
    TokensByOwnerSet { owner_key: CryptoHash },
    TokendataById,
    ActordataById,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,
    //keeps track of the contract data for a contract
    pub guild_count: LazyOption<u64>,
    //keeps track of the contract data for a contract
    pub contract_data: LazyOption<ContractData>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token tokendata for a given token ID
    pub tokendata_by_id: UnorderedMap<TokenId, TokenData>,
    //keeps track of the token actordata for a given token ID
    pub actordata_by_id: UnorderedMap<TokenId, ActorData>,

    //keeps track of all the token for a given account
    pub tokens_per_owner: LookupMap<CryptoHash, UnorderedSet<TokenId>>,
}

#[near_bindgen]
impl Contract {
    pub fn nft_metadata(&self) -> ContractData {
        self.contract_data.get().unwrap()
    }

    #[init]
    pub fn new(owner_id: AccountId, contract_data: ContractData) -> Self {
        contract_data.assert_valid();

        //initialize the contract data and return it
        let this = Self {
            owner_id,
            guild_count: LazyOption::new(StorageKey::GuildCount.try_to_vec().unwrap(), Some(&0)),
            contract_data: LazyOption::new(
                StorageKey::ContractData.try_to_vec().unwrap(),
                Some(&contract_data),
            ),

            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            tokens_per_owner: LookupMap::new(StorageKey::TokensByOwner.try_to_vec().unwrap()),

            tokendata_by_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
            actordata_by_id: UnorderedMap::new(StorageKey::ActordataById.try_to_vec().unwrap()),
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
