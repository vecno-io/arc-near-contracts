use crate::*;

use std::collections::HashMap;

#[derive(Copy, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenType {
    None,
    Actor,
    Member,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenKey(String);

impl ToString for TokenKey {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for TokenKey {
    fn from(item: String) -> Self {
        TokenKey { 0: item }
    }
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Token {
    //type id for the token
    pub type_id: TokenType,
    //owner id for the token
    pub owner_id: AccountId,
    //royalties for this token
    pub royalty: HashMap<AccountId, u32>,
    //approval index tracker
    pub approval_index: u64,
    //approved account mapped to an index
    pub approved_accounts: HashMap<AccountId, u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token id
    pub token_id: TokenKey,
    //owner of the token
    pub owner_id: AccountId,
    //metadata for the token
    pub metadata: TokenData,
}

pub trait NftToken {
    //view call for returning the token data for the provided id
    fn nft_token(&self, token_id: TokenKey) -> Option<JsonToken>;

    //transfers a token to a receiver ID
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenKey,
        approval_id: u64,
        memo: Option<String>,
    );

    //transfers a token to a receiver and calls the receivers contract
    /// Returns `true` if the token was transferred from the sender's account
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenKey,
        approval_id: u64,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;
}

pub trait NftTokenEnumerator {
    fn nft_total_supply(&self) -> U128;

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken>;

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128;

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken>;
}

#[ext_contract(ext_nft_receiver)]
trait NftReceiver {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenKey,
        msg: String,
    ) -> Promise;
}

#[ext_contract(ext_self)]
trait NftResolver {
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenKey,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>,
    ) -> bool;
}

trait NftResolver {
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenKey,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>,
    ) -> bool;
}

#[near_bindgen]
impl NftToken for Contract {
    fn nft_token(&self, token_id: TokenKey) -> Option<JsonToken> {
        //if there is some data for the token id in the token data store:
        if let Some(tokendata) = self.tokens.data_for_id.get(&token_id) {
            let token = self.tokens.info_by_id.get(&token_id).unwrap();
            //then return the wrapped JsonActor
            Some(JsonToken {
                token_id: token_id,
                owner_id: token.owner_id,
                metadata: tokendata,
            })
        } else {
            //else return None
            None
        }
    }

    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenKey,
        approval_id: u64,
        memo: Option<String>,
    ) {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();
        let token = self.transfer(&sender_id, &receiver_id, &token_id, Some(approval_id), memo);
        refund_approved_accounts(token.owner_id.clone(), &token.approved_accounts);
    }

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenKey,
        approval_id: u64,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        assert_one_yocto();

        // Validate gass before calling transfers
        let attached_gas = env::prepaid_gas();
        assert!(
            attached_gas >= MIN_GAS_FOR_NFT_TRANSFER_CALL,
            "You cannot attach less than {:?} Gas to nft_transfer_call",
            MIN_GAS_FOR_NFT_TRANSFER_CALL
        );

        let sender_id = env::predecessor_account_id();
        let token = self.transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            Some(approval_id),
            memo.clone(),
        );

        let mut authorized_id = None;
        if sender_id != token.owner_id {
            authorized_id = Some(sender_id.to_string());
        }

        // Initiating receiver's call and the callback
        ext_nft_receiver::nft_on_transfer(
            sender_id,
            token.owner_id.clone(),
            token_id.clone(),
            msg,
            receiver_id.clone(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
        )
        .then(ext_self::nft_resolve_transfer(
            authorized_id,
            token.owner_id,
            receiver_id,
            token_id,
            token.approved_accounts,
            memo,
            env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_RESOLVE_TRANSFER,
        ))
        .into()
    }
}

#[near_bindgen]
impl NftResolver for Contract {
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenKey,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>,
    ) -> bool {
        //when all went well the only thing left is to refund the storage cost
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                if !return_token {
                    refund_approved_accounts(owner_id, &approved_account_ids);
                    return true;
                }
            }
        }

        //if there is some token info, and the token is set to the new owner, undo the transaction
        let mut token = if let Some(token) = self.tokens.info_by_id.get(&token_id) {
            if token.owner_id != receiver_id {
                refund_approved_accounts(owner_id, &approved_account_ids);
                return true;
            }
            token
        } else {
            //else the contract broke, and the token was burned, refund storage
            refund_approved_accounts(owner_id, &approved_account_ids);
            return true;
        };

        //return the token to the original owner and remove it from the new owner
        self.remove_token_from_owner(&token_id, &receiver_id.clone());
        self.add_token_to_owner(&token_id, &owner_id);

        token.owner_id = owner_id.clone();

        //refund the approved IDs storage that the reciever may have set on the token
        refund_approved_accounts(receiver_id.clone(), &token.approved_accounts);

        //restore the old approved accounts information
        token.approved_accounts = approved_account_ids;
        self.tokens.info_by_id.insert(&token_id, &token);

        //log an event message for the undo transfer
        let nft_transfer_log: EventLog = EventLog {
            standard: EVENT_NFT_METADATA_SPEC.to_string(),
            version: EVENT_NFT_STANDARD_NAME.to_string(),
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                authorized_id,
                old_owner_id: receiver_id.to_string(),
                new_owner_id: owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&nft_transfer_log.to_string());

        false
    }
}

impl NftTokenEnumerator for Contract {
    fn nft_total_supply(&self) -> U128 {
        U128(self.tokens.data_for_id.len() as u128)
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.tokens
            .data_for_id
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        if let Some(tokens_for_owner_set) = self
            .tokens
            .list_per_owner
            .get(&hash_storage_key(account_id.as_bytes()))
        {
            U128(tokens_for_owner_set.len() as u128)
        } else {
            U128(0)
        }
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        if let Some(tokens_for_owner_set) = self
            .tokens
            .list_per_owner
            .get(&hash_storage_key(account_id.as_bytes()))
        {
            let start = u128::from(from_index.unwrap_or(U128(0)));
            return tokens_for_owner_set
                .iter()
                .skip(start as usize)
                .take(limit.unwrap_or(50) as usize)
                .map(|token_id| self.nft_token(token_id.clone()).unwrap())
                .collect();
        } else {
            return vec![];
        };
    }
}
