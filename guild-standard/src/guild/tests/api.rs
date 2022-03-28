use crate::*;

macro_rules! get_account_nitya {
    () => {
        "nitya.near".parse::<AccountId>().unwrap()
    };
}
macro_rules! get_account_nodra {
    () => {
        "nodra.near".parse::<AccountId>().unwrap()
    };
}
macro_rules! get_account_vecno {
    () => {
        "vecno.near".parse::<AccountId>().unwrap()
    };
}
macro_rules! get_guild_vecno {
    () => {
        GuildInfo {
            ceo_id: get_account_vecno!(),
            ceo_share: 1000,
            board_size: 1,
            board_share: 4000,
            members_size: 2,
            members_share: 5000,
        }
    };
}

// ==== Standard Implementation ====

#[test]
fn guilds_new() {
    let id = &"G:01".to_string().into();
    let data = Guilds::new(id);
    assert_eq!(data.guild_map.len(), 0);
    assert_eq!(data.account_map.len(), 0);

    let state = data
        .state
        .get()
        .expect("the data manager to be set to a guild id");

    assert!(state.vote.is_none());
    assert_eq!(LockedFor::None, state.lock);
    assert_eq!(id.to_string(), state.manager.to_string());
}

#[test]
fn guilds_register() {
    let id = &"G:01".to_string().into();
    let nodra = get_account_nodra!();
    let vecno = get_account_vecno!();
    let guild = &get_guild_vecno!();

    let mut member_map = HashMap::new();
    member_map.insert(nodra.clone(), 123000321 as u128);
    member_map.insert(vecno.clone(), 321000123 as u128);

    let mut board_map = HashMap::new();
    board_map.insert(nodra.clone(), 5050 as u16);

    let mut data = Guilds::new(id);
    data.register(id, guild, &board_map, &member_map);

    let state = data
        .guild_map
        .get(id)
        .expect("guild_map guild G1 not found");
    assert_eq!(
        guild.try_to_vec().unwrap(),
        state.info.try_to_vec().unwrap()
    );

    let board = data
        .board_map
        .get(id)
        .expect("board_map: guild not found: G1");
    match board.list.get(&nodra) {
        Some(share) => assert_eq!(share, 5050),
        _ => panic!("board membership not found"),
    }

    let members = data
        .member_map
        .get(id)
        .expect("member_map: guild not found: G1");
    require!(
        members.value == 444000444,
        "the member_map value is incorrect"
    );
    match members.list.get(&nodra) {
        Some(value) => assert_eq!(value, 123000321),
        _ => panic!("guild membership not found: nodra"),
    }
    match members.list.get(&vecno) {
        Some(value) => assert_eq!(value, 321000123),
        _ => panic!("guild membership not found: vecno"),
    }

    let nodra_map = data
        .account_map
        .get(&nodra)
        .expect("account_map: account not found: nodra");
    require!(
        nodra_map.value == 123000321,
        "nodra_map's value is not correct"
    );
    require!(
        nodra_map.store.contains(id),
        "nodra_map does not contain the guild id"
    );

    let vecno_map = data
        .account_map
        .get(&vecno)
        .expect("account_map: account not found: vecno");
    require!(
        vecno_map.value == 321000123,
        "vecno_map's value is not correct"
    );
    require!(
        vecno_map.store.contains(id),
        "vecno_map does not contain the guild id"
    );
}

#[test]
#[should_panic(expected = "Need at least one guild members or more")]
fn guilds_register_member_min() {
    let id = &"G:01".to_string().into();
    let nodra = get_account_nodra!();
    let guild = &get_guild_vecno!();

    let member_map = HashMap::new();
    let mut board_map = HashMap::new();
    board_map.insert(nodra.clone(), 5050 as u16);

    let mut data = Guilds::new(id);
    data.register(id, &guild, &board_map, &member_map);
}

#[test]
#[should_panic(expected = "The board list can not be larger than the boards size")]
fn guilds_register_board_max() {
    let id = &"G:01".to_string().into();
    let nodra = get_account_nodra!();
    let vecno = get_account_vecno!();

    let mut guild = get_guild_vecno!();
    guild.board_size = 0;
    guild.members_size = 1;

    let mut member_map = HashMap::new();
    member_map.insert(vecno.clone(), 123000321 as u128);

    let mut board_map = HashMap::new();
    board_map.insert(nodra.clone(), 5050 as u16);

    let mut data = Guilds::new(id);
    data.register(id, &guild, &board_map, &member_map);
}

#[test]
#[should_panic(expected = "The members list can not be larger than the guilds size")]
fn guilds_register_members_max() {
    let id = &"G:01".to_string().into();
    let nodra = get_account_nodra!();
    let vecno = get_account_vecno!();

    let mut guild = get_guild_vecno!();
    guild.board_size = 0;
    guild.members_size = 1;

    let mut member_map = HashMap::new();
    member_map.insert(nodra.clone(), 123000321 as u128);
    member_map.insert(vecno.clone(), 321000123 as u128);

    let board_map = HashMap::new();

    let mut data = Guilds::new(id);
    data.register(id, &guild, &board_map, &member_map);
}

#[test]
#[should_panic(expected = "The CEO can not be a board member")]
fn guilds_register_member_ceo() {
    let id = &"G:01".to_string().into();
    let nodra = get_account_nodra!();
    let vecno = get_account_vecno!();
    let guild = &get_guild_vecno!();

    let mut member_map = HashMap::new();
    member_map.insert(nodra.clone(), 123000321 as u128);
    member_map.insert(vecno.clone(), 321000123 as u128);

    let mut board_map = HashMap::new();
    board_map.insert(vecno.clone(), 5050 as u16);

    let mut data = Guilds::new(id);
    data.register(id, guild, &board_map, &member_map);
}

#[test]
#[should_panic(expected = "The CEO must be a guild member")]
fn guilds_register_board_ceo() {
    let id = &"G:01".to_string().into();
    let nodra = get_account_nodra!();
    let guild = &get_guild_vecno!();

    let mut member_map = HashMap::new();
    member_map.insert(nodra.clone(), 123000321 as u128);

    let mut board_map = HashMap::new();
    board_map.insert(nodra.clone(), 5050 as u16);

    let mut data = Guilds::new(id);
    data.register(id, guild, &board_map, &member_map);
}

#[test]
#[should_panic(expected = "The provided guild id is already in use")]
fn guilds_register_guild_id() {
    let id = &"G:01".to_string().into();
    let nodra = get_account_nodra!();
    let vecno = get_account_vecno!();
    let guild = &get_guild_vecno!();

    let mut member_map = HashMap::new();
    member_map.insert(nodra.clone(), 123000321 as u128);
    member_map.insert(vecno.clone(), 321000123 as u128);

    let mut board_map = HashMap::new();
    board_map.insert(nodra.clone(), 5050 as u16);

    let mut data = Guilds::new(id);
    data.register(id, guild, &board_map, &member_map);
    data.register(id, guild, &board_map, &member_map);
}

#[test]
#[should_panic(expected = "Board member need to be a guild member: nodra.near")]
fn guilds_register_board_member() {
    let id = &"G:01".to_string().into();
    let nodra = get_account_nodra!();
    let vecno = get_account_vecno!();
    let guild = &get_guild_vecno!();

    let mut member_map = HashMap::new();
    member_map.insert(vecno.clone(), 321000123 as u128);

    let mut board_map = HashMap::new();
    board_map.insert(nodra.clone(), 5050 as u16);

    let mut data = Guilds::new(id);
    data.register(id, guild, &board_map, &member_map);
}

#[test]
#[should_panic(expected = "Total board shares can not be more than 100_00 basis points")]
fn guilds_register_total_points() {
    let id = &"G:01".to_string().into();
    let nitya = get_account_nitya!();
    let nodra = get_account_nodra!();
    let vecno = get_account_vecno!();

    let mut guild = get_guild_vecno!();
    guild.board_size = 2;
    guild.members_size = 3;

    let mut member_map = HashMap::new();
    member_map.insert(nitya.clone(), 123000321 as u128);
    member_map.insert(nodra.clone(), 321000123 as u128);
    member_map.insert(vecno.clone(), 321000123 as u128);

    let mut board_map = HashMap::new();
    board_map.insert(nitya.clone(), 5000 as u16);
    board_map.insert(nodra.clone(), 5001 as u16);

    let mut data = Guilds::new(id);
    data.register(id, &guild, &board_map, &member_map);
}
