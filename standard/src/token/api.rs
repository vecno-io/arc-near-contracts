use super::*;

use std::mem::size_of;

pub const NO_DEPOSIT: Balance = 0;
pub const MAX_BASE_POINTS_TOTAL: u16 = 10000;

pub const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);
pub const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
pub const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(35_000_000_000_000);
pub const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);

pub trait NftCore {
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;

    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;
}

pub trait NftApproval {
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;

    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);

    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    fn nft_revoke_all(&mut self, token_id: TokenId);
}

pub trait NftRoyalties {
    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> JsonPayout;

    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> JsonPayout;
}

pub trait NftEnumeration {
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

// ==== Helper Functions ====

//calculate how many bytes the account ID is taking up
#[inline(always)]
pub fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    // The extra 4 bytes are coming from Borsh serialization to store the length of the string.
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

//refund the storage taken up by passed in approved account IDs and send the funds to the passed in account ID.
pub fn refund_approved_account_ids_iter<'a, I>(
    account_id: AccountId,
    approved_accounts: I,
) -> Promise
where
    I: Iterator<Item = &'a AccountId>,
{
    //get the storage total by going through and summing all the bytes for each approved account IDs
    let storage_released: u64 = approved_accounts.map(bytes_for_approved_account_id).sum();
    Promise::new(account_id).transfer(Balance::from(storage_released) * env::storage_byte_cost())
}

//refund the initial deposit based on the amount of storage that was used up
pub fn refund_storage_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();
    let finalized_refund = attached_deposit - required_cost;
    //make sure that the attached deposit is greater than or equal to the required cost
    require!(
        required_cost <= attached_deposit,
        format!("Must attach {} yocto to cover storage cost", required_cost),
    );

    if finalized_refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(finalized_refund);
    }
}

//refund a map of approved account IDs and send the funds to the passed in account ID
#[inline(always)]
pub fn refund_approved_accounts(
    account_id: AccountId,
    approved_accounts: &HashMap<AccountId, u64>,
) -> Promise {
    //call the refund_approved_account_ids_iter with the approved account IDs as keys
    refund_approved_account_ids_iter(account_id, approved_accounts.keys())
}

#[macro_export]
macro_rules! impl_nft_tokens {
    ($contract: ident, $tokens: ident, $actors: ident) => {
        use std::collections::HashMap;
        use $crate::actor::*;
        use $crate::event::*;
        use $crate::token::*;
        use $crate::*;

        pub const GAS_NFT_TOKEN: Gas = Gas(80_000_000_000_000);
        pub const GAS_NFT_TOKEN_CALLBACK: Gas = Gas(10_000_000_000_000);

        #[ext_contract(ext_token_call)]
        pub trait ExtTokenCall {
            fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
        }

        #[ext_contract(ext_nft_receiver)]
        trait NftReceiver {
            fn nft_on_transfer(
                &mut self,
                sender_id: AccountId,
                previous_owner_id: AccountId,
                token_id: TokenId,
                msg: String,
            ) -> Promise;

            fn nft_on_approve(
                &mut self,
                token_id: TokenId,
                owner_id: AccountId,
                approval_id: u64,
                msg: String,
            );
        }

        #[ext_contract(ext_self)]
        trait NftResolver {
            fn nft_resolve_transfer(
                &mut self,
                authorized_id: Option<String>,
                owner_id: AccountId,
                receiver_id: AccountId,
                token_id: TokenId,
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
                token_id: TokenId,
                approved_account_ids: HashMap<AccountId, u64>,
                memo: Option<String>,
            ) -> bool;
        }

        #[near_bindgen]
        impl NftCore for $contract {
            fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
                if let Some(tokendata) = self.$tokens.data_for_id.get(&token_id) {
                    let token = self.$tokens.info_by_id.get(&token_id).unwrap();
                    return Some(JsonToken {
                        token_id: token_id,
                        owner_id: token.owner.account,
                        metadata: tokendata,
                    });
                }
                return None;
            }

            #[payable]
            fn nft_transfer(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
            ) {
                require_one_yocto();
                let sender_id = env::predecessor_account_id();
                let token =
                    self.$tokens
                        .transfer(&token_id, &sender_id, &receiver_id, approval_id, memo);

                if (token.type_id == TokenType::Actor) {
                    self.$actors.transfer(&token_id, &sender_id, &receiver_id);
                }

                refund_approved_accounts(token.owner.account.clone(), &token.approved_accounts);
            }

            #[payable]
            fn nft_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<bool> {
                require_one_yocto();

                let attached_gas = env::prepaid_gas();
                require!(
                    attached_gas >= MIN_GAS_FOR_NFT_TRANSFER_CALL,
                    format!(
                        "you cannot attach less than {:?} Gas to nft_transfer_call",
                        MIN_GAS_FOR_NFT_TRANSFER_CALL
                    )
                );

                let sender_id = env::predecessor_account_id();
                let token = self.$tokens.transfer(
                    &token_id,
                    &sender_id,
                    &receiver_id,
                    approval_id,
                    memo.clone(),
                );

                if (token.type_id == TokenType::Actor) {
                    self.$actors.transfer(&token_id, &sender_id, &receiver_id);
                }

                let mut authorized_id = None;
                if sender_id != token.owner.account {
                    authorized_id = Some(sender_id.to_string());
                }

                return ext_nft_receiver::nft_on_transfer(
                    sender_id,
                    token.owner.account.clone(),
                    token_id.clone(),
                    msg,
                    receiver_id.clone(),
                    NO_DEPOSIT,
                    env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL,
                )
                .then(ext_self::nft_resolve_transfer(
                    authorized_id,
                    token.owner.account,
                    receiver_id,
                    token_id,
                    token.approved_accounts,
                    memo,
                    env::current_account_id(),
                    NO_DEPOSIT,
                    GAS_FOR_RESOLVE_TRANSFER,
                ))
                .into();
            }
        }

        #[near_bindgen]
        impl NftApproval for $contract {
            fn nft_is_approved(
                &self,
                token_id: TokenId,
                approved_account_id: AccountId,
                approval_id: Option<u64>,
            ) -> bool {
                let token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id)
                    .expect("token not found");

                if (token.owner.token_id.is_some()) {
                    return false;
                }

                if let Some(approval) = token.approved_accounts.get(&approved_account_id) {
                    if let Some(approval_id) = approval_id {
                        return approval_id == *approval;
                    }
                    return true;
                }
                return false;
            }

            #[payable]
            fn nft_approve(
                &mut self,
                token_id: TokenId,
                account_id: AccountId,
                msg: Option<String>,
            ) {
                require_min_one_yocto();

                let mut token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id)
                    .expect("token not found");

                require!(
                    &env::predecessor_account_id() == &token.owner.account,
                    "signer must be the token owner"
                );
                require!(
                    token.owner.token_id.is_none(),
                    "can not approve transfers for tokens owned by other tokens"
                );

                let approval_id: u64 = token.approval_index;
                token.approval_index += 1;

                let is_new_approval = token
                    .approved_accounts
                    .insert(account_id.clone(), approval_id)
                    .is_none();
                self.$tokens.info_by_id.insert(&token_id, &token);

                let storage_used = if is_new_approval {
                    bytes_for_approved_account_id(&account_id)
                } else {
                    0
                };
                refund_storage_deposit(storage_used);

                if let Some(msg) = msg {
                    ext_nft_receiver::nft_on_approve(
                        token_id,
                        token.owner.account,
                        approval_id,
                        msg,
                        account_id,
                        NO_DEPOSIT,
                        env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
                    )
                    .as_return();
                }
            }

            #[payable]
            fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
                require_one_yocto();

                let mut token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id)
                    .expect("token not found");
                require!(
                    token.owner.token_id.is_none(),
                    "tokens owned by other tokens have no approval"
                );

                let sender_id = env::predecessor_account_id();
                require!(
                    &sender_id == &token.owner.account,
                    "signer must be the token owner"
                );

                if token.approved_accounts.remove(&account_id).is_some() {
                    refund_approved_account_ids_iter(sender_id, [account_id].iter());
                    self.$tokens.info_by_id.insert(&token_id, &token);
                }
            }

            #[payable]
            fn nft_revoke_all(&mut self, token_id: TokenId) {
                require_one_yocto();

                let mut token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id)
                    .expect("token not found");
                require!(
                    token.owner.token_id.is_none(),
                    "tokens owned by other tokens have no approval"
                );

                let sender_id = env::predecessor_account_id();
                require!(
                    &sender_id == &token.owner.account,
                    "signer must be the token owner"
                );

                if !token.approved_accounts.is_empty() {
                    refund_approved_accounts(sender_id, &token.approved_accounts);
                    token.approved_accounts.clear();
                    self.$tokens.info_by_id.insert(&token_id, &token);
                }
            }
        }

        #[near_bindgen]
        impl NftRoyalties for $contract {
            fn nft_payout(
                &self,
                token_id: String,
                balance: U128,
                max_len_payout: u32,
            ) -> JsonPayout {
                let token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id.into())
                    .expect("token not found");
                require!(
                    token.owner.token_id.is_none(),
                    "can not payout tokens owned by other tokens"
                );
                return token.payout.compute(
                    balance.into(),
                    max_len_payout,
                    token.owner.account,
                    None,
                );
            }

            #[payable]
            fn nft_transfer_payout(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                balance: U128,
                max_len_payout: u32,
            ) -> JsonPayout {
                require_one_yocto();
                let sender_id = env::predecessor_account_id();
                let token =
                    self.$tokens
                        .transfer(&token_id, &sender_id, &receiver_id, approval_id, memo);

                if (token.type_id == TokenType::Actor) {
                    self.$actors.transfer(&token_id, &sender_id, &receiver_id);
                }

                refund_approved_accounts(token.owner.account.clone(), &token.approved_accounts);

                let mut guild_account: Option<AccountId> = None;

                return token.payout.compute(
                    balance.into(),
                    max_len_payout,
                    token.owner.account,
                    guild_account,
                );
            }
        }

        #[near_bindgen]
        impl NftEnumeration for $contract {
            fn nft_total_supply(&self) -> U128 {
                return U128(self.$tokens.data_for_id.len() as u128);
            }

            fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
                let start = u128::from(from_index.unwrap_or(U128(0)));
                return self
                    .$tokens
                    .data_for_id
                    .keys()
                    .skip(start as usize)
                    .take(limit.unwrap_or(50) as usize)
                    .map(|token_id| self.nft_token(token_id.clone()).unwrap())
                    .collect();
            }

            fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
                if let Some(tokens_for_owner_set) =
                    self.$tokens.list_per_owner.get(&account_id.into())
                {
                    return U128(tokens_for_owner_set.len() as u128);
                }
                return U128(0);
            }

            fn nft_tokens_for_owner(
                &self,
                account_id: AccountId,
                from_index: Option<U128>,
                limit: Option<u64>,
            ) -> Vec<JsonToken> {
                if let Some(tokens_for_owner_set) =
                    self.$tokens.list_per_owner.get(&account_id.into())
                {
                    let start = u128::from(from_index.unwrap_or(U128(0)));
                    return tokens_for_owner_set
                        .iter()
                        .skip(start as usize)
                        .take(limit.unwrap_or(50) as usize)
                        .map(|token_id| self.nft_token(token_id.clone()).unwrap())
                        .collect();
                }
                return vec![];
            }
        }

        #[near_bindgen]
        impl NftResolver for $contract {
            #[private]
            fn nft_resolve_transfer(
                &mut self,
                authorized_id: Option<String>,
                owner_id: AccountId,
                receiver_id: AccountId,
                token_id: TokenId,
                approved_account_ids: HashMap<AccountId, u64>,
                memo: Option<String>,
            ) -> bool {
                if let PromiseResult::Successful(value) = env::promise_result(0) {
                    if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                        if !return_token {
                            refund_approved_accounts(owner_id, &approved_account_ids);
                            return true;
                        }
                    }
                }

                let mut token = if let Some(token) = self.$tokens.info_by_id.get(&token_id) {
                    if token.owner.account != receiver_id {
                        refund_approved_accounts(owner_id, &approved_account_ids);
                        return true;
                    }
                    token
                } else {
                    refund_approved_accounts(owner_id, &approved_account_ids);
                    return true;
                };

                self.$tokens
                    .remove_from_owner(receiver_id.clone().into(), &token_id);
                self.$tokens
                    .add_to_owner(owner_id.clone().into(), &token_id);

                token.owner.account = owner_id.clone();
                refund_approved_accounts(receiver_id.clone(), &token.approved_accounts);

                token.approved_accounts = approved_account_ids;
                self.$tokens.info_by_id.insert(&token_id, &token);

                let nft_transfer_log: JsonEventLog = JsonEventLog {
                    standard: EVENT_NFT_METADATA_SPEC.to_string(),
                    version: EVENT_NFT_STANDARD_NAME.to_string(),
                    event: JsonEventVariant::NftTransfer(vec![NftTransferLog {
                        authorized_id,
                        old_owner_id: receiver_id.to_string(),
                        new_owner_id: owner_id.to_string(),
                        token_ids: vec![token_id.to_string()],
                        memo,
                    }]),
                };
                env::log_str(&nft_transfer_log.to_string());

                return false;
            }
        }
    };
}
