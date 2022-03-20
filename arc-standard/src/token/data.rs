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
    //guild id for the token
    pub guild_id: Option<GuildId>,
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
    pub media_hash: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenPayout {
    pub guild: u16,
    pub accounts: HashMap<AccountId, u16>,
}

impl TokenData {
    pub fn assert_valid(&self) {
        require!(
            self.title.is_some() && self.title.as_ref().unwrap().len() <= 28,
            "Max title length = 28"
        );
        require!(
            self.description.is_some() && self.description.as_ref().unwrap().len() <= 128,
            "Max description length = 128"
        );
        require!(self.media.is_some() == self.media_hash.is_some());
        if let Some(media_hash) = &self.media_hash {
            require!(
                media_hash.len() == 64,
                "Media hash has to be hex encoded string (64 bytes)"
            );
        }

        require!(self.reference.is_some() == self.reference_hash.is_some());
        if let Some(reference_hash) = &self.reference_hash {
            require!(
                reference_hash.len() == 64,
                "Reference hash has to be hex encoded string (64 bytes)"
            );
        }
    }
}

impl TokenPayout {
    // Note: Extend with configuration options
    // pub fn set_valid_cfg(cfg: TokenPayoutCfg)
    // pub fn load_valid_cfg() -> TokenPayoutCfg

    pub fn new() -> Self {
        return Self {
            guild: 0,
            accounts: HashMap::new(),
        };
    }

    pub fn assert_valid(&self) {
        let mut total = self.guild;
        require!(
            self.accounts.len() < 5,
            "Cannot add more than 4 payouts per token"
        );
        for (_account, amount) in &self.accounts {
            total += amount;
        }
        require!(
            total <= MAX_BASE_POINTS_TOTAL,
            format!(
                "The total for payouts can not be more than {}, got {}",
                MAX_BASE_POINTS_TOTAL, total
            )
        );
    }

    pub fn compute(
        &self,
        amount: u128,
        max_len: u32,
        owner_id: AccountId,
        guild_id: Option<AccountId>,
    ) -> JsonPayout {
        let mut max_payouts = max_len;
        let mut total_payout = 0;
        let mut payout_object = JsonPayout {
            payout: HashMap::new(),
        };

        if let Some(account) = guild_id.clone() {
            max_payouts -= 1;
            if account != owner_id {
                total_payout += self.guild;
                payout_object
                    .payout
                    .insert(account, royalty_to_payout(self.guild, amount));
            }
        }
        assert!(
            self.accounts.len() as u32 <= max_payouts,
            "The request cannot payout all royalties"
        );

        for (key, val) in self.accounts.iter() {
            if key != &owner_id {
                total_payout += *val;
                let mut royalty = *val;
                if Some(key.clone()) == guild_id {
                    royalty += self.guild;
                }
                payout_object
                    .payout
                    .insert(key.clone(), royalty_to_payout(royalty, amount));
            }
        }
        assert!(
            total_payout <= MAX_BASE_POINTS_TOTAL,
            "The total payout percentage is to large"
        );

        payout_object.payout.insert(
            owner_id.clone(),
            royalty_to_payout(MAX_BASE_POINTS_TOTAL - total_payout, amount),
        );
        payout_object
    }
}
