use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, require, AccountId, Balance, CryptoHash, Gas, Promise, PromiseOrValue};

pub const ARC_STANDARD_SPEC: &str = "ARC-1.0.0";

pub mod meta;
pub mod utils;

pub mod actor;
pub mod event;
pub mod guild;
pub mod token;

pub use crate::meta::*;
pub use crate::utils::*;

pub use crate::actor::*;
pub use crate::event::*;
pub use crate::guild::*;
pub use crate::token::*;

pub type Admin = LazyOption<GuildId>;

// TODO AccountKey Needs to be fixed size,
// the annoying part: no support for [u8, 64].

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct AccountKey(String);

impl ToString for AccountKey {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for AccountKey {
    fn from(item: String) -> Self {
        AccountKey { 0: item }
    }
}

impl From<AccountId> for AccountKey {
    fn from(item: AccountId) -> Self {
        AccountKey {
            0: item.to_string(),
        }
    }
}

impl From<&AccountKey> for AccountId {
    fn from(item: &AccountKey) -> Self {
        AccountId::new_unchecked(item.to_string())
    }
}

pub trait ArcApp {
    fn arc_create_guild(
        &mut self,
        ceo_id: AccountId,
        guild_key: GuildId,
        guild_type: GuildType,
        guild_data: GuildData,
        guild_board: GuildBoard,
        guild_payout: Option<AccountId>,
    );

    fn arc_mint_actor(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        actor_data: ActorData,
        token_data: TokenData,
        token_payout: TokenPayout,
        guild_id: Option<GuildId>,
    );

    fn arc_add_guild_member(&mut self, guild_key: GuildId, member_id: AccountId);
}

#[macro_export]
macro_rules! use_imports {
    () => {
        use std::collections::HashMap;

        use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
        use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
        use near_sdk::json_types::U128;
        use near_sdk::{env, ext_contract, near_bindgen, require};
        use near_sdk::{AccountId, PanicOnDefault, PromiseOrValue, PromiseResult};
    };
}
