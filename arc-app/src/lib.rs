arc_standard::use_imports!();

use arc_standard::{Actors, Admin, Guilds, Metadata, Tokens};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct App {
    admin: Admin,

    actor: Actors,
    guild: Guilds,
    token: Tokens,

    metadata: LazyOption<Metadata>,
}

arc_standard::impl_meta!(App, metadata);

arc_standard::impl_arc_actors!(App, token, actor);
arc_standard::impl_arc_guilds!(App, token, guild);
arc_standard::impl_nft_tokens!(App, token, actor, guild);

#[derive(BorshSerialize)]
pub enum StorageKey {
    AppAdmin,
    AppMetadata,
}

#[near_bindgen]
impl App {
    #[init]
    pub fn new(
        ceo_id: AccountId,
        guild_id: GuildId,
        guild_data: GuildData,
        guild_board: GuildBoard,
        guild_payout: Option<AccountId>,
        app_metadata: Metadata,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");

        guild_data.assert_valid();
        guild_board.assert_valid();
        app_metadata.assert_valid();

        let mut this = Self {
            admin: Admin::new(StorageKey::AppAdmin.try_to_vec().unwrap(), Some(&guild_id)),
            actor: Actors::new(),
            guild: Guilds::new(),
            token: Tokens::new(),

            metadata: LazyOption::new(
                StorageKey::AppMetadata.try_to_vec().unwrap(),
                Some(&app_metadata),
            ),
        };
        this.guild.register(
            &ceo_id,
            &guild_id,
            GuildType::Core,
            guild_data,
            guild_board,
            guild_payout,
            None,
        );
        this
    }

    #[init]
    pub fn new_default(ceo_id: AccountId, board: AccountId) -> Self {
        require!(!env::state_exists(), "Already initialized");

        let mut members = HashMap::new();
        members.insert(board, 10000);

        Self::new(
            ceo_id,
            GuildId::from("admin:guild".to_string()),
            GuildData {
                spec: NFT_METADATA_SPEC.to_string(),
                tag: "Arc-Core".to_string(),
                name: "The Core Guild".to_string(),
                icon: None,
                icon_hash: None,
                media: None,
                media_hash: None,
                reference: None,
                reference_hash: None,
            },
            GuildBoard {
                size: 1,
                share: 5000,
                members,
            },
            None,
            Metadata {
                spec: ARC_STANDARD_SPEC.to_string(),
                name: "The Core App".to_string(),
                symbol: "ARC-C".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }
}

#[near_bindgen]
impl ArcApp for App {
    fn arc_create_guild(
        &mut self,
        ceo_id: AccountId,
        guild_id: GuildId,
        guild_type: GuildType,
        guild_data: GuildData,
        guild_board: GuildBoard,
        guild_payout: Option<AccountId>,
    ) {
        assert_min_one_yocto();
        let storage_usage = env::storage_usage();

        // TODO Implement checks who can call this?
        // Make the admin guild vote on creation?
        // For demo/Testing it is open to all.

        self.guild.register(
            &ceo_id,
            &guild_id,
            guild_type,
            guild_data,
            guild_board,
            guild_payout,
            None,
        );

        refund_storage_deposit(env::storage_usage() - storage_usage);
    }

    fn arc_mint_actor(
        &mut self,
        owner_id: AccountId,
        token_id: TokenId,
        actor_data: ActorData,
        token_data: TokenData,
        token_payout: TokenPayout,
        guild_id: Option<GuildId>,
    ) {
        assert_min_one_yocto();
        let storage_usage = env::storage_usage();

        self.token.register(
            &owner_id,
            &token_id,
            TokenType::Actor,
            token_data,
            token_payout,
            guild_id,
            None,
        );
        self.actor.register(&owner_id, &token_id, actor_data, None);

        refund_storage_deposit(env::storage_usage() - storage_usage);
    }

    fn arc_add_guild_member(&mut self, guild_key: GuildId, member_id: AccountId) {
        assert_min_one_yocto();
        let storage_usage = env::storage_usage();

        // TODO: FixMe: Temp rush job to meet encode club dealines
        // Note: Base setup, no verification or reuse of old tokens implemented
        self.guild.create_member_token(&guild_key, Some(member_id));

        refund_storage_deposit(env::storage_usage() - storage_usage);
    }
}
