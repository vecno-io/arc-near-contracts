use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, require, Promise};
use near_sdk::{AccountId, Gas, PanicOnDefault, PromiseOrValue, PromiseResult};

use arc_standard::{actor::Actors, token::Tokens};

const MIN_MINT_COST: u128 = 100_000_000_000_000_000_000_000;

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

#[ext_contract(ext_token_resolve)]
pub trait ExtTokenCResolver {
    fn arc_mint_actor_link_callback(
        &mut self,
        contract_id: AccountId,
        sender_id: AccountId,
        linked_id: TokenId,
        token_id: TokenId,
        actor_data: ActorData,
        token_data: TokenData,
        token_payout: TokenPayout,
        guild_id: Option<GuildId>,
    ) -> bool;
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
        require!(
            env::attached_deposit() >= MIN_MINT_COST,
            // TODO Set a more flexible mint cost
            "minimum mint cost is 0.1 near"
        );

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
    }

    #[payable]
    pub fn arc_mint_actor_link(
        &mut self,
        contract_id: AccountId,
        linked_id: TokenId,
        token_id: TokenId,
        actor_data: ActorData,
        token_data: TokenData,
        token_payout: TokenPayout,
        guild_id: Option<GuildId>,
    ) -> Promise {
        require!(
            env::attached_deposit() >= MIN_MINT_COST,
            // TODO Set a more flexible mint cost
            "minimum mint cost is 0.1 near"
        );

        token_data.require_valid();
        token_payout.require_valid();

        require!(
            self.tokens.info_by_id.get(&token_id).is_none(),
            "a token with the provided id already exits"
        );

        let sender_id = env::predecessor_account_id();

        ext_token_call::nft_token(token_id.clone(), contract_id.clone(), 0, GAS_NFT_TOKEN)
            .then(ext_token_resolve::arc_mint_actor_link_callback(
                contract_id,
                sender_id,
                linked_id,
                token_id,
                actor_data,
                token_data,
                token_payout,
                guild_id,
                env::current_account_id(),
                0,
                GAS_NFT_TOKEN_CALLBACK,
            ))
            .as_return()
    }

    pub fn arc_mint_actor_link_callback(
        &mut self,
        contract_id: AccountId,
        sender_id: AccountId,
        linked_id: TokenId,
        token_id: TokenId,
        actor_data: ActorData,
        token_data: TokenData,
        token_payout: TokenPayout,
        guild_id: Option<GuildId>,
    ) {
        require!(
            env::promise_results_count() == 1,
            "the method can only be called as a callback"
        );

        let info = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic_str("failed to get token info"),
            PromiseResult::Successful(result) => {
                near_sdk::serde_json::from_slice::<Option<JsonToken>>(&result).unwrap()
            }
        };

        require!(info.is_some(), "invalid token info");
        if let Some(token) = info {
            require!(token.owner_id == sender_id, "invalid owner link");
            require!(token.token_id == linked_id, "invalid token link");
        }

        let owner = OwnerIds {
            account: contract_id,
            guild_id: guild_id,
            token_id: Some(linked_id),
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
    }
}
