use crate::*;

const MAX_BASE_POINTS_TOTAL: u16 = 10000;

#[derive(Copy, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum GuildType {
    Base,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GuildKey(String);

impl ToString for GuildKey {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for GuildKey {
    fn from(item: String) -> Self {
        GuildKey { 0: item }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guild {
    //ceo id for the guild
    pub ceo_id: AccountId,
    //type id for the guild
    pub type_id: GuildType,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonGuild {
    //guild id
    pub id: GuildKey,
    //ceo of the guild
    pub ceo: AccountId,
    //data for the guild
    pub data: GuildData,
    //board of the members
    pub board: GuildBoard,
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

pub trait ArcGuild {
    fn arc_guild(&self, guild_id: GuildKey) -> Option<JsonGuild>;

    fn arc_register_guild(
        &mut self,
        ceo_id: AccountId,
        guild_key: GuildKey,
        guild_data: GuildData,
        guild_board: GuildBoard,
        memo: Option<String>,
    );
}

pub trait ArcGuildEnumerator {
    fn arc_guild_count(&self) -> U128;

    fn arc_guilds(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonGuild>;
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

#[macro_export]
macro_rules! impl_arc_guilds {
    //where $data is LazyOption<ContractData>
    ($contract: ident, $guilds: ident) => {
        use $crate::*;

        #[near_bindgen]
        impl ArcGuild for $contract {
            fn arc_guild(&self, guild_key: GuildKey) -> Option<JsonGuild> {
                if let Some(guild) = self.$guilds.info_by_id.get(&guild_key) {
                    let data = self.$guilds.data_for_id.get(&guild_key).unwrap();
                    let board = self.$guilds.board_for_id.get(&guild_key).unwrap();
                    Some(JsonGuild {
                        id: guild_key,
                        ceo: guild.ceo_id,
                        data: data,
                        board: board,
                    })
                } else {
                    None
                }
            }

            // TODO FixMe: This methode is not a contract call
            // It is internal to the contract so !#[near_bindgen]
            fn arc_register_guild(
                &mut self,
                ceo_id: AccountId,
                guild_key: GuildKey,
                guild_data: GuildData,
                guild_board: GuildBoard,
                memo: Option<String>,
            ) {
                assert_min_one_yocto();
                guild_data.assert_valid();
                guild_board.assert_valid();

                let storage_usage = env::storage_usage();

                // TODO: move to contract implementaion
                //create the guild and store it
                // let guild_id = format!(
                //     "{}:Guild:{:06}",
                //     asset_data.symbol,
                //     self.guilddata_by_id.len()
                // );

                let guild = Guild {
                    ceo_id: ceo_id,
                    type_id: GuildType::Base,
                };
                require!(
                    self.$guilds.info_by_id.insert(&guild_key, &guild).is_none(),
                    "A guild with the provided id already exits"
                );
                self.$guilds.data_for_id.insert(&guild_key, &guild_data);
                self.$guilds.board_for_id.insert(&guild_key, &guild_board);

                // TODO: Implement events for guild registration
                if let Some(message) = memo {
                    env::log_str(&message);
                }

                // // TEMP TEST Methode
                // // TODO Implement API Hooks
                // self.$guilds.create_vote();

                //refund unused storage fees and return the id to the caller,
                refund_storage_deposit(env::storage_usage() - storage_usage);
            }
        }

        impl ArcGuildEnumerator for $contract {
            fn arc_guild_count(&self) -> U128 {
                U128(self.$guilds.data_for_id.len() as u128)
            }

            fn arc_guilds(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonGuild> {
                let start = u128::from(from_index.unwrap_or(U128(0)));
                self.$guilds
                    .data_for_id
                    .keys()
                    .skip(start as usize)
                    .take(limit.unwrap_or(50) as usize)
                    .map(|guild_key| self.arc_guild(guild_key.clone()).unwrap())
                    .collect()
            }
        }
    };
}
