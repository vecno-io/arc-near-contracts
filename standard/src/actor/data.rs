use super::*;

use crate::token::TokenData;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonActor {
    //token ID
    pub token_id: TokenId,
    //owner info for the token
    pub ownerdata: OwnerIds,
    //token metadata
    pub tokendata: TokenData,
    //token actordata
    pub actordata: ActorData,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ActorData {
    pub key: String,
    pub handle: Handle,
    pub persona: Persona,
    pub ancestry: Ancestry,
    pub attributes: Attributes,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Handle {
    pub last: String,
    pub first: String,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Persona {
    pub root: u8,
    pub style: u8,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Social {
    pub age: u8,
    pub style: u8,
    pub economy: u8,
    pub community: u8,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Habitat {
    pub age: u8,
    pub style: u8,
    pub scale: u8,
    pub nature: u8,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ancestry {
    pub social: Social,
    pub habitat: Habitat,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Attributes {
    pub dexterity: u8,
    pub empathy: u8,
    pub intellect: u8,

    pub perception: u8,
    pub presence: u8,
    pub stamina: u8,

    pub strength: u8,
    pub vitality: u8,
    pub wisdom: u8,
}

impl ActorData {
    pub fn require_valid(&self) {
        self.handle.require_valid();
    }
}

impl Handle {
    pub fn require_valid(&self) {
        // Note: Look in to regex in near
        // https://github.com/rust-lang/regex
        require!(
            &self.last.len() < &16,
            "Last name can not be longer than 16 characters"
        );
        require!(
            &self.first.len() > &2 && &self.first.len() < &16,
            "First name needs to be between 2 and 16 characters"
        );
    }
}
