use crate::*;

pub mod utility;

pub const MAX_BASE_POINTS_TOTAL: u16 = 10000;

// ==== Storage Keys ====

#[derive(BorshSerialize)]
pub enum StorageKey {
    ActorDataForId,
    ActorListPerOwner,
    ActorListPerOwnerSet { owner_key: AccountId },
    GuildsState,
    GuildInfoMap,
    GuildAccountMap,
    GuildAccountSet { id: AccountId },
    GuildBoardMap,
    GuildBoardList { id: GuildId },
    GuildMembersMap,
    GuildMembersList { id: GuildId },
    TokenInfoById,
    TokenDataForId,
    TokenListPerOwner,
    TokenListPerOwnerSet { owner_key: AccountId },
    VotesMotionMap,
    VotesResultMap,
    VotesVoicesMap,
    VotesVoicesMapList { id: MotionId },
}

// ==== Lock State ====

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum LockedFor {
    None,
    Locking,
    Emergency,
    Challenge,
    Challenging,
}

// ==== String IDs ====

impl_string_id!("guild", GuildId, GuildIdParseError);
impl_string_id!("motion", MotionId, MotionIdParseError);
impl_string_id!("token", TokenId, TokenIdParseError);
impl_string_id!("vote", VoteId, VoteIdParseError);

// ==== Required States ====

#[inline(always)]
pub fn require_one_yocto() {
    require!(
        env::attached_deposit() == 1,
        "requires attached deposit of exactly 1 yocto",
    )
}

#[inline(always)]
pub fn require_min_one_yocto() {
    require!(
        env::attached_deposit() >= 1,
        "requires attached deposit of at least 1 yocto",
    )
}

// ==== Shared Utilities ====

/// Converts the input into an amount to pay out.
///
/// Note: It does not validate, the caller needs to ensure values are valid.
#[inline(always)]
pub fn royalty_to_payout(royalty_percentage: u16, amount_to_pay: Balance) -> U128 {
    U128(royalty_percentage as u128 * amount_to_pay / MAX_BASE_POINTS_TOTAL as u128)
}

#[cfg(test)]
mod tests {
    mod strings;
}
