use crate::*;

pub trait ArcGuild {
    fn arc_guild(&self, guild_id: GuildId) -> Option<JsonGuild>;

    fn arc_guild_count(&self) -> U128;

    fn arc_guilds(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonGuild>;
}

pub trait ArcMembers {
    fn arc_guild_membership(&self, token_id: TokenId) -> Option<JsonMember>;

    fn arc_guild_members_count(&self, guild_id: GuildId) -> U128;

    fn arc_guild_members(
        &self,
        guild_id: GuildId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<AccountId>;
}

#[macro_export]
macro_rules! impl_arc_guilds {
    ($contract: ident, $tokens: ident, $guilds: ident) => {
        use $crate::guild::*;
        use $crate::*;

        #[near_bindgen]
        impl ArcGuild for $contract {
            fn arc_guild(&self, guild_id: GuildId) -> Option<JsonGuild> {
                if let Some(guild) = self.$guilds.info_by_id.get(&guild_id) {
                    let data = self.$guilds.data_for_id.get(&guild_id).unwrap();
                    let board = self.$guilds.board_for_id.get(&guild_id).unwrap();
                    return Some(JsonGuild {
                        id: guild_id,
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
                    .map(|guild_id| self.arc_guild(guild_id.clone()).unwrap())
                    .collect();
            }
        }

        #[near_bindgen]
        impl ArcMembers for $contract {
            fn arc_guild_membership(&self, token_id: TokenId) -> Option<JsonMember> {
                if let Some(guild_member) = self.$guilds.member_by_id.get(&token_id) {
                    return Some(JsonMember {
                        guild_id: guild_member.guild_id,
                        owner_id: guild_member.owner_id,
                    });
                }
                return None;
            }

            fn arc_guild_members_count(&self, guild_id: GuildId) -> U128 {
                if let Some(member_list) = self.$guilds.list_per_guild.get(&guild_id) {
                    return U128(member_list.len() as u128);
                }
                return U128::from(0);
            }

            fn arc_guild_members(
                &self,
                guild_id: GuildId,
                from_index: Option<U128>,
                limit: Option<u64>,
            ) -> Vec<AccountId> {
                if let Some(member_list) = self.$guilds.list_per_guild.get(&guild_id) {
                    let start = u128::from(from_index.unwrap_or(U128(0)));
                    return member_list
                        .iter()
                        .skip(start as usize)
                        .take(limit.unwrap_or(50) as usize)
                        .map(|token_id| {
                            self.arc_guild_membership(token_id.clone())
                                .unwrap()
                                .owner_id
                        })
                        .filter(|owner_id| owner_id.is_some())
                        .map(|owner_id| owner_id.unwrap())
                        .collect();
                }
                return vec![];
            }
        }
    };
}
