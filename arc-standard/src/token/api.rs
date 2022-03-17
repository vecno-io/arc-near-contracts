use crate::*;

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

#[macro_export]
macro_rules! impl_nft_tokens {
    ($contract: ident, $tokens: ident, $actors: ident, $guilds: ident) => {
        use $crate::*;

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
                        owner_id: token.owner_id,
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
                assert_one_yocto();
                let sender_id = env::predecessor_account_id();
                let token =
                    self.$tokens
                        .transfer(&token_id, &sender_id, &receiver_id, approval_id, memo);

                if (token.type_id == TokenType::Actor) {
                    self.$actors.transfer(&token_id, &sender_id, &receiver_id);
                }

                refund_approved_accounts(token.owner_id.clone(), &token.approved_accounts);
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
                assert_one_yocto();

                let attached_gas = env::prepaid_gas();
                assert!(
                    attached_gas >= MIN_GAS_FOR_NFT_TRANSFER_CALL,
                    "You cannot attach less than {:?} Gas to nft_transfer_call",
                    MIN_GAS_FOR_NFT_TRANSFER_CALL
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
                if sender_id != token.owner_id {
                    authorized_id = Some(sender_id.to_string());
                }

                return ext_nft_receiver::nft_on_transfer(
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
                    .expect("Token not found");
                if let Some(approval) = token.approved_accounts.get(&approved_account_id) {
                    if let Some(approval_id) = approval_id {
                        return approval_id == *approval;
                    }
                    return true;
                }
                return false;
            }

            fn nft_approve(
                &mut self,
                token_id: TokenId,
                account_id: AccountId,
                msg: Option<String>,
            ) {
                assert_min_one_yocto();

                let mut token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id)
                    .expect("Token not found");
                assert_eq!(
                    &env::predecessor_account_id(),
                    &token.owner_id,
                    "Signer must be the token owner."
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
                        token.owner_id,
                        approval_id,
                        msg,
                        account_id,
                        NO_DEPOSIT,
                        env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
                    )
                    .as_return();
                }
            }

            fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
                assert_one_yocto();

                let mut token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id)
                    .expect("Token not found");
                let sender_id = env::predecessor_account_id();
                assert_eq!(
                    &sender_id, &token.owner_id,
                    "Signer must be the token owner."
                );

                if token.approved_accounts.remove(&account_id).is_some() {
                    refund_approved_account_ids_iter(sender_id, [account_id].iter());
                    self.$tokens.info_by_id.insert(&token_id, &token);
                }
            }

            fn nft_revoke_all(&mut self, token_id: TokenId) {
                assert_one_yocto();

                let mut token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id)
                    .expect("Token not found");
                let sender_id = env::predecessor_account_id();
                assert_eq!(
                    &sender_id, &token.owner_id,
                    "Signer must be the token owner."
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
                    .expect("Token not found");

                let mut guild_account: Option<AccountId> = None;
                if let Some(guild_id) = token.guild_id {
                    if let Some(guild_info) = self.$guilds.info_by_id.get(&guild_id) {
                        guild_account = guild_info.payout_id;
                    }
                }

                return token.payout.compute(
                    balance.into(),
                    max_len_payout,
                    token.owner_id,
                    guild_account,
                );
            }

            fn nft_transfer_payout(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                balance: U128,
                max_len_payout: u32,
            ) -> JsonPayout {
                assert_one_yocto();
                let sender_id = env::predecessor_account_id();
                let token =
                    self.$tokens
                        .transfer(&token_id, &sender_id, &receiver_id, approval_id, memo);

                if (token.type_id == TokenType::Actor) {
                    self.$actors.transfer(&token_id, &sender_id, &receiver_id);
                }

                refund_approved_accounts(token.owner_id.clone(), &token.approved_accounts);

                let mut guild_account: Option<AccountId> = None;
                if let Some(guild_id) = token.guild_id {
                    if let Some(guild_info) = self.$guilds.info_by_id.get(&guild_id) {
                        guild_account = guild_info.payout_id;
                    }
                }

                return token.payout.compute(
                    balance.into(),
                    max_len_payout,
                    token.owner_id,
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
                    if token.owner_id != receiver_id {
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

                token.owner_id = owner_id.clone();
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
