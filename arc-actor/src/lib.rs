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

        self.tokens.register(
            &owner_id,
            &token_id,
            TokenType::Actor,
            token_data,
            token_payout,
            guild_id,
            None,
        );
        self.actors.register(&owner_id, &token_id, actor_data, None);

        refund_storage_deposit(env::storage_usage() - storage_usage);
    }
}

// #[near_bindgen]
// impl ArcApp for ArcActors {
//     #[payable]
//     fn arc_create_guild(
//         &mut self,
//         ceo_id: AccountId,
//         guild_id: GuildId,
//         guild_type: GuildType,
//         guild_data: GuildData,
//         guild_board: GuildBoard,
//         guild_payout: Option<AccountId>,
//     ) {
//         require_min_one_yocto();
//         let storage_usage = env::storage_usage();

//         // TODO Implement checks who can call this?
//         // Make the admin guild vote on creation?
//         // For demo/Testing it is open to all.

//         self.guild.register(
//             &ceo_id,
//             &guild_id,
//             guild_type,
//             guild_data,
//             guild_board,
//             guild_payout,
//             None,
//         );

//         refund_storage_deposit(env::storage_usage() - storage_usage);
//     }

//     #[payable]
//     fn arc_add_guild_member(&mut self, guild_key: GuildId, member_id: AccountId) {
//         require_min_one_yocto();
//         let storage_usage = env::storage_usage();

//         // TODO: FixMe: Temp rush job to meet encode club dealines
//         // Note: Base setup, no verification or reuse of old tokens implemented
//         self.guild.create_member_token(&guild_key, Some(member_id));

//         refund_storage_deposit(env::storage_usage() - storage_usage);
//     }
// }
