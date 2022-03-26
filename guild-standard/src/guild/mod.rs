use crate::*;

pub mod api;
pub mod data;

pub use self::api::*;
pub use self::data::*;

#[derive(BorshSerialize)]
pub enum StorageKey {
    GuildGuildMap,
    GuildBoardMap,
    GuildBoardList { id: GuildId },
    GuildMemberMap,
    GuildMemberSet { id: AccountId },
    GuildMembersMap,
    GuildMembersList { id: GuildId },
}

#[cfg(test)]
mod tests {
    mod api;
    mod data;
}
