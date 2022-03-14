use crate::*;

pub trait ArcGuild {
    fn arc_guild(&self, guild_id: GuildKey) -> Option<JsonGuild>;

    fn arc_guild_count(&self) -> U128;

    fn arc_guilds(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonGuild>;
}

#[macro_export]
macro_rules! impl_arc_guilds {
    ($contract: ident, $tokens: ident, $guilds: ident) => {
        use $crate::guild::*;
        use $crate::*;

        #[near_bindgen]
        impl ArcGuild for $contract {
            fn arc_guild(&self, guild_key: GuildKey) -> Option<JsonGuild> {
                if let Some(guild) = self.$guilds.info_by_id.get(&guild_key) {
                    let data = self.$guilds.data_for_id.get(&guild_key).unwrap();
                    let board = self.$guilds.board_for_id.get(&guild_key).unwrap();
                    return Some(JsonGuild {
                        id: guild_key,
                        ceo: guild.ceo_id,
                        data: data,
                        board: board,
                    });
                }
                return None;
            }

            fn arc_guild_count(&self) -> U128 {
                return U128(self.$guilds.data_for_id.len() as u128);
            }

            fn arc_guilds(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonGuild> {
                let start = u128::from(from_index.unwrap_or(U128(0)));
                return self
                    .$guilds
                    .data_for_id
                    .keys()
                    .skip(start as usize)
                    .take(limit.unwrap_or(50) as usize)
                    .map(|guild_key| self.arc_guild(guild_key.clone()).unwrap())
                    .collect();
            }
        }
    };
}
