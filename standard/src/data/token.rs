use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenApproval {
    pub start: u32,
    pub index: u32,
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct TokenInfo {
    pub type_id: String,
    pub group_id: String,
    pub token_id: String,
}

#[derive(Clone, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub copies: Option<u64>,
    pub issued_at: Option<String>,
    pub expires_at: Option<String>,
    pub starts_at: Option<String>,
    pub updated_at: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub extra: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenOwner {
    pub account: AccountId,
    pub guild_id: Option<String>,
    pub token_id: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenPayouts {
    pub guild: u16,
    pub accounts: HashMap<AccountId, u16>,
}

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize,
)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenType {
    None = 0,
    Actor,
    Asset,
    Guild,
}
