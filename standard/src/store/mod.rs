use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::{env, AccountId};

pub mod token;

pub type StorageId = [u8; 32];

#[derive(BorshSerialize)]
pub enum StorageKey {
    // Actor Storage Keys
    ActorCnt {},

    ActorIndex {
        token: StorageId,
    },

    ActorInfoKey {
        index: u64,
    },

    ActorOwnerCnt {
        account: StorageId,
    },
    ActorOwnerIndex {
        account: StorageId,
        token: StorageId,
    },
    ActorOwnerToken {
        account: StorageId,
        index: u64,
    },

    // Token Storage Keys
    TokenCnt {},

    TokenIndex {
        token: StorageId,
    },

    TokenInfoKey {
        index: u64,
    },
    TokenOwnerKey {
        index: u64,
    },
    TokenPayoutsKey {
        index: u64,
    },
    TokenMetadataKey {
        index: u64,
    },

    TokenOwnerCnt {
        account: StorageId,
    },
    TokenOwnerIndex {
        account: StorageId,
        token: StorageId,
    },
    TokenOwnerToken {
        account: StorageId,
        index: u64,
    },

    TokenApproval {
        index: u64,
    },
    TokenApprovalKey {
        account: StorageId,
        index: u64,
    },
    TokenApprovalAccount {
        value: u32,
        index: u64,
    },
}

pub fn make_string_key(str: &String) -> StorageId {
    let mut key = StorageId::default();
    key.copy_from_slice(&env::sha256(str.as_bytes()));
    key
}

pub fn make_account_key(account: &AccountId) -> StorageId {
    let mut key = StorageId::default();
    key.copy_from_slice(&env::sha256(account.as_bytes()));
    key
}
