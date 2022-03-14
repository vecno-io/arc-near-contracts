use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env, require, AccountId};

pub mod api;
pub mod data;

pub use self::api::*;
pub use self::data::*;

#[derive(BorshSerialize)]
pub enum StorageKey {
    ActorDataForId,
    ActorListPerOwner,
    ActorListPerOwnerSet { owner_key: AccountKey },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Actors {
    //keeps track of the tokens data for a given token key
    pub data_for_id: UnorderedMap<TokenKey, ActorData>,
    //keeps track of all the tokens for a given account key
    pub list_per_owner: LookupMap<AccountKey, UnorderedSet<TokenKey>>,
}

impl Actors {
    pub fn new() -> Self {
        let this = Self {
            data_for_id: UnorderedMap::new(StorageKey::ActorDataForId.try_to_vec().unwrap()),
            list_per_owner: LookupMap::new(StorageKey::ActorListPerOwner.try_to_vec().unwrap()),
        };
        this
    }

    pub fn register(
        &mut self,
        token_id: TokenKey,
        actor_data: ActorData,
        receiver_id: AccountId,
        memo: Option<String>,
    ) {
        actor_data.assert_valid();

        // TODO: Storage cost is now the contracts task
        // assert_min_one_yocto();
        // let storage_usage = env::storage_usage();

        require!(
            self.data_for_id.insert(&token_id, &actor_data).is_none(),
            "A token with the provided id already exits"
        );

        self.add_to_owner(receiver_id.clone().into(), &token_id);

        let nft_transfer_log: EventLog = EventLog {
            standard: EVENT_ARC_STANDARD_NAME.to_string(),
            version: EVENT_ARC_METADATA_SPEC.to_string(),
            event: EventLogVariant::ArcMint(vec![ArcMintLog {
                owner_id: receiver_id.to_string(),
                token_type: TokenType::Actor,
                token_list: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&nft_transfer_log.to_string());

        // TODO: Storage cost is now the contracts task
        //refund_storage_deposit(env::storage_usage() - storage_usage);
    }

    pub fn transfer(
        &mut self,
        token_id: &TokenKey,
        sender_id: &AccountId,
        receiver_id: &AccountId,
    ) {
        self.remove_from_owner(sender_id.clone().into(), &token_id);
        self.add_to_owner(receiver_id.clone().into(), &token_id);
    }
}

crate::impl_item_is_owned!(Actors, TokenKey, ActorListPerOwnerSet);
