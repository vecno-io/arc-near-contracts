use crate::*;

const MAX_BASE_POINTS_TOTAL: u16 = 10000;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize,
)]
#[serde(crate = "near_sdk::serde")]
pub enum GuildType {
    Core = 0,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GuildId(String);

impl ToString for GuildId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for GuildId {
    fn from(item: String) -> Self {
        GuildId { 0: item }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guild {
    //ceo id for the guild
    pub ceo_id: AccountId,
    //type id for the guild
    pub type_id: GuildType,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GuildMember {
    //id of the memberships guild
    pub guild_id: GuildId,
    //id of the memberships owner
    pub owner_id: Option<AccountId>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonGuild {
    //guild id
    pub id: GuildId,
    //ceo of the guild
    pub ceo: AccountId,
    //data for the guild
    pub data: GuildData,
    //board of the members
    pub board: GuildBoard,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonMember {
    pub guild_id: GuildId,
    pub owner_id: Option<AccountId>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GuildData {
    pub spec: String,
    pub tag: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<Base64VecU8>,
    pub media: Option<String>,
    pub media_hash: Option<Base64VecU8>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GuildBoard {
    pub size: u16,
    pub share: u16,
    pub members: HashMap<AccountId, u16>,
}

impl GuildData {
    pub fn assert_valid(&self) {
        require!(self.icon.is_some() == self.icon_hash.is_some());
        if let Some(icon_hash) = &self.icon_hash {
            require!(icon_hash.0.len() == 32, "Icon hash must be 32 bytes");
        }
        require!(self.media.is_some() == self.media_hash.is_some());
        if let Some(media_hash) = &self.media_hash {
            require!(media_hash.0.len() == 32, "Media hash must be 32 bytes");
        }
        require!(self.reference.is_some() == self.reference_hash.is_some());
        if let Some(reference_hash) = &self.reference_hash {
            require!(
                reference_hash.0.len() == 32,
                "Reference hash must be 32 bytes"
            );
        }
    }
}

impl GuildBoard {
    pub fn assert_valid(&self) {
        require!(
            (self.size as usize) >= self.members.len(),
            format!(
                "Can not have more members ({}) then board size ({})",
                self.members.len(),
                self.size
            )
        );
        require!(
            MAX_BASE_POINTS_TOTAL >= self.share,
            format!(
                "The boards share ({}) can not be more then max base points ({})",
                self.share, MAX_BASE_POINTS_TOTAL
            )
        );
        let mut total: u16 = 0;
        for amount in self.members.values() {
            total += amount;
        }
        require!(
            MAX_BASE_POINTS_TOTAL >= total,
            format!(
                "Total member shares ({}) can not be more then max base points ({})",
                total, MAX_BASE_POINTS_TOTAL
            )
        );
    }
}
