use crate::*;

use near_sdk::json_types::U128;

pub type GuildId = String;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonGuild {
    //guild ID
    pub guild_id: GuildId,
    //token count in the guild
    pub token_cnt: U128,
    //owner of the token
    pub manager_id: AccountId,
    //token metadata
    pub metadata: GuildData,
}

#[derive(Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GuildData {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub icon_hash: Option<Base64VecU8>,
    pub media: Option<String>,
    pub media_hash: Option<Base64VecU8>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

impl GuildData {
    pub fn assert_valid(&self) {
        require!(self.icon.is_some() == self.icon_hash.is_some());
        if let Some(icon_hash) = &self.icon_hash {
            require!(icon_hash.0.len() == 32, "Icon hash has to be 32 bytes");
        }

        require!(self.media.is_some() == self.media_hash.is_some());
        if let Some(media_hash) = &self.media_hash {
            require!(media_hash.0.len() == 32, "Media hash has to be 32 bytes");
        }

        require!(self.reference.is_some() == self.reference_hash.is_some());
        if let Some(reference_hash) = &self.reference_hash {
            require!(
                reference_hash.0.len() == 32,
                "Reference hash has to be 32 bytes"
            );
        }
    }
}
