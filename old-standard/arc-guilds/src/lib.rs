use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize},
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet},
use near_sdk::serde::{Deserialize, Serialize},
use near_sdk::{env, require, AccountId};
use std::{collections::HashMap, fmt};

pub const GUILD_STANDARD_SPEC: &str = "GUILD-0.1.0";

pub mod base;

pub mod guild;
pub mod utils;
pub mod votes;

pub use crate::base::*;

pub use crate::guild::*;
pub use crate::utils::*;
pub use crate::votes::*;


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
