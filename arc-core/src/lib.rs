use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

//use arc_standard::{Assets, Journals, StateStore};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {}

// assets: Assets,
// journals: Journals,
// stateStore: StateStore,
