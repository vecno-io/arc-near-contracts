use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, require, AccountId};

pub mod api;
pub mod data;

pub use self::api::*;
pub use self::data::*;

#[derive(BorshSerialize)]
pub enum StorageKey {
    ActorDataForId,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Actors {
    //keeps track of the tokens data for a given token key
    pub data_for_id: UnorderedMap<TokenKey, ActorData>,
}

impl Actors {
    pub fn new() -> Self {
        let this = Self {
            data_for_id: UnorderedMap::new(StorageKey::ActorDataForId.try_to_vec().unwrap()),
        };
        this
    }

    pub fn register(
        &mut self,
        owner_id: AccountId,
        token_id: TokenKey,
        actor_data: ActorData,
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

        // TODO Implement this tracking per user?
        //self.add_to_owner(&token_id, &owner_id);

        // TODO: Implement events for actor registration
        if let Some(message) = memo {
            env::log_str(&message);
        }

        // TODO: Storage cost is now the contracts task
        //refund_storage_deposit(env::storage_usage() - storage_usage);
    }
}
