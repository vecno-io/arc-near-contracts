use crate::*;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonActor {
    //token ID
    pub token_id: TokenKey,
    //owner of the token
    pub owner_id: AccountId,
    //token metadata
    pub tokendata: TokenData,
    //token actordata
    pub actordata: ActorData,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ActorData {
    pub name: Name,
    pub persona: Persona,
    pub ancestry: Ancestry,
    pub attributes: Attributes,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Name {
    pub last: String,
    pub first: String,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Persona {
    pub root: u8,
    pub style: u8,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Social {
    pub age: u8,
    pub style: u8,
    pub economy: u8,
    pub community: u8,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Habitat {
    pub age: u8,
    pub style: u8,
    pub scale: u8,
    pub nature: u8,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ancestry {
    pub social: Social,
    pub habitat: Habitat,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Attributes {
    pub dexterity: u8,
    pub empathy: u8,
    pub intellect: u8,

    pub perception: u8,
    pub presence: u8,
    pub stamina: u8,

    pub strength: u8,
    pub vitality: u8,
    pub wisdom: u8,
}

pub trait ArcActor {
    //view call for returning the actor data for the provided id
    fn arc_actor(&self, token_id: TokenKey) -> Option<JsonActor>;

    //register a new actor with the provided data, returns the token id
    fn arc_register_actor(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenKey,
        token_data: TokenData,
        actor_data: ActorData,
        royalties: Option<HashMap<AccountId, u32>>,
        memo: Option<String>,
    ) -> TokenKey;
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

impl ActorData {
    pub fn assert_valid(&self) {
        self.name.assert_valid();
    }
}

impl Name {
    pub fn assert_valid(&self) {
        // Note: Look in to regex in near
        // https://github.com/rust-lang/regex
        require!(
            &self.last.len() < &16,
            "Last name can not be longer than 16 characters"
        );
        require!(
            &self.first.len() > &2 && &self.first.len() < &16,
            "First name needs to be between 2 and 16 characters"
        );
    }
}

#[macro_export]
macro_rules! impl_arc_actors {
    ($contract: ident, $tokens: ident, $actors: ident) => {
        use $crate::*;

        #[near_bindgen]
        impl ArcActor for $contract {
            fn arc_actor(&self, token_id: TokenKey) -> Option<JsonActor> {
                //if there is some data for the actor id in the actor data store:
                if let Some(actordata) = self.$actors.data_for_id.get(&token_id) {
                    let tokendata = self.$tokens.data_for_id.get(&token_id).unwrap();
                    let token = self.$tokens.info_by_id.get(&token_id).unwrap();
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
                token_id: TokenKey,
                token_data: TokenData,
                actor_data: ActorData,
                royalties: Option<HashMap<AccountId, u32>>,
                memo: Option<String>,
            ) -> TokenKey {
                assert_min_one_yocto();
                actor_data.assert_valid();
                token_data.assert_valid();

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

                // TODO: move to contract implementaion
                //create the token and store it
                // let token_id = format!(
                //     "{}:Actor:{:06}",
                //     asset_data.symbol,
                //     self.actors.data_for_id.len()
                // );

                let token = Token {
                    type_id: TokenType::Actor,
                    owner_id: receiver_id,
                    royalty: royalty,
                    approval_index: 0,
                    approved_accounts: Default::default(),
                };
                require!(
                    self.$tokens.info_by_id.insert(&token_id, &token).is_none(),
                    "A token with the provided id already exits"
                );
                self.$actors.data_for_id.insert(&token_id, &actor_data);
                self.$tokens.data_for_id.insert(&token_id, &token_data);

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

        impl ArcActorEnumerator for $contract {
            fn arc_actor_count(&self) -> U128 {
                U128(self.$actors.data_for_id.len() as u128)
            }

            fn arc_actors(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonActor> {
                let start = u128::from(from_index.unwrap_or(U128(0)));
                self.$tokens
                    .data_for_id
                    .keys()
                    .skip(start as usize)
                    .take(limit.unwrap_or(50) as usize)
                    .map(|token_id| self.arc_actor(token_id.clone()).unwrap())
                    .collect()
            }

            fn arc_actor_supply_for_owner(&self, account_id: AccountId) -> U128 {
                if let Some(tokens_for_owner_set) = self
                    .$tokens
                    .list_per_owner
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
                } else {
                    return vec![];
                };
            }
        }
    };
}
