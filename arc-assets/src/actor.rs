use crate::*;

use arc_shared::JsonActor;
use near_sdk::json_types::U128;
use near_sdk::require;
use std::collections::HashMap;

pub trait ArcActor {
    //view call for returning the actor data for the provided id
    fn arc_actor(&self, token_id: TokenId) -> Option<JsonActor>;

    //register a new actor with the provided data, returns the token id
    fn arc_register_actor(
        &mut self,
        receiver_id: AccountId,
        actor_data: ActorData,
        token_data: TokenData,
        royalties: Option<HashMap<AccountId, u32>>,
        memo: Option<String>,
    ) -> TokenId;
}

pub trait ArcActorEnumerator {
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

#[near_bindgen]
impl ArcActor for Contract {
    fn arc_actor(&self, token_id: TokenId) -> Option<JsonActor> {
        //if there is some data for the actor id in the actor data store:
        if let Some(actordata) = self.actordata_by_id.get(&token_id) {
            let tokendata = self.tokendata_by_id.get(&token_id).unwrap();
            let token = self.tokens_by_id.get(&token_id).unwrap();
            //then return the wrapped JsonActor
            Some(JsonActor {
                token_id: token_id,
                owner_id: token.owner_id,
                tokendata,
                actordata,
            })
        } else {
            //else return None
            None
        }
    }

    #[payable]
    fn arc_register_actor(
        &mut self,
        receiver_id: AccountId,
        actor_data: ActorData,
        token_data: TokenData,
        royalties: Option<HashMap<AccountId, u32>>,
        memo: Option<String>,
    ) -> TokenId {
        assert_min_one_yocto();
        actor_data.assert_valid();
        token_data.assert_valid();

        let asset_data = self
            .contract_data
            .get()
            .expect("Contract to be initialized");

        let storage_usage = env::storage_usage();

        //verify royalties
        let mut total = 0;
        let mut royalty = HashMap::new();
        if let Some(royalties) = royalties {
            require!(
                royalties.len() < 5,
                "Cannot add more than 4 royalty amounts per actor"
            );
            for (account, amount) in royalties {
                royalty.insert(account, amount);
                total += amount;
            }
            require!(
                total <= MAX_TOTAL_ROYALTIES,
                "The total of all royalties can not be larger than 10000"
            );
        }

        //create the token and store it
        let token_id = format!(
            "{}:Actor:{:06}",
            asset_data.symbol,
            self.actordata_by_id.len()
        );
        let token = Token {
            owner_id: receiver_id,
            royalty: royalty,
            approval_index: 0,
            approved_accounts: Default::default(),
        };
        require!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "A token with the generated id already exits"
        );
        self.actordata_by_id.insert(&token_id, &actor_data);
        self.tokendata_by_id.insert(&token_id, &token_data);

        let created = self.add_token_to_owner(&token_id, &token.owner_id);

        //log an event message for the mint
        let nft_mint_log: EventLog = EventLog {
            version: EVENT_NFT_METADATA_SPEC.to_string(),
            standard: EVENT_NFT_STANDARD_NAME.to_string(),
            event: EventLogVariant::NftMint(vec![NftMintLog {
                owner_id: token.owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo: memo.clone(),
            }]),
        };
        env::log_str(&nft_mint_log.to_string());
        let arc_mint_log: EventLog = EventLog {
            version: EVENT_ARC_METADATA_SPEC.to_string(),
            standard: EVENT_ARC_STANDARD_NAME.to_string(),
            event: EventLogVariant::ArcMint(vec![ArcMintLog {
                owner_id: token.owner_id.to_string(),
                token_type: "arc:actor".to_string(),
                token_list: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&arc_mint_log.to_string());

        //refund unused storage fees and return the id to the caller,
        let key_cost = if created { 0 } else { 32 }; //edge: one owner per token
        refund_storage_deposit((env::storage_usage() - storage_usage) + key_cost);
        token_id
    }
}

impl ArcActorEnumerator for Contract {
    fn arc_actor_count(&self) -> U128 {
        U128(self.actordata_by_id.len() as u128)
    }

    fn arc_actors(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonActor> {
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.tokendata_by_id
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.arc_actor(token_id.clone()).unwrap())
            .collect()
    }

    fn arc_actor_supply_for_owner(&self, account_id: AccountId) -> U128 {
        if let Some(tokens_for_owner_set) = self
            .tokens_per_owner
            .get(&hash_storage_key(account_id.as_bytes()))
        {
            U128(tokens_for_owner_set.len() as u128)
        } else {
            U128(0)
        }
    }

    fn arc_actors_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonActor> {
        if let Some(tokens_for_owner_set) = self
            .tokens_per_owner
            .get(&hash_storage_key(account_id.as_bytes()))
        {
            let start = u128::from(from_index.unwrap_or(U128(0)));
            return tokens_for_owner_set
                .iter()
                .skip(start as usize)
                .take(limit.unwrap_or(50) as usize)
                .map(|token_id| self.arc_actor(token_id.clone()).unwrap())
                .collect();
        } else {
            return vec![];
        };
    }
}
