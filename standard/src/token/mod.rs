use crate::*;

use crate::event::*;
use crate::share::*;

pub mod api;
pub mod data;

pub use self::api::*;
pub use self::data::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Tokens {
    //keeps track of the token info for a given token key
    pub info_by_id: LookupMap<TokenId, Token>,
    //keeps track of the tokens data for a given token key
    pub data_for_id: UnorderedMap<TokenId, TokenData>,
    //keeps track of all the tokens for a given account key
    pub list_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
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
        owner: &OwnerIds,
        token_id: &TokenId,
        type_id: TokenType,
        token_data: TokenData,
        token_payout: TokenPayout,
        memo: Option<String>,
    ) {
        token_data.require_valid();
        token_payout.require_valid();

        let token = Token {
            type_id: type_id,
            owner: owner.clone(),
            payout: token_payout,
            approval_index: 0,
            approved_accounts: Default::default(),
        };
        require!(
            self.info_by_id.insert(&token_id, &token).is_none(),
            "a token with the provided id already exits"
        );

        self.add_to_owner(owner.account.clone().into(), &token_id);
        self.data_for_id.insert(&token_id, &token_data);

        let nft_mint_log: JsonEventLog = JsonEventLog {
            standard: EVENT_NFT_STANDARD_NAME.to_string(),
            version: EVENT_NFT_METADATA_SPEC.to_string(),
            event: JsonEventVariant::NftMint(vec![NftMintLog {
                owner_id: token.owner.account.to_string(),
                token_ids: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&nft_mint_log.to_string());
    }

    pub fn transfer(
        &mut self,
        token_id: &TokenId,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Token {
        let token = self
            .info_by_id
            .get(&token_id)
            .expect("Token info not found");

        require!(
            token.owner.token_id.is_none(),
            "can not transfer tokens owned by other tokens"
        );

        if sender_id != &token.owner.account {
            if !token.approved_accounts.contains_key(&sender_id) {
                env::panic_str("Unauthorized transfer");
            }
            if let Some(enforced_approval_id) = approval_id {
                let actual_approval_id = token
                    .approved_accounts
                    .get(&sender_id)
                    .expect("Sender is not authorized to transfer");
                require!(
                    actual_approval_id == &enforced_approval_id,
                    "Sender provided an invalid approval id",
                );
            }
        }
        require!(
            &token.owner.account != receiver_id,
            "The owner and the receiver should be different."
        );

        let new_token = Token {
            type_id: token.type_id,
            owner: OwnerIds {
                account: receiver_id.clone(),
                guild_id: token.owner.guild_id.clone(),
                token_id: token.owner.token_id.clone(),
            },
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
        let nft_transfer_log: JsonEventLog = JsonEventLog {
            standard: EVENT_NFT_STANDARD_NAME.to_string(),
            version: EVENT_NFT_METADATA_SPEC.to_string(),
            event: JsonEventVariant::NftTransfer(vec![NftTransferLog {
                authorized_id,
                old_owner_id: token.owner.account.to_string(),
                new_owner_id: receiver_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&nft_transfer_log.to_string());

        token
    }
}

crate::impl_is_owned!(Tokens, TokenId, TokenListPerOwnerSet);

#[cfg(test)]
mod tests {
    mod data;
}
