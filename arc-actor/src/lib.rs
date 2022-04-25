use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, require, Promise};
use near_sdk::{AccountId, Gas, PanicOnDefault, PromiseOrValue, PromiseResult};

use arc_standard::{actor::Actors, token::Tokens};

const MIN_MINT_COST: u128 = 100_000_000_000_000_000_000_000;

pub const GAS_LINK_TOKEN: Gas = Gas(80_000_000_000_000);
pub const GAS_LINK_TOKEN_CALLBACK: Gas = Gas(80_000_000_000_000);

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

#[ext_contract(ext_token_resolve)]
pub trait ExtTokenCResolver {
    fn arc_mint_actor_link_callback(
        &mut self,
        actor_id: TokenId,
        token_id: TokenId,
        token_account: AccountId,
        sender_account: AccountId,
        actor_data: ActorData,
        token_data: TokenData,
        token_payout: TokenPayout,
        guild_id: Option<GuildId>,
    ) -> bool;

    fn arc_actor_link_callback(
        &mut self,
        actor_id: TokenId,
        token_id: TokenId,
        token_account: AccountId,
        sender_account: AccountId,
    ) -> bool;

    fn arc_actor_ulink_callback(
        actor_id: TokenId,
        token_id: TokenId,
        sender_account: AccountId,
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

impl ArcActors {
    fn token_from_promise_result(&mut self) -> Option<JsonToken> {
        require!(
            env::promise_results_count() == 1,
            "the method can only be called as a callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic_str("failed to get token info"),
            PromiseResult::Successful(result) => {
                near_sdk::serde_json::from_slice::<Option<JsonToken>>(&result).unwrap()
            }
        }
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
        // contract_id: AccountId,
        // linked_id: TokenId,
        // token_id: TokenId,
        actor_id: TokenId,
        token_id: TokenId,
        token_account: AccountId,
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
        require!(
            env::prepaid_gas() > GAS_LINK_TOKEN + GAS_LINK_TOKEN_CALLBACK,
            "not enough prepaid gas to be attached to the transaction"
        );
        require!(
            self.tokens.info_by_id.get(&actor_id).is_none(),
            "an actor with the provided id already exits"
        );
        require!(
            self.actors.link_for_token.get(&token_id).is_none(),
            "the token is already linked to an other actor"
        );

        let sender_account = env::predecessor_account_id();
        ext_token_call::nft_token(token_id.clone(), token_account.clone(), 0, GAS_LINK_TOKEN).then(
            ext_token_resolve::arc_mint_actor_link_callback(
                actor_id,
                token_id,
                token_account,
                sender_account,
                actor_data,
                token_data,
                token_payout,
                guild_id,
                env::current_account_id(),
                0,
                GAS_LINK_TOKEN_CALLBACK,
            ),
        )
    }

    pub fn arc_mint_actor_link_callback(
        &mut self,
        actor_id: TokenId,
        token_id: TokenId,
        token_account: AccountId,
        sender_account: AccountId,
        actor_data: ActorData,
        token_data: TokenData,
        token_payout: TokenPayout,
        guild_id: Option<GuildId>,
    ) {
        let info = self.token_from_promise_result();
        require!(info.is_some(), "invalid token info");

        if let Some(token) = info {
            require!(token.owner_id == sender_account, "invalid owner link");
            require!(token.token_id == token_id, "invalid token link");
        }

        let owner = OwnerIds {
            account: token_account,
            guild_id: guild_id,
            token_id: Some(token_id),
        };
        self.tokens.register(
            &owner,
            &actor_id,
            TokenType::Actor,
            token_data,
            token_payout,
            None,
        );
        self.actors.register(&owner, &actor_id, actor_data, None);
    }
}

#[near_bindgen]
impl ArcActors {
    #[payable]
    pub fn arc_actor_link(
        &mut self,
        actor_id: TokenId,
        token_id: TokenId,
        token_account: AccountId,
    ) -> Promise {
        require!(
            env::prepaid_gas() > GAS_LINK_TOKEN + GAS_LINK_TOKEN_CALLBACK,
            "not enough prepaid gas to be attached to the transaction"
        );
        require!(
            self.actors.link_for_token.get(&token_id).is_none(),
            "the token is already linked to an other actor"
        );

        let actor = self
            .tokens
            .info_by_id
            .get(&actor_id)
            .expect("actor info not found");
        require!(
            actor.owner.token_id.is_none(),
            "only an unlinked actor can be linked"
        );

        let sender_account = env::predecessor_account_id();
        require!(
            sender_account == actor.owner.account,
            "only the owner of an actor can link it"
        );

        ext_token_call::nft_token(token_id.clone(), token_account.clone(), 0, GAS_LINK_TOKEN).then(
            ext_token_resolve::arc_actor_link_callback(
                actor_id,
                token_id,
                token_account,
                sender_account,
                env::current_account_id(),
                0,
                GAS_LINK_TOKEN_CALLBACK,
            ),
        )
    }

    pub fn arc_actor_link_callback(
        &mut self,
        actor_id: TokenId,
        token_id: TokenId,
        token_account: AccountId,
        sender_account: AccountId,
    ) {
        let info = self.token_from_promise_result();
        require!(info.is_some(), "invalid token info");

        if let Some(token) = info {
            require!(token.owner_id == sender_account, "invalid link owner");
            require!(token.token_id == token_id, "invalid token id");
        }

        self.set_link(&actor_id, &token_account, Some(token_id));
    }

    #[payable]
    pub fn arc_actor_ulink(&mut self, actor_id: TokenId) -> Promise {
        let actor = self
            .tokens
            .info_by_id
            .get(&actor_id)
            .expect("actor info not found");

        let token_id = actor
            .owner
            .token_id
            .expect("only an linked actor can be unlinked");

        let sender_account = env::predecessor_account_id();

        ext_token_call::nft_token(
            token_id.clone(),
            actor.owner.account.clone(),
            0,
            GAS_LINK_TOKEN,
        )
        .then(ext_token_resolve::arc_actor_ulink_callback(
            actor_id,
            token_id,
            sender_account,
            env::current_account_id(),
            0,
            GAS_LINK_TOKEN_CALLBACK,
        ))
    }

    pub fn arc_actor_ulink_callback(
        &mut self,
        actor_id: TokenId,
        token_id: TokenId,
        sender_account: AccountId,
    ) {
        let info = self.token_from_promise_result();
        require!(info.is_some(), "invalid token info");

        if let Some(token) = info {
            require!(token.owner_id == sender_account, "invalid owner link");
            require!(token.token_id == token_id, "invalid token link");
        }

        self.set_link(&actor_id, &sender_account, None);
    }
}
