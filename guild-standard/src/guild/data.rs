use crate::*;

pub const MAX_BASIS_POINTS: u16 = 10000;

// ==== Guild ID ====

impl_string_id!("guild", GuildId, GuildIdParseError);

// ==== Guild Info ====

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GuildInfo {
    pub ceo_id: AccountId,
    pub ceo_share: u16,
    pub board_size: u64,
    pub board_share: u16,
    pub members_size: u64,
    pub members_share: u16,
}

impl GuildInfo {
    pub fn assert_valid(&self) {
        require!(
            self.members_size > 0,
            "Members size must be atleast one or more"
        );
        require!(
            self.board_size <= self.members_size,
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

// ==== Guild State ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    pub exec: GuildId,
    pub lock: LockedFor,
    pub time: Option<u64>,
    pub vote: Option<MotionId>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GuildState {
    pub info: GuildInfo,
    pub lock: LockedFor,
    pub vote: Option<MotionId>,
}

// ==== Guild Board ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BoardMembers {
    /// List of board members and their share.
    pub list: UnorderedMap<AccountId, u16>,
}

impl BoardMembers {
    pub fn assert_valid(&self, max_members: u64) {
        require!(
            self.list.len() <= max_members,
            format!("The board can not have more then {} members", max_members)
        );
        let mut total: u16 = 0;
        for amount in self.list.values() {
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
    /// Total of membership values for the guild.
    pub value: u128,
    /// List of guild members and their stake
    pub list: UnorderedMap<AccountId, u128>,
}

impl GuildMembers {
    pub fn assert_valid(&self, max_members: u64) {
        require!(
            self.list.len() > 0,
            "Guild members size must be atleast one or more"
        );
        require!(
            self.list.len() <= max_members,
            format!("The guild can not have more then {} members", max_members)
        );
        let mut total: u128 = 0;
        for amount in self.list.values() {
            total += amount;
        }
        require!(
            self.value == total,
            "Total value must be the sum of all member values"
        );
    }
}

// ==== Guild Member Set ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MemberSet {
    /// Total membership values for a member.
    pub value: u128,
    /// Set of all the guild IDs for a member.
    pub store: UnorderedSet<GuildId>,
}
