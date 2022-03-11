use crate::*;

use std::collections::{HashMap, HashSet};

#[derive(BorshDeserialize, BorshSerialize)]
enum GuildType {
    None,
}

pub type GuildId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guild {
    //manager id of the token
    pub manager_id: AccountId,
    //store royalties for this token
    pub royalty: HashMap<AccountId, u32>,
    //allow minting calls from all account
    pub approval_open: bool,
    //map approved account IDs to an approval ID
    pub approved_accounts: HashSet<AccountId>,
}

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

pub trait ArcGuild {
    fn arc_guild(&self, guild_id: GuildId) -> Option<JsonGuild>;

    fn arc_register_guild(
        &mut self,
        manager_id: AccountId,
        guild_data: GuildData,
        approval_open: bool,
        royalties: Option<HashMap<AccountId, u32>>,
        memo: Option<String>,
    ) -> GuildId;
}

pub trait ArcGuildEnumerator {
    fn arc_guild_count(&self) -> U128;

    fn arc_guilds(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonGuild>;
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

impl Contract {
    pub(crate) fn register_token_for_guild(&mut self, guild_id: &GuildId) -> TokenId {
        let guild_info = self
            .guilds_by_id
            .get(&guild_id)
            .expect("Guild id to be valid");

        let sender_id = env::predecessor_account_id();
        if !guild_info.approval_open && guild_info.manager_id != sender_id {
            if !guild_info.approved_accounts.contains(&sender_id) {
                env::panic_str("Unauthorized transfer");
            }
        }

        let guild_data = self
            .guilddata_by_id
            .get(&guild_id)
            .expect("Guild id to be valid");
        let asset_data = self
            .contract_data
            .get()
            .expect("Contract to be initialized");
        let mut guild_set = self
            .tokens_per_guild
            .get(&guild_id)
            .expect("A guild set to be stored");

        let asset_key = asset_data.symbol;
        let guild_key = guild_data.symbol;
        let guild_idx = guild_set.len();

        let token_id = format!("{:>4}:{:>4} #{:05}", asset_key, guild_key, guild_idx);
        require!(guild_set.insert(&token_id), "Guild is out of IDs");
        token_id
    }
}

impl ArcGuild for Contract {
    fn arc_guild(&self, guild_id: GuildId) -> Option<JsonGuild> {
        //if there is some data for the token id in the token data store:
        if let Some(guild) = self.guilds_by_id.get(&guild_id) {
            let guilddata = self.guilddata_by_id.get(&guild_id).unwrap();
            let guildset = self.tokens_per_guild.get(&guild_id).unwrap();
            //then return the wrapped JsonActor
            Some(JsonGuild {
                guild_id: guild_id,
                token_cnt: U128(guildset.len() as u128),
                manager_id: guild.manager_id,
                metadata: guilddata,
            })
        } else {
            //else return None
            None
        }
    }

    fn arc_register_guild(
        &mut self,
        manager_id: AccountId,
        guild_data: GuildData,
        approval_open: bool,
        royalties: Option<HashMap<AccountId, u32>>,
        memo: Option<String>,
    ) -> GuildId {
        assert_min_one_yocto();
        guild_data.assert_valid();

        let manager = self.manager_id.get().expect("Contract to be initialized");
        assert_eq!(manager, env::predecessor_account_id(), "Call unauthorized");

        let asset_data = self
            .contract_data
            .get()
            .expect("Contract to be initialized");

        let storage_usage = env::storage_usage();

        //verify royalties
        let mut total = 0;
        let mut royalty = HashMap::new();
        if let Some(royalties) = royalties {
            require!(
                royalties.len() < 5,
                "Cannot add more than 4 royalty amounts per guild"
            );
            for (account, amount) in royalties {
                royalty.insert(account, amount);
                total += amount;
            }
            require!(
                total <= MAX_TOTAL_ROYALTIES,
                "The total of all royalties can not be larger than 10000"
            );
        }

        //create the guild and store it
        let guild_id = format!(
            "{}:Guild:{:06}",
            asset_data.symbol,
            self.guilddata_by_id.len()
        );
        let guild = Guild {
            royalty: royalty,
            manager_id: manager_id,
            approval_open: approval_open,
            approved_accounts: Default::default(),
        };
        require!(
            self.guilds_by_id.insert(&guild_id, &guild).is_none(),
            "A guild with the generated id already exits"
        );
        self.guilddata_by_id.insert(&guild_id, &guild_data);

        //log an event message for the registration
        let arc_mint_log: EventLog = EventLog {
            version: EVENT_ARC_METADATA_SPEC.to_string(),
            standard: EVENT_ARC_STANDARD_NAME.to_string(),
            event: EventLogVariant::ArcMint(vec![ArcMintLog {
                owner_id: guild.manager_id.to_string(),
                token_type: "arc:guild".to_string(),
                token_list: vec![guild_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&arc_mint_log.to_string());

        //refund unused storage fees and return the id to the caller,
        refund_storage_deposit(env::storage_usage() - storage_usage);
        guild_id
    }
}

impl ArcGuildEnumerator for Contract {
    fn arc_guild_count(&self) -> U128 {
        U128(self.guilddata_by_id.len() as u128)
    }

    fn arc_guilds(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonGuild> {
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.tokendata_by_id
            .keys()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|guild_id| self.arc_guild(guild_id.clone()).unwrap())
            .collect()
    }
}
