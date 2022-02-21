use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Name {
    pub last: String,
    pub first: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Persona {
    pub root: u8,
    pub style: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Social {
    pub age: u8,
    pub style: u8,
    pub economy: u8,
    pub community: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Habitat {
    pub age: u8,
    pub style: u8,
    pub scale: u8,
    pub nature: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Ancestry {
    pub social: Social,
    pub habitat: Habitat,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenActordata {
    pub name: Name,
    pub persona: Persona,
    pub ancestry: Ancestry,
    pub attributes: Attributes,
}