use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, require};
use near_sdk::{AccountId, PanicOnDefault, PromiseOrValue, PromiseResult};

use arc_standard::{actor::Actors, token::Tokens};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ArcActors {
    meta: LazyOption<Metadata>,
    actors: Actors,
    tokens: Tokens,
}

arc_standard::impl_meta!(ArcActors, meta);
arc_standard::impl_arc_actors!(ArcActors, tokens, actors);
arc_standard::impl_nft_tokens!(ArcActors, tokens, actors);

#[derive(BorshSerialize)]
pub enum StorageKey {
    AppAdmin,
    AppMetadata,
}

#[near_bindgen]
impl ArcActors {
    #[init]
    pub fn new(app_metadata: Metadata) -> Self {
        require!(!env::state_exists(), "already initialized");
        app_metadata.require_valid();

        Self {
            actors: Actors::new(),
            tokens: Tokens::new(),
            meta: LazyOption::new(
                StorageKey::AppMetadata.try_to_vec().unwrap(),
                Some(&app_metadata),
            ),
        }
    }

    #[init]
    pub fn new_default() -> Self {
        require!(!env::state_exists(), "already initialized");
        Self::new(Metadata {
            //spec: ARC_STANDARD_SPEC.to_string(),
            spec: "arc-1.0.0".to_string(),
            name: "Arc Actors".to_string(),
            symbol: "ARC".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        })
    }
}

#[near_bindgen]
impl ArcActors {
    #[payable]
    pub fn arc_mint_actor(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        actor_data: ActorData,
        token_data: TokenData,
        token_payout: TokenPayout,
        guild_id: Option<GuildId>,
    ) {
        require_min_one_yocto();
        let storage_usage = env::storage_usage();

        let owner = OwnerIds {
            account: owner_id,
            guild_id: guild_id,
            token_id: None,
        };
        self.tokens.register(
            &owner,
            &token_id,
            TokenType::Actor,
            token_data,
            token_payout,
            None,
        );
        self.actors.register(&owner, &token_id, actor_data, None);

        refund_storage_deposit(env::storage_usage() - storage_usage);
    }
}
