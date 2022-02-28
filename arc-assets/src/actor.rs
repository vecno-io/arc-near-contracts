use crate::*;

use arc_shared::JsonActor;
use near_sdk::json_types::U128;
use near_sdk::require;
use std::collections::HashMap;

pub trait ArcActor {
    //view call for returning the actor data for the provided id
    fn arc_actor(&self, token_id: TokenId) -> Option<JsonActor>;

    //mint a new actor with the provided data, returns the token id
    fn arc_mint_actor(
        &mut self,
        receiver_id: AccountId,
        actor_data: ActorData,
        token_data: TokenData,
        royalties: Option<HashMap<AccountId, u32>>,
        memo: Option<String>,
    ) -> TokenId;
}

pub trait ArcActorEnumerator {
    fn arc_actor_supply(&self) -> U128;

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
    fn arc_mint_actor(
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

        let storage_usage = env::storage_usage();

        //verify royalties
        let mut total = 0;
        let mut royalty = HashMap::new();
        if let Some(royalties) = royalties {
            require!(
                royalties.len() < 11,
                "Cannot add more than 10 royalty amounts per token"
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

        // DEV temp use the guild count as a unique ID
        // TODO Generate a unique id based on guild data
        let cnt = self.guild_count.get().unwrap();
        self.guild_count.set(&(cnt + 1));
        let id = cnt.to_string();

        //create the token and store it
        let token = Token {
            owner_id: receiver_id,
            royalty: royalty,
            approval_index: 0,
            approved_accounts: Default::default(),
        };
        require!(
            self.tokens_by_id.insert(&id, &token).is_none(),
            "A token with the generated id already exits"
        );
        self.actordata_by_id.insert(&id, &actor_data);
        self.tokendata_by_id.insert(&id, &token_data);
        self.add_token_to_owner(&id, &token.owner_id);

        //log an event message for the mint
        let nft_mint_log: EventLog = EventLog {
            version: EVENT_NFT_METADATA_SPEC.to_string(),
            standard: EVENT_NFT_STANDARD_NAME.to_string(),
            event: EventLogVariant::NftMint(vec![NftMintLog {
                owner_id: token.owner_id.to_string(),
                token_ids: vec![id.to_string()],
                memo,
            }]),
        };
        env::log_str(&nft_mint_log.to_string());

        //refund unused storage fees and return the id to the caller
        refund_storage_deposit(env::storage_usage() - storage_usage);
        id
    }
}

impl ArcActorEnumerator for Contract {
    fn arc_actor_supply(&self) -> U128 {
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
        if let Some(tokens_for_owner_set) = self.tokens_per_owner.get(&account_id) {
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
        if let Some(tokens_for_owner_set) = self.tokens_per_owner.get(&account_id) {
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
