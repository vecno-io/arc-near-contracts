use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env, require, AccountId};

pub mod api;
pub mod data;

pub use self::api::*;
pub use self::data::*;

#[derive(BorshSerialize)]
pub enum StorageKey {
    GuildInfoById,
    GuildDataForId,
    GuildBoardForId,
    GuildMemberById,
    GuildListPerGuild,
    GuildListPerGuildSet { guild_id: GuildId },
    GuildListPerOwner,
    GuildListPerOwnerSet { owner_key: AccountKey },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guilds {
    //keeps track of the guild info for a given guild key
    pub info_by_id: LookupMap<GuildId, Guild>,
    //keeps track of the member info for a given token key
    pub member_by_id: LookupMap<TokenId, GuildMember>,
    //keeps track of the guilds data for a given guild key
    pub data_for_id: UnorderedMap<GuildId, GuildData>,
    //keeps track of the guilds board for a given guild key
    pub board_for_id: UnorderedMap<GuildId, GuildBoard>,
    //keeps track of all the member token keys per guild key
    pub list_per_guild: LookupMap<GuildId, UnorderedSet<TokenId>>,
    //keeps track of the guild memberships for a given account key
    pub list_per_owner: LookupMap<AccountKey, UnorderedMap<GuildId, TokenId>>,
}

impl Guilds {
    pub fn new() -> Self {
        let this = Self {
            info_by_id: LookupMap::new(StorageKey::GuildInfoById.try_to_vec().unwrap()),
            member_by_id: LookupMap::new(StorageKey::GuildMemberById.try_to_vec().unwrap()),
            data_for_id: UnorderedMap::new(StorageKey::GuildDataForId.try_to_vec().unwrap()),
            board_for_id: UnorderedMap::new(StorageKey::GuildBoardForId.try_to_vec().unwrap()),
            list_per_guild: LookupMap::new(StorageKey::GuildListPerGuild.try_to_vec().unwrap()),
            list_per_owner: LookupMap::new(StorageKey::GuildListPerOwner.try_to_vec().unwrap()),
        };
        this
    }

    pub fn register(
        &mut self,
        ceo_id: &AccountId,
        guild_id: &GuildId,
        guild_type: GuildType,
        guild_data: GuildData,
        guild_board: GuildBoard,
        memo: Option<String>,
    ) {
        guild_data.assert_valid();
        guild_board.assert_valid();

        let guild = Guild {
            ceo_id: ceo_id.clone(),
            type_id: guild_type.clone(),
        };
        require!(
            self.info_by_id.insert(&guild_id, &guild).is_none(),
            "A guild with the provided id already exits"
        );
        self.data_for_id.insert(&guild_id, &guild_data);
        self.board_for_id.insert(&guild_id, &guild_board);

        self.list_per_guild.insert(
            guild_id,
            &UnorderedSet::new(
                StorageKey::GuildListPerGuildSet {
                    guild_id: guild_id.clone(),
                }
                .try_to_vec()
                .unwrap(),
            ),
        );

        self.create_member_token(guild_id, Some(ceo_id.clone()));

        for member in guild_board.members.keys() {
            require!(
                ceo_id != member,
                "The CEO can not be an active board member"
            );
            self.create_member_token(guild_id, Some(member.clone()));
        }

        let arc_register_log: ArcEventLog = ArcEventLog {
            module: EVENT_ARC_STANDARD_GUILD.to_string(),
            version: EVENT_ARC_METADATA_SPEC.to_string(),
            event: ArcEventVariant::ArcRegister(vec![ArcRegisterLog {
                user_id: ceo_id.to_string(),
                keys_list: vec![guild_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&arc_register_log.to_string());
    }

    pub fn set_member_token(&mut self, token_id: &TokenId, owner_id: Option<AccountId>) {
        let info = self.member_by_id.get(token_id).expect("token id not found");

        if let Some(owner) = info.owner_id {
            if let Some(mut map) = self.list_per_owner.get(&owner.clone().into()) {
                map.remove(&info.guild_id);
                self.list_per_owner.insert(&owner.into(), &map);
            }
        }

        if let Some(owner) = owner_id.clone() {
            let mut map = self
                .list_per_owner
                .get(&owner.clone().into())
                .unwrap_or_else(|| {
                    UnorderedMap::new(
                        StorageKey::GuildListPerOwnerSet {
                            owner_key: owner.clone().into(),
                        }
                        .try_to_vec()
                        .unwrap(),
                    )
                });

            require!(
                map.insert(&info.guild_id, &token_id).is_none(),
                "The owner already is a member of the guild"
            );
            self.list_per_owner.insert(&owner.into(), &map);
        }

        self.member_by_id.insert(
            token_id,
            &GuildMember {
                guild_id: info.guild_id,
                owner_id,
            },
        );

        // TODO EVENT NEW GUILD MEMBER
    }

    pub fn create_member_token(
        &mut self,
        guild_id: &GuildId,
        owner_id: Option<AccountId>,
    ) -> Option<TokenId> {
        let mut guild_set = self
            .list_per_guild
            .get(guild_id)
            .expect("guild id not found");

        let token_id: TokenId = format!("{} #{}", guild_id.to_string(), guild_set.len()).into();

        require!(
            guild_set.insert(&token_id),
            "The members token id is already used"
        );
        self.member_by_id.insert(
            &token_id,
            &GuildMember {
                guild_id: guild_id.clone(),
                owner_id: owner_id.clone(),
            },
        );

        if owner_id.is_none() {
            return Some(token_id);
        }

        let owner_key = AccountKey::from(owner_id.unwrap().clone());
        let mut owner_map = self.list_per_owner.get(&owner_key).unwrap_or_else(|| {
            UnorderedMap::new(
                StorageKey::GuildListPerOwnerSet {
                    owner_key: owner_key.clone(),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        require!(
            owner_map.insert(guild_id, &token_id).is_none(),
            "The owner already is a member of the guild"
        );
        self.list_per_owner.insert(&owner_key, &owner_map);

        // TODO EVENT NEW GUILD MEMBER
        return Some(token_id);
    }
}
