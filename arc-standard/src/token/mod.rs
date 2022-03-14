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
    TokenInfoById,
    TokenDataForId,
    TokenListPerOwner,
    TokenListPerOwnerSet { owner_key: AccountKey },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tokens {
    //keeps track of the token info for a given token key
    pub info_by_id: LookupMap<TokenKey, Token>,
    //keeps track of the tokens data for a given token key
    pub data_for_id: UnorderedMap<TokenKey, TokenData>,
    //keeps track of all the tokens for a given account key
    pub list_per_owner: LookupMap<AccountKey, UnorderedSet<TokenKey>>,
}

impl Tokens {
    pub fn new() -> Self {
        let this = Self {
            info_by_id: LookupMap::new(StorageKey::TokenInfoById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::TokenDataForId.try_to_vec().unwrap()),
            list_per_owner: LookupMap::new(StorageKey::TokenListPerOwner.try_to_vec().unwrap()),
        };
        this
    }

    pub fn register(
        &mut self,
        type_id: TokenType,
        token_id: TokenKey,
        token_data: TokenData,
        token_payout: TokenPayout,
        receiver_id: AccountId,
        memo: Option<String>,
    ) {
        token_data.assert_valid();
        token_payout.assert_valid();
        // TODO: Storage cost is now the contracts task
        // assert_min_one_yocto();
        // let storage_usage = env::storage_usage();

        let token = Token {
            type_id: type_id,
            owner_id: receiver_id.clone(),
            payout: token_payout,
            approval_index: 0,
            approved_accounts: Default::default(),
        };
        require!(
            self.info_by_id.insert(&token_id, &token).is_none(),
            "A token with the provided id already exits"
        );

        self.add_to_owner(receiver_id.into(), &token_id);
        self.data_for_id.insert(&token_id, &token_data);

        let nft_transfer_log: EventLog = EventLog {
            standard: EVENT_NFT_STANDARD_NAME.to_string(),
            version: EVENT_NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftMint(vec![NftMintLog {
                owner_id: token.owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&nft_transfer_log.to_string());

        // TODO: Storage cost is now the contracts task
        //refund_storage_deposit(env::storage_usage() - storage_usage);
    }

    pub fn transfer(
        &mut self,
        token_id: &TokenKey,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Token {
        let token = self
            .info_by_id
            .get(&token_id)
            .expect("Token info not found");

        if sender_id != &token.owner_id {
            if !token.approved_accounts.contains_key(&sender_id) {
                env::panic_str("Unauthorized transfer");
            }
            if let Some(enforced_approval_id) = approval_id {
                let actual_approval_id = token
                    .approved_accounts
                    .get(&sender_id)
                    .expect("Sender is not authorized to transfer");
                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "Sender provided an invalid approval id",
                );
            }
        }
        assert_ne!(
            &token.owner_id, receiver_id,
            "The owner and the receiver should be different."
        );

        let new_token = Token {
            type_id: token.type_id,
            owner_id: receiver_id.clone(),
            payout: token.payout.clone(),
            approval_index: token.approval_index,
            approved_accounts: Default::default(),
        };
        self.info_by_id.insert(&token_id, &new_token);

        self.remove_from_owner(sender_id.clone().into(), &token_id);
        self.add_to_owner(receiver_id.clone().into(), &token_id);

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
}

crate::impl_item_is_owned!(Tokens, TokenKey, TokenListPerOwnerSet);
