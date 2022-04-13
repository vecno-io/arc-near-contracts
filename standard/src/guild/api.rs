use super::*;

// ==== Lock Contract ====

pub trait ContractLocking {
    fn act_lock_contract(&mut self, details: Option<String>);
    fn vote_lock_contract(&mut self);
    fn act_unlock_contract(&mut self, details: String);
    fn vote_unlock_contract(&mut self);
}

pub trait ContractChallenging {
    fn act_challenge_contract_exec(&mut self, details: String);
    fn vote_challenge_contract_exec(&mut self);
    fn act_replace_contract_exec(&mut self, details: String);
    fn vote_replace_contract_exec(&mut self);
}

pub trait ContractBoard {
    fn act_lock_contract_ceo(&mut self, details: String);
    fn vote_lock_contract_ceo(&mut self);
}

// ==== Lock Guild ====

pub trait LockGuild {
    fn act_lock_guild(&mut self, guild_id: &GuildId, details: String);
    fn vote_lock_guild(&mut self, guild_id: &GuildId, details: Option<String>);
    fn vote_unlock_guild(&mut self, guild_id: &GuildId);
}

// ==== Standard Implementation ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guilds {
    pub state: LazyOption<State>,
    pub guild_map: UnorderedMap<GuildId, GuildState>,
    pub board_map: LookupMap<GuildId, BoardMembers>,
    pub member_map: LookupMap<GuildId, GuildMembers>,
    pub account_map: UnorderedMap<AccountId, MemberSet>,
}

impl Guilds {
    pub fn new(exec: &GuildId) -> Self {
        Self {
            state: LazyOption::new(
                StorageKey::GuildsState.try_to_vec().unwrap(),
                Some(&State {
                    exec: exec.clone(),
                    lock: LockedFor::None,
                    vote: None,
                    time: None,
                }),
            ),
            guild_map: UnorderedMap::new(StorageKey::GuildInfoMap.try_to_vec().unwrap()),
            board_map: LookupMap::new(StorageKey::GuildBoardMap.try_to_vec().unwrap()),
            member_map: LookupMap::new(StorageKey::GuildMembersMap.try_to_vec().unwrap()),
            account_map: UnorderedMap::new(StorageKey::GuildAccountMap.try_to_vec().unwrap()),
        }
    }

    pub fn register(
        &mut self,
        id: &GuildId,
        guild: &GuildInfo,
        board_map: &HashMap<AccountId, u16>,
        member_map: &HashMap<AccountId, u128>,
    ) {
        guild.assert_valid();

        require!(
            member_map.len() > 0,
            "Need at least one guild members or more"
        );
        require!(
            board_map.len() <= guild.board_size as usize,
            "The board list can not be larger than the boards size"
        );
        require!(
            member_map.len() <= guild.members_size as usize,
            "The members list can not be larger than the guilds size"
        );
        require!(
            board_map.get(&guild.ceo_id).is_none(),
            format!("The CEO can not be a board member")
        );
        require!(
            !member_map.get(&guild.ceo_id).is_none(),
            format!("The CEO must be a guild member")
        );

        let state = GuildState {
            info: guild.clone(),
            lock: LockedFor::None,
            vote: None,
        };
        require!(
            self.guild_map.insert(&id, &state).is_none(),
            "The provided guild id is already in use"
        );

        let mut guild_members = GuildMembers {
            value: 0,
            list: UnorderedMap::new(
                StorageKey::GuildMembersList { id: id.clone() }
                    .try_to_vec()
                    .unwrap(),
            ),
        };
        for (account, value) in member_map.iter() {
            guild_members.value += value;
            require!(
                // Note: In theory this is imposible, still check
                guild_members.list.insert(account, value).is_none(),
                format!("Duplicated member entry found for {}", account)
            );
            let mut guild_set = self.account_map.get(&account).unwrap_or_else(|| MemberSet {
                value: 0,
                store: UnorderedSet::new(
                    StorageKey::GuildAccountSet {
                        id: account.clone(),
                    }
                    .try_to_vec()
                    .unwrap(),
                ),
            });
            guild_set.value += value;
            guild_set.store.insert(id);
            self.account_map.insert(account, &guild_set);
        }
        self.member_map.insert(id, &guild_members);

        let mut board_members = BoardMembers {
            list: UnorderedMap::new(
                StorageKey::GuildBoardList { id: id.clone() }
                    .try_to_vec()
                    .unwrap(),
            ),
        };
        let mut total: u16 = 0;
        for (account, share) in board_map.iter() {
            total += share;
            require!(
                // Note: In theory this is imposible, still check
                board_members.list.insert(account, share).is_none(),
                format!("Duplicated board entry found: {}", account)
            );
            require!(
                !guild_members.list.get(account).is_none(),
                format!("Board member need to be a guild member: {}", account)
            );
        }
        self.board_map.insert(id, &board_members);
        require!(
            MAX_BASIS_POINTS >= total,
            "Total board shares can not be more than 100_00 basis points"
        );
    }
}
