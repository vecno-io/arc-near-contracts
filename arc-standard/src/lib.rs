use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, require, AccountId, Balance, CryptoHash, Gas, Promise, PromiseOrValue};

pub mod intern;

pub mod actor;
pub mod events;
pub mod guild;
pub mod meta;
pub mod token;

pub use crate::intern::*;

pub use crate::actor::*;
pub use crate::events::*;
pub use crate::guild::*;
pub use crate::meta::*;
pub use crate::token::*;

#[derive(BorshSerialize)]
pub enum StorageKey {
    ManagerId,
    ContractData,
    GuildsById,
    TokensById,
    GuilddataById,
    TokendataById,
    ActordataById,
    TokensPerGuild,
    TokensPerOwner,
    TokensPerOwnerSet { owner_key: CryptoHash },
}

pub type Admin = LazyOption<GuildKey>;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Actors {
    //keeps track of the token info for a given token key
    //pub info_by_id: LookupMap<TokenKey, Token>,
    //keeps track of the tokens data for a given token key
    pub data_for_id: UnorderedMap<TokenKey, ActorData>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guilds {
    //keeps track of the guild info for a given guild key
    pub info_by_id: LookupMap<GuildKey, Guild>,
    //keeps track of the guilds data for a given guild key
    pub data_for_id: UnorderedMap<GuildKey, GuildData>,
    //keeps track of the guilds board for a given guild key
    pub board_for_id: UnorderedMap<GuildKey, GuildBoard>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tokens {
    //keeps track of the token info for a given token key
    pub info_by_id: LookupMap<TokenKey, Token>,
    //keeps track of the tokens data for a given token key
    pub data_for_id: UnorderedMap<TokenKey, TokenData>,
    //keeps track of all the tokens for a given account key
    pub list_per_owner: LookupMap<CryptoHash, UnorderedSet<TokenKey>>,
}

impl Actors {
    pub fn new() -> Self {
        let this = Self {
            //info_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
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
        assert_min_one_yocto();
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

impl Guilds {
    pub fn new() -> Self {
        let this = Self {
            info_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
            board_for_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
        };
        this
    }

    pub fn register(
        &mut self,
        ceo_id: AccountId,
        guild_key: GuildKey,
        guild_data: GuildData,
        guild_board: GuildBoard,
        memo: Option<String>,
    ) {
        guild_data.assert_valid();
        guild_board.assert_valid();

        // TODO: Storage cost is now the contracts task
        // assert_min_one_yocto();
        // let storage_usage = env::storage_usage();

        let guild = Guild {
            ceo_id: ceo_id,
            type_id: GuildType::Base,
        };
        require!(
            self.info_by_id.insert(&guild_key, &guild).is_none(),
            "A guild with the provided id already exits"
        );
        self.data_for_id.insert(&guild_key, &guild_data);
        self.board_for_id.insert(&guild_key, &guild_board);

        // TODO: Implement events for guild registration
        if let Some(message) = memo {
            env::log_str(&message);
        }

        // TODO Implement tracking per member? (board, owners)
        //self.add_to_member(&guild_id, &member_id);

        // TODO: Storage cost is now the contracts task
        //refund_storage_deposit(env::storage_usage() - storage_usage);
    }
}

impl Tokens {
    pub fn new() -> Self {
        let this = Self {
            info_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::TokendataById.try_to_vec().unwrap()),
            list_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
        };
        this
    }
    pub fn register(
        &mut self,
        type_id: TokenType,
        owner_id: AccountId,
        token_id: TokenKey,
        token_data: TokenData,
        token_payout: TokenPayout,
        memo: Option<String>,
    ) {
        token_data.assert_valid();
        token_payout.assert_valid();
        // TODO: Storage cost is now the contracts task
        // assert_min_one_yocto();
        // let storage_usage = env::storage_usage();

        let token = Token {
            type_id: type_id,
            owner_id: owner_id.clone(),
            payout: token_payout,
            approval_index: 0,
            approved_accounts: Default::default(),
        };
        require!(
            self.info_by_id.insert(&token_id, &token).is_none(),
            "A token with the provided id already exits"
        );
        self.add_to_owner(&token_id, &owner_id);
        self.data_for_id.insert(&token_id, &token_data);

        // TODO: Implement events for token registration
        if let Some(message) = memo {
            env::log_str(&message);
        }

        // TODO: Storage cost is now the contracts task
        //refund_storage_deposit(env::storage_usage() - storage_usage);
    }

    pub fn transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenKey,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Token {
        let token = self.info_by_id.get(token_id).expect("Token info not found");

        //no sending it to themselves
        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        //check the approval list when not owner
        if sender_id != &token.owner_id {
            if !token.approved_accounts.contains_key(sender_id) {
                env::panic_str("Unauthorized transfer");
            }

            if let Some(enforced_approval_id) = approval_id {
                let actual_approval_id = token
                    .approved_accounts
                    .get(sender_id)
                    .expect("Sender is not authorized to transfer");

                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "Sender provided an invalid approval id",
                );
            }
        }

        //remove the token fro mthe old owner and add it to the new owner
        self.remove_from_owner(token_id, &token.owner_id);
        self.add_to_owner(token_id, receiver_id);

        //create the token and store it
        let new_token = Token {
            type_id: token.type_id,
            owner_id: receiver_id.clone(),
            payout: token.payout.clone(),
            approval_index: token.approval_index,
            approved_accounts: Default::default(),
        };
        self.info_by_id.insert(token_id, &new_token);

        //log the memo message if one is provided
        if let Some(memo) = memo.as_ref() {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }

        //log an event message for the transfer
        let mut authorized_id = None;
        if approval_id.is_some() {
            authorized_id = Some(sender_id.to_string());
        }
        let nft_transfer_log: EventLog = EventLog {
            standard: EVENT_NFT_STANDARD_NAME.to_string(),
            version: EVENT_NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                authorized_id,
                old_owner_id: token.owner_id.to_string(),
                new_owner_id: receiver_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&nft_transfer_log.to_string());

        token
    }
    // TODO Rename token for to actor for
    // TODO Implement actors/guilds per owner update
    pub fn add_to_owner(&mut self, token_id: &TokenKey, owner_id: &AccountId) {
        let owner_key = hash_storage_key(owner_id.as_bytes());
        let mut tokens_set = self.list_per_owner.get(&owner_key).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::TokensPerOwnerSet { owner_key }
                    .try_to_vec()
                    .unwrap(),
            )
        });
        tokens_set.insert(token_id);
        self.list_per_owner.insert(&owner_key, &tokens_set);
    }

    // TODO Rename token for to actor for
    // TODO Implement actors/guilds per owner update
    pub fn remove_from_owner(&mut self, token_id: &TokenKey, account_id: &AccountId) {
        let owner_key = hash_storage_key(account_id.as_bytes());
        let mut tokens_set = self
            .list_per_owner
            .get(&owner_key)
            .expect("Sender must own the token");
        if tokens_set.len() == 1 {
            self.list_per_owner.remove(&owner_key);
        } else {
            tokens_set.remove(token_id);
            self.list_per_owner.insert(&owner_key, &tokens_set);
        }
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
    //where $data is LazyOption<ContractData>
    () => {
        use std::collections::HashMap;

        use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
        use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
        use near_sdk::json_types::U128;
        use near_sdk::{env, ext_contract, near_bindgen, require};
        use near_sdk::{AccountId, PanicOnDefault, PromiseOrValue, PromiseResult};
    };
}
