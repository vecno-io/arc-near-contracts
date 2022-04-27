use crate::data::token::{TokenApproval, TokenInfo, TokenMetadata, TokenOwner, TokenPayouts};

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, require, AccountId};

use super::{StorageId, StorageKey};

const MAX_APPROVALS: u32 = 32;

const ERR_MAX_TOKENS: &str = "No token indecies available";

const ERR_COUNT_NOTFOUND: &str = "Cannot find count value";
const ERR_INDEX_NOTFOUND: &str = "Cannot find index value for key";

const ERR_TOKEN_INFO_NOTFOUND: &str = "Cannot find token info for index";
const ERR_TOKEN_OWNER_NOTFOUND: &str = "Cannot find token owner for index";
const ERR_TOKEN_PAYOUTS_NOTFOUND: &str = "Cannot find token payouts for index";
const ERR_TOKEN_METADATA_NOTFOUND: &str = "Cannot find token metadata for index";
const ERR_TOKEN_APPROVAL_NOTFOUND: &str = "Cannot find token approval for index";

const ERR_COUNT_DESERIALIZATION: &str = "Cannot deserialize count with Borsh";
const ERR_INDEX_DESERIALIZATION: &str = "Cannot deserialize index with Borsh";
const ERR_TOKEN_INFO_DESERIALIZATION: &str = "Cannot deserialize token info with Borsh";
const ERR_TOKEN_OWNER_DESERIALIZATION: &str = "Cannot deserialize token owner with Borsh";
const ERR_TOKEN_PAYOUTS_DESERIALIZATION: &str = "Cannot deserialize token payouts with Borsh";
const ERR_TOKEN_METADATA_DESERIALIZATION: &str = "Cannot deserialize token metadata with Borsh";
const ERR_TOKEN_APPROVAL_DESERIALIZATION: &str = "Cannot deserialize token approval with Borsh";

const ERR_APPROVAL_SERIALIZATION: &str = "Cannot serialize token approval with Borsh";
const ERR_TOKEN_INFO_SERIALIZATION: &str = "Cannot serialize token info with Borsh";
const ERR_TOKEN_OWNER_SERIALIZATION: &str = "Cannot serialize token owner with Borsh";
const ERR_TOKEN_PAYOUTS_SERIALIZATION: &str = "Cannot serialize token payouts with Borsh";
const ERR_TOKEN_META_SERIALIZATION: &str = "Cannot serialize token metadata with Borsh";

const ERR_OWNER_INDEX_NOTFOUND: &str = "Cannot find owner index for account with token";
const ERR_OWNER_TOKEN_NOTFOUND: &str = "Cannot find owner token for account with index";

const ERR_OWNER_COUNT_DESERIALIZATION: &str = "Cannot deserialize owner count with Borsh";
const ERR_OWNER_INDEX_DESERIALIZATION: &str = "Cannot deserialize owner index with Borsh";
const ERR_OWNER_TOKEN_DESERIALIZATION: &str = "Cannot deserialize owner token with Borsh";

/******************/
/* VIEW METHODS */
/******************/

pub(crate) fn token_count() -> u64 {
    let key = StorageKey::TokenCnt {}.try_to_vec().unwrap();
    match u64::try_from_slice(&env::storage_read(&key).expect(ERR_COUNT_NOTFOUND)) {
        Err(_) => env::panic_str(ERR_COUNT_DESERIALIZATION),
        Ok(info) => info,
    }
}

pub(crate) fn token_index(token: StorageId) -> Option<u64> {
    let key = StorageKey::TokenIndex { token: token }
        .try_to_vec()
        .unwrap();
    if let Some(data) = env::storage_read(&key) {
        return match u64::try_from_slice(&data) {
            Err(_) => None,
            Ok(info) => Some(info),
        };
    }
    None
}

pub(crate) fn token_info(index: u64) -> TokenInfo {
    let key = StorageKey::TokenInfoKey { index }.try_to_vec().unwrap();
    match TokenInfo::try_from_slice(&env::storage_read(&key).expect(ERR_TOKEN_INFO_NOTFOUND)) {
        Err(_) => env::panic_str(ERR_TOKEN_INFO_DESERIALIZATION),
        Ok(info) => info,
    }
}

pub(crate) fn token_owner(index: u64) -> TokenOwner {
    let key = StorageKey::TokenOwnerKey { index }.try_to_vec().unwrap();
    match TokenOwner::try_from_slice(&env::storage_read(&key).expect(ERR_TOKEN_OWNER_NOTFOUND)) {
        Err(_) => env::panic_str(ERR_TOKEN_OWNER_DESERIALIZATION),
        Ok(info) => info,
    }
}

pub(crate) fn token_payouts(index: u64) -> TokenPayouts {
    let key = StorageKey::TokenPayoutsKey { index }.try_to_vec().unwrap();
    match TokenPayouts::try_from_slice(&env::storage_read(&key).expect(ERR_TOKEN_PAYOUTS_NOTFOUND))
    {
        Err(_) => env::panic_str(ERR_TOKEN_PAYOUTS_DESERIALIZATION),
        Ok(info) => info,
    }
}

pub(crate) fn token_metadata(index: u64) -> TokenMetadata {
    let key = StorageKey::TokenMetadataKey { index }.try_to_vec().unwrap();
    match TokenMetadata::try_from_slice(
        &env::storage_read(&key).expect(ERR_TOKEN_METADATA_NOTFOUND),
    ) {
        Err(_) => env::panic_str(ERR_TOKEN_METADATA_DESERIALIZATION),
        Ok(info) => info,
    }
}

pub(crate) fn owner_index(account: StorageId, token: StorageId) -> u64 {
    let key = StorageKey::TokenOwnerIndex { account, token }
        .try_to_vec()
        .unwrap();
    match u64::try_from_slice(&env::storage_read(&key).expect(ERR_OWNER_INDEX_NOTFOUND)) {
        Err(_) => env::panic_str(ERR_OWNER_INDEX_DESERIALIZATION),
        Ok(info) => info,
    }
}

pub(crate) fn owner_token(account: StorageId, index: u64) -> StorageId {
    let key = StorageKey::TokenOwnerToken { account, index }
        .try_to_vec()
        .unwrap();
    match StorageId::try_from_slice(&env::storage_read(&key).expect(ERR_OWNER_TOKEN_NOTFOUND)) {
        Err(_) => env::panic_str(ERR_OWNER_TOKEN_DESERIALIZATION),
        Ok(info) => info,
    }
}

/******************/
/* INTERN METHODS */
/******************/

fn owner_cnt_increment(account: StorageId) -> u64 {
    let mut out = 0;
    let key = StorageKey::TokenOwnerCnt { account }.try_to_vec().unwrap();
    if let Some(data) = env::storage_read(&key) {
        match u64::try_from_slice(&data) {
            Err(_) => env::panic_str(ERR_OWNER_COUNT_DESERIALIZATION),
            Ok(info) => out = info,
        }
    }
    env::storage_write(&key, &(out + 1).try_to_vec().unwrap());
    out
}

fn owner_cnt_decrement(account: StorageId) -> u64 {
    let mut out = 0;
    let key = StorageKey::TokenOwnerCnt { account }.try_to_vec().unwrap();
    if let Some(data) = env::storage_read(&key) {
        match u64::try_from_slice(&data) {
            Err(_) => env::panic_str(ERR_OWNER_COUNT_DESERIALIZATION),
            Ok(info) => out = info,
        }
    }
    require!(out > 0, "Can not decrement, the account index is zero");
    env::storage_write(&key, &(out - 1).try_to_vec().unwrap());
    return out - 1;
}

fn add_token_to(account: StorageId, token: StorageId) {
    let index = owner_cnt_increment(account);

    // Create storage keys
    let index_store = StorageKey::TokenOwnerIndex { account, token }
        .try_to_vec()
        .unwrap();
    let token_store = StorageKey::TokenOwnerToken { account, index }
        .try_to_vec()
        .unwrap();

    // Link token info in to owner list
    require!(
        !env::storage_write(&index_store, &index.try_to_vec().unwrap()),
        "Not a unique key on owner index"
    );
    require!(
        !env::storage_write(&token_store, &token.try_to_vec().unwrap()),
        "Not a unique key on owner token"
    );
}

fn remove_token_from(account: StorageId, token: StorageId) {
    let index = owner_index(account, token);
    let count = owner_cnt_decrement(account);

    // Swap top in to free token index
    if count > 0 && count != index {
        let top_key = owner_token(account, count);
        let top_store = StorageKey::TokenOwnerIndex {
            account,
            token: top_key,
        }
        .try_to_vec()
        .unwrap();
        let token_store = StorageKey::TokenOwnerToken { account, index }
            .try_to_vec()
            .unwrap();
        env::storage_write(&top_store, &index.try_to_vec().unwrap());
        env::storage_write(&token_store, &top_key.try_to_vec().unwrap());
    }

    // Pop and remove the token from the stack
    env::storage_remove(
        &StorageKey::TokenOwnerIndex {
            account,
            token: token,
        }
        .try_to_vec()
        .unwrap(),
    );
    env::storage_remove(
        &StorageKey::TokenOwnerToken {
            account,
            index: count,
        }
        .try_to_vec()
        .unwrap(),
    );
}

fn set_token_owner(index: u64, owner: &TokenOwner) {
    let key = StorageKey::TokenOwnerKey { index }.try_to_vec().unwrap();
    let value = owner.try_to_vec().expect(ERR_TOKEN_OWNER_SERIALIZATION);
    require!(
        // Only `create` makes new keys
        env::storage_write(&key, &value),
        "Setting the token owner must not create a new key"
    );
}

fn clear_approval(index: u64) -> u32 {
    // Load and verify the active approval range
    let key = StorageKey::TokenApproval { index }.try_to_vec().unwrap();
    let range = match TokenApproval::try_from_slice(
        &env::storage_read(&key).expect(ERR_TOKEN_APPROVAL_NOTFOUND),
    ) {
        Err(_) => env::panic_str(ERR_TOKEN_APPROVAL_DESERIALIZATION),
        Ok(info) => info,
    };
    if range.start == range.index {
        // Keep start value
        return range.index;
    }

    // for range: remove all entries
    for value in range.start..range.index {
        let acc_key = StorageKey::TokenApprovalAccount { value, index }
            .try_to_vec()
            .unwrap();
        // FixMe: Allot of assumptions here, no leak verification?
        if let Some(acc_raw) = &env::storage_read(&acc_key) {
            env::storage_remove(&acc_key);
            if let Ok(account) = StorageId::try_from_slice(&acc_raw) {
                env::storage_remove(
                    &StorageKey::TokenApprovalKey { account, index }
                        .try_to_vec()
                        .unwrap(),
                );
            }
        }
    }

    // Next start value
    range.index + 1
}

fn check_approval(
    owner: &AccountId,
    sender: &AccountId,
    index: u64,
    approval: Option<u32>,
) -> bool {
    if owner == sender {
        return true;
    }
    if approval.is_none() {
        return false;
    }

    // Load and verify the active approval range
    let key = StorageKey::TokenApproval { index }.try_to_vec().unwrap();
    let range = match TokenApproval::try_from_slice(
        &env::storage_read(&key).expect(ERR_TOKEN_APPROVAL_NOTFOUND),
    ) {
        Err(_) => env::panic_str(ERR_TOKEN_APPROVAL_DESERIALIZATION),
        Ok(info) => info,
    };
    if approval.unwrap() < range.start || approval.unwrap() > (range.start + MAX_APPROVALS) {
        return false;
    }

    // Load and verify the stored account approval value
    let account = super::make_account_key(sender);
    let value = match u32::try_from_slice(
        &env::storage_read(
            &StorageKey::TokenApprovalKey { account, index }
                .try_to_vec()
                .unwrap(),
        )
        .expect("Approval value for account not found"),
    ) {
        Err(_) => env::panic_str(&"Cannot deserialize approval value with Borsh"),
        Ok(info) => info,
    };

    // Valid when the same
    approval.unwrap() == value
}

/******************/
/* CHANGE METHODS */
/******************/

pub(crate) fn create(
    info: &TokenInfo,
    owner: &TokenOwner,
    payouts: &TokenPayouts,
    metadata: &TokenMetadata,
) {
    let index = token_count();
    require!(index < u64::MAX, ERR_MAX_TOKENS);

    // Verify the token id
    let token_key = super::make_string_key(&info.token_id);
    let index_store = StorageKey::TokenIndex { token: token_key }
        .try_to_vec()
        .unwrap();
    require!(
        !env::storage_has_key(&index_store),
        "The token key must be unique, try a difrent key"
    );

    // Verify the index and setup token info
    require!(
        !env::storage_write(
            &StorageKey::TokenApproval { index }.try_to_vec().unwrap(),
            &TokenApproval { start: 0, index: 0 }
                .try_to_vec()
                .expect(ERR_APPROVAL_SERIALIZATION),
        ),
        "Not a unique key on approval"
    );
    require!(
        !env::storage_write(
            &StorageKey::TokenInfoKey { index }.try_to_vec().unwrap(),
            &info.try_to_vec().expect(ERR_TOKEN_INFO_SERIALIZATION),
        ),
        "Not a unique key on info"
    );
    require!(
        !env::storage_write(
            &StorageKey::TokenMetadataKey { index }.try_to_vec().unwrap(),
            &metadata.try_to_vec().expect(ERR_TOKEN_META_SERIALIZATION),
        ),
        "Not a unique key on metadata"
    );
    require!(
        !env::storage_write(
            &StorageKey::TokenOwnerKey { index }.try_to_vec().unwrap(),
            &owner.try_to_vec().expect(ERR_TOKEN_OWNER_SERIALIZATION),
        ),
        "Not a unique key on owner"
    );
    require!(
        !env::storage_write(
            &StorageKey::TokenPayoutsKey { index }.try_to_vec().unwrap(),
            &payouts.try_to_vec().expect(ERR_TOKEN_PAYOUTS_SERIALIZATION),
        ),
        "Not a unique key on payouts"
    );

    // Add the token to the current owner account
    add_token_to(super::make_account_key(&owner.account), token_key);

    env::storage_write(
        // Finaly save the count by seting the next index
        &StorageKey::TokenCnt {}.try_to_vec().unwrap(),
        &(index + 1).try_to_vec().unwrap(),
    );
}

pub(crate) fn transfer(
    token: &String,
    sender: &AccountId,
    receiver: &AccountId,
    approval: Option<u32>,
) -> (u64, AccountId) {
    let token_key = super::make_string_key(token);

    // Verify token ownership
    let index = token_index(token_key.clone()).expect("Token key not found");
    let owner = token_owner(index);
    require!(
        &owner.account != receiver,
        "Can not transfer to the same owner"
    );
    require!(owner.token_id.is_none(), "Can not transfer a linked token");

    // Verify senders transfer authorization
    require!(
        check_approval(&owner.account, sender, index, approval),
        "The sender does not have permission to transfer the token"
    );

    // Move the token and set the new token owner account
    remove_token_from(super::make_account_key(&owner.account), token_key);
    add_token_to(super::make_account_key(receiver), token_key);
    set_token_owner(
        index,
        &TokenOwner {
            account: receiver.clone(),
            guild_id: owner.guild_id,
            token_id: owner.token_id,
        },
    );

    // Clear out the approvals and reset
    let mut next = clear_approval(index);
    if next >= (u32::MAX - 1) - MAX_APPROVALS {
        next = 0;
    }
    require!(
        !env::storage_write(
            &StorageKey::TokenApproval { index }.try_to_vec().unwrap(),
            &TokenApproval {
                start: next,
                index: next
            }
            .try_to_vec()
            .expect(ERR_APPROVAL_SERIALIZATION),
        ),
        "Not a unique key on approval"
    );

    // Return old owner
    (index, owner.account)
}

pub(crate) fn revoke(sender: AccountId, account: AccountId, index: u64) -> bool {
    // Verify token and owner
    let owner = token_owner(index);
    require!(owner.account == sender, "Sender must be current owner");

    // If keys exist remove them
    let key = StorageKey::TokenApprovalKey {
        account: super::make_account_key(&account),
        index,
    }
    .try_to_vec()
    .unwrap();
    // FixMe: Some assumptions, no leak verification?
    if let Some(raw) = &env::storage_read(&key) {
        env::storage_remove(&key);
        if let Ok(value) = u32::try_from_slice(&raw) {
            env::storage_remove(
                &StorageKey::TokenApprovalAccount { value, index }
                    .try_to_vec()
                    .unwrap(),
            );
            return true;
        }
    }
    return false;
}

pub(crate) fn approve(sender: AccountId, account: AccountId, index: u64) -> u32 {
    // Verify token, owner and link
    let owner = token_owner(index);
    require!(owner.account == sender, "Sender must be current owner");
    require!(owner.token_id.is_none(), "Can not approve a linked token");

    // Get and verify approval range
    let key = StorageKey::TokenApproval { index }.try_to_vec().unwrap();
    let range = match TokenApproval::try_from_slice(
        &env::storage_read(&key).expect(ERR_TOKEN_APPROVAL_NOTFOUND),
    ) {
        Err(_) => env::panic_str(ERR_TOKEN_APPROVAL_DESERIALIZATION),
        Ok(info) => info,
    };
    require!(
        range.start + MAX_APPROVALS > range.index,
        "Unable to add approval for account, already at max"
    );

    // Only `create` makes new keys
    require!(
        env::storage_write(
            &StorageKey::TokenApproval { index }.try_to_vec().unwrap(),
            &TokenApproval {
                start: range.start,
                index: range.index + 1,
            }
            .try_to_vec()
            .expect(ERR_APPROVAL_SERIALIZATION),
        ),
        "Approving an account can not create a new approval key"
    );

    let account_key = super::make_account_key(&account);

    // Needs to be unique, can not approve twice
    require!(
        !env::storage_write(
            &StorageKey::TokenApprovalKey {
                account: account_key,
                index,
            }
            .try_to_vec()
            .unwrap(),
            &range.index.try_to_vec().unwrap(),
        ),
        "Approval index key must be unique"
    );
    require!(
        !env::storage_write(
            &StorageKey::TokenApprovalAccount {
                value: range.index,
                index,
            }
            .try_to_vec()
            .unwrap(),
            &account_key.try_to_vec().unwrap(),
        ),
        "Approval acount key must be unique"
    );

    // The approval id
    range.index
}
