use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

use arc_standard::{Actors, Admin, Guilds, Tokens};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    admin: Admin,

    actors: Actors,
    guilds: Guilds,
    tokens: Tokens,
}
