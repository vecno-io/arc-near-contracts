use crate::*;

use std::collections::HashMap;

const MAX_BASE_POINTS_TOTAL: u16 = 10000;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize,
)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenType {
    None = 0,
    Actor,
    Asset,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenId(String);

impl ToString for TokenId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for TokenId {
    fn from(item: String) -> Self {
        TokenId { 0: item }
    }
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Token {
    //type id for the token
    pub type_id: TokenType,
    //owner id for the token
    pub owner_id: AccountId,
    //royalties for this token
    pub payout: TokenPayout,
    //approval index tracker
    pub approval_index: u64,
    //approved account mapped to an index
    pub approved_accounts: HashMap<AccountId, u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token id
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
    //metadata for the token
    pub metadata: TokenData,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonPayout {
    pub payout: HashMap<AccountId, U128>,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenData {
    pub copies: Option<u64>,
    pub issued_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub starts_at: Option<u64>,
    pub updated_at: Option<u64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub extra: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<Base64VecU8>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(Clone, Default, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenPayout(HashMap<AccountId, u16>);

impl TokenData {
    pub fn assert_valid(&self) {
        require!(self.media.is_some() == self.media_hash.is_some());
        if let Some(media_hash) = &self.media_hash {
            require!(media_hash.0.len() == 32, "Media hash has to be 32 bytes");
        }

        require!(self.reference.is_some() == self.reference_hash.is_some());
        if let Some(reference_hash) = &self.reference_hash {
            require!(
                reference_hash.0.len() == 32,
                "Reference hash has to be 32 bytes"
            );
        }
    }
}

impl TokenPayout {
    // Note: Extend with configuration options
    // pub fn set_valid_cfg(cfg: TokenPayoutCfg)
    // pub fn load_valid_cfg() -> TokenPayoutCfg

    pub fn assert_valid(&self) {
        let mut total = 0;
        require!(
            self.0.len() < 5,
            format!(
                "Cannot add more than {} payouts per token, got {}",
                4,
                self.0.len()
            )
        );
        for (_account, amount) in &self.0 {
            total += amount;
        }
        require!(
            total <= MAX_BASE_POINTS_TOTAL,
            format!(
                "The total for payouts can not be larger than {}, got {}",
                total, MAX_BASE_POINTS_TOTAL
            )
        );
    }

    pub fn compute(&self, owner_id: AccountId, amount: u128, max_payouts: u32) -> JsonPayout {
        assert!(
            self.0.len() as u32 <= max_payouts,
            "The request cannot payout all royalties"
        );
        let mut total_payout = 0;
        let mut payout_object = JsonPayout {
            payout: HashMap::new(),
        };
        for (k, v) in self.0.iter() {
            let key = k.clone();
            if key != owner_id {
                payout_object
                    .payout
                    .insert(key, royalty_to_payout(*v, amount));
                total_payout += *v;
            }
        }
        payout_object.payout.insert(
            owner_id.clone(),
            royalty_to_payout(MAX_BASE_POINTS_TOTAL - total_payout, amount),
        );
        payout_object
    }
}
