use super::*;

// ==== Actor Enumeration ====

pub trait ArcEnumeration {
    fn arc_actor(&self, actor_id: TokenId) -> Option<JsonActor>;

    fn arc_actor_count(&self) -> U128;

    fn arc_actors(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonActor>;

    fn arc_actor_count_for_owner(&self, account_id: AccountId) -> U128;

    fn arc_actors_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonActor>;

    fn arc_actor_for_token(&self, token_id: TokenId) -> Option<JsonActor>;
}

// ==== Actor Internal ====

pub trait ArcInternal {
    fn set_link(
        &mut self,
        token_id: &TokenId,
        owner_account: &AccountId,
        owner_token_id: Option<TokenId>,
    );
}

#[macro_export]
macro_rules! impl_arc_actors {
    ($contract: ident, $tokens: ident, $actors: ident) => {
        use $crate::actor::*;
        use $crate::share::*;
        use $crate::*;

        #[near_bindgen]
        impl ArcEnumeration for $contract {
            fn arc_actor(&self, actor_id: TokenId) -> Option<JsonActor> {
                if let Some(actordata) = self.$actors.data_for_id.get(&actor_id) {
                    let tokendata = self.$tokens.data_for_id.get(&actor_id).unwrap();
                    let token = self.$tokens.info_by_id.get(&actor_id).unwrap();
                    return Some(JsonActor {
                        token_id: actor_id,
                        ownerdata: token.owner,
                        tokendata,
                        actordata,
                    });
                }
                return None;
            }

            fn arc_actor_count(&self) -> U128 {
                return U128(self.$actors.data_for_id.len() as u128);
            }

            fn arc_actors(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonActor> {
                let start = u128::from(from_index.unwrap_or(U128(0)));
                return self
                    .$actors
                    .data_for_id
                    .keys()
                    .skip(start as usize)
                    .take(limit.unwrap_or(50) as usize)
                    .map(|token_id| self.arc_actor(token_id.clone()).unwrap())
                    .collect();
            }

            fn arc_actor_count_for_owner(&self, account_id: AccountId) -> U128 {
                if let Some(tokens_for_owner_set) =
                    self.$actors.list_per_owner.get(&account_id.into())
                {
                    return U128(tokens_for_owner_set.len() as u128);
                }
                return U128(0);
            }

            fn arc_actors_for_owner(
                &self,
                account_id: AccountId,
                from_index: Option<U128>,
                limit: Option<u64>,
            ) -> Vec<JsonActor> {
                if let Some(tokens_for_owner_set) =
                    self.$actors.list_per_owner.get(&account_id.into())
                {
                    let start = u128::from(from_index.unwrap_or(U128(0)));
                    return tokens_for_owner_set
                        .iter()
                        .skip(start as usize)
                        .take(limit.unwrap_or(50) as usize)
                        .map(|token_id| self.arc_actor(token_id.clone()).unwrap())
                        .collect();
                }
                return vec![];
            }

            fn arc_actor_for_token(&self, token_id: TokenId) -> Option<JsonActor> {
                let actor_id = self.actors.link_for_token.get(&token_id).expect("");
                return self.arc_actor(actor_id);
            }
        }

        impl ArcInternal for $contract {
            fn set_link(
                &mut self,
                token_id: &TokenId,
                owner_account: &AccountId,
                owner_token_id: Option<TokenId>,
            ) {
                let token = self
                    .$tokens
                    .info_by_id
                    .get(&token_id)
                    .expect("token info not found");

                let new_token = Token {
                    type_id: token.type_id,
                    owner: OwnerIds {
                        account: owner_account.clone(),
                        guild_id: token.owner.guild_id.clone(),
                        token_id: owner_token_id.clone(),
                    },
                    payout: token.payout,
                    approval_index: 0,
                    approved_accounts: Default::default(),
                };
                self.$tokens.info_by_id.insert(token_id, &new_token);

                // Update token and actor ownership lists
                let current = token.owner.account.clone();
                self.$tokens.remove_from_owner(current.clone(), token_id);
                self.$tokens.add_to_owner(owner_account.clone(), token_id);
                self.$actors.remove_from_owner(current.clone(), token_id);
                self.$actors.add_to_owner(owner_account.clone(), token_id);

                // Update the token ownership map
                if let Some(id) = token.owner.token_id {
                    self.$actors.link_for_token.remove(&id);
                }
                if let Some(id) = owner_token_id {
                    self.$actors.link_for_token.insert(&id, token_id);
                }
            }
        }
    };
}
