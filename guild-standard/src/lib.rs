use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, require};

pub const GUILD_STANDARD_SPEC: &str = "GUILD-0.1.0";

pub mod utils;
pub mod guild;

pub use crate::utils::*;
pub use crate::guild::*;
