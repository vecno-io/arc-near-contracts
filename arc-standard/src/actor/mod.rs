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
    pub data_for_id: UnorderedMap<TokenId, ActorData>,
    //keeps track of all the tokens for a given account key
    pub list_per_owner: LookupMap<AccountKey, UnorderedSet<TokenId>>,
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
        receiver_id: &AccountId,
        token_id: &TokenId,
        actor_data: ActorData,
        memo: Option<String>,
    ) {
        actor_data.assert_valid();

        require!(
            self.data_for_id.insert(token_id, &actor_data).is_none(),
            "An actor with the provided id already exits"
        );

        self.add_to_owner(receiver_id.clone().into(), token_id);

        let arc_register_log: ArcEventLog = ArcEventLog {
            module: EVENT_ARC_STANDARD_ACTOR.to_string(),
            version: EVENT_ARC_METADATA_SPEC.to_string(),
            event: ArcEventVariant::ArcRegister(vec![ArcRegisterLog {
                user_id: receiver_id.to_string(),
                keys_list: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&arc_register_log.to_string());
    }

    pub fn transfer(&mut self, token_id: &TokenId, sender_id: &AccountId, receiver_id: &AccountId) {
        self.remove_from_owner(sender_id.clone().into(), &token_id);
        self.add_to_owner(receiver_id.clone().into(), &token_id);
    }
}

crate::impl_item_is_owned!(Actors, TokenId, ActorListPerOwnerSet);
