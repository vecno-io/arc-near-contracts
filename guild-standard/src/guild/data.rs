use crate::*;

const MAX_BASIS_POINTS: u16 = 10000;

// ==== Guild ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guild {
    pub ceo_id: AccountId,
    pub ceo_share: u16,
    pub board_size: u16,
    pub board_share: u16,
    pub members_size: u64,
    pub members_share: u16,
}

impl Guild {
    pub fn assert_valid(&self) {
        require!(
            self.members_size > 0,
            "Members size must be atleast one or more"
        );
        require!(
            self.board_size as u64 <= self.members_size,
            "Board size can not be larger than members size"
        );
        require!(
            self.ceo_share <= MAX_BASIS_POINTS,
            "CEO share can not be more than 100_00 basis points"
        );
        require!(
            self.board_share <= MAX_BASIS_POINTS,
            "Board share can not be more than 100_00 basis points"
        );
        require!(
            self.members_share <= MAX_BASIS_POINTS,
            "Members share can not be more than 100_00 basis points"
        );
        require!(
            // Note: This check depends on the Rust compilers setting: `overflow-checks = true`
            (self.ceo_share + self.board_share + self.members_share) <= MAX_BASIS_POINTS,
            "Total shares can not be more than 100_00 basis points"
        );
    }
}

// ==== Guild ID ====

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GuildId(String);

impl GuildId {
    /// Returns reference to the guild ID bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    /// Returns reference to the guild ID string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for GuildId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for GuildId {
    fn from(item: String) -> Self {
        require!(
            is_valid_guild_id(item.as_bytes()),
            "The string is not a valid guild id"
        );
        GuildId { 0: item }
    }
}

impl fmt::Display for GuildId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuildIdParseError {}

impl std::str::FromStr for GuildId {
    type Err = GuildIdParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if is_valid_guild_id(value.as_bytes()) {
            Ok(Self(value.to_string()))
        } else {
            Err(GuildIdParseError {})
        }
    }
}

impl fmt::Display for GuildIdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the guild id is invalid")
    }
}

impl std::error::Error for GuildIdParseError {}

#[inline(always)]
fn is_valid_guild_id(id: &[u8]) -> bool {
    return id.len() > 0 && id.len() <= 32;
}

// ==== Guild Board ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GuildBoard {
    /// List of board members and their share.
    pub members_list: UnorderedMap<AccountId, u16>,
}

impl GuildBoard {
    pub fn assert_valid(&self, max_members: u64) {
        require!(
            self.members_list.len() <= max_members,
            format!("The board can not have more then {} members", max_members)
        );
        let mut total: u16 = 0;
        for amount in self.members_list.values() {
            total += amount;
        }
        require!(
            MAX_BASIS_POINTS >= total,
            "Total shares can not be more than 100_00 basis points"
        );
    }
}

// ==== Guild Members ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GuildMembers {
    /// Total membership value
    pub total_value: u128,
    /// List of guild members and their stake
    pub members_list: UnorderedMap<AccountId, u128>,
}

impl GuildMembers {
    pub fn assert_valid(&self, max_members: u64) {
        require!(
            self.members_list.len() > 0,
            "Guild members size must be atleast one or more"
        );
        require!(
            self.members_list.len() <= max_members,
            format!("The guild can not have more then {} members", max_members)
        );
        let mut total: u128 = 0;
        for amount in self.members_list.values() {
            total += amount;
        }
        require!(
            self.total_value == total,
            "Total value must be the sum of all member values"
        );
    }
}

// ==== Guild Member Set ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GuildMemberSet {
    /// Set of all the guilds for a member.
    pub guilds_set: UnorderedSet<GuildId>,
}
