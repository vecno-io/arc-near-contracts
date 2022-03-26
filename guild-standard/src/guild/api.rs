use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guilds {
    pub guild_map: UnorderedMap<GuildId, Guild>,
    pub member_map: UnorderedMap<AccountId, GuildMemberSet>,
    pub board_map: LookupMap<GuildId, GuildBoard>,
    pub members_map: LookupMap<GuildId, GuildMembers>,
}

impl Guilds {
    pub fn new() -> Self {
        Self {
            guild_map: UnorderedMap::new(StorageKey::GuildGuildMap.try_to_vec().unwrap()),
            member_map: UnorderedMap::new(StorageKey::GuildMemberMap.try_to_vec().unwrap()),
            board_map: LookupMap::new(StorageKey::GuildBoardMap.try_to_vec().unwrap()),
            members_map: LookupMap::new(StorageKey::GuildMembersMap.try_to_vec().unwrap()),
        }
    }
}
