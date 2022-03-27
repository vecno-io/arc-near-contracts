use crate::*;

pub mod api;
pub mod data;

pub use self::api::*;
pub use self::data::*;

#[derive(BorshSerialize)]
pub enum StorageKey {
    GuildInfoMap,
    GuildAccountMap,
    GuildAccountSet { id: AccountId },
    GuildBoardMap,
    GuildBoardList { id: GuildId },
    GuildMembersMap,
    GuildMembersList { id: GuildId },
}

#[cfg(test)]
mod tests {
    mod api;
    mod data;
}
