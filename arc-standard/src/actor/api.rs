use crate::*;

pub trait ArcActor {
    fn arc_actor(&self, token_id: TokenKey) -> Option<JsonActor>;

    fn arc_actor_count(&self) -> U128;

    fn arc_actors(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonActor>;

    fn arc_actor_supply_for_owner(&self, account_id: AccountId) -> U128;

    fn arc_actors_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonActor>;
}

#[macro_export]
macro_rules! impl_arc_actors {
    ($contract: ident, $tokens: ident, $actors: ident) => {
        use $crate::actor::*;
        use $crate::*;

        #[near_bindgen]
        impl ArcActor for $contract {
            fn arc_actor(&self, token_id: TokenKey) -> Option<JsonActor> {
                if let Some(actordata) = self.$actors.data_for_id.get(&token_id) {
                    let tokendata = self.$tokens.data_for_id.get(&token_id).unwrap();
                    let token = self.$tokens.info_by_id.get(&token_id).unwrap();
                    return Some(JsonActor {
                        token_id: token_id,
                        owner_id: token.owner_id,
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
                    .$tokens
                    .data_for_id
                    .keys()
                    .skip(start as usize)
                    .take(limit.unwrap_or(50) as usize)
                    .map(|token_id| self.arc_actor(token_id.clone()).unwrap())
                    .collect();
            }

            // TODO Fix Actors per owner, atm it just looks at all tokens, see registration
            fn arc_actor_supply_for_owner(&self, account_id: AccountId) -> U128 {
                if let Some(tokens_for_owner_set) = self
                    .$tokens
                    .list_per_owner
                    .get(&hash_storage_key(account_id.as_bytes()))
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
                // TODO Swap to actors for owner or filet
                if let Some(tokens_for_owner_set) = self
                    .$tokens
                    .list_per_owner
                    .get(&hash_storage_key(account_id.as_bytes()))
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
        }
    };
}
