use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, require, AccountId, Balance, CryptoHash, Gas, Promise, PromiseOrValue};

pub mod intern;

pub mod actor;
pub mod event;
pub mod guild;
pub mod meta;
pub mod token;

pub use crate::intern::*;

pub use crate::actor::*;
pub use crate::event::*;
pub use crate::guild::*;
pub use crate::meta::*;
pub use crate::token::*;

pub type Admin = LazyOption<GuildKey>;

// TODO AccountKey Needs to be fixed size,
// the annoying part: no support for [u8, 64].

#[derive(Clone, BorshSerialize)]
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

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// pub struct Contract {
//     admin: Admin,
//     actors: Actors,
//     guilds: Guilds,
//     tokens: Tokens,
// }

// #[near_bindgen]
// impl Contract {
//     #[init]
//     pub fn new(
//         ceo_id: AccountId,
//         guild_key: GuildKey,
//         guild_data: GuildData,
//         guild_board: GuildBoard,
//     ) -> Self {
//         require!(!env::state_exists(), "Already initialized");
//         guild_data.assert_valid();
//         guild_board.assert_valid();

//         let mut this = Self {
//             admin: Admin::new(
//                 StorageKey::ManagerId.try_to_vec().unwrap(),
//                 Some(&guild_key),
//             ),
//             actors: Actors::new(),
//             guilds: Guilds::new(),
//             tokens: Tokens::new(),
//         };
//         this.arc_register_guild(ceo_id, guild_key, guild_data, guild_board, None);
//         this
//     }

//     #[init]
//     pub fn new_default_guild(ceo_id: AccountId, board: AccountId) -> Self {
//         let mut members = HashMap::new();
//         members.insert(board, 10000);
//         Self::new(
//             ceo_id,
//             GuildKey::from("admin:guild".to_string()),
//             GuildData {
//                 spec: NFT_METADATA_SPEC.to_string(),
//                 tag: "Arc-Core".to_string(),
//                 name: "The Core Guild".to_string(),
//                 icon: None,
//                 icon_hash: None,
//                 media: None,
//                 media_hash: None,
//                 reference: None,
//                 reference_hash: None,
//             },
//             GuildBoard {
//                 size: 1,
//                 share: 5000,
//                 members,
//             },
//         )
//     }
// }

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
