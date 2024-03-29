use crate::guild::*;

#[derive(BorshSerialize)]
pub enum TestStorageKeys {
    KeyA,
    KeyB,
    KeyC,
}

macro_rules! account_nodra {
    () => {
        "nodra.near".parse::<AccountId>().unwrap()
    };
}
macro_rules! account_vecno {
    () => {
        "vecno.near".parse::<AccountId>().unwrap()
    };
}

// ==== Guild Info ====

#[test]
fn guild_info_assert_new() {
    let ceo = GuildInfo {
        ceo_id: account_nodra!(),
        ceo_share: 10000,
        board_size: 0,
        board_share: 0,
        members_size: 1,
        members_share: 0,
    };
    ceo.assert_valid();
    let board = GuildInfo {
        ceo_id: account_vecno!(),
        ceo_share: 0,
        board_size: 1,
        board_share: 10000,
        members_size: 1,
        members_share: 0,
    };
    board.assert_valid();
    let members = GuildInfo {
        ceo_id: account_nodra!(),
        ceo_share: 0,
        board_size: 5,
        board_share: 0,
        members_size: 10,
        members_share: 10000,
    };
    members.assert_valid();
}

#[test]
#[should_panic(expected = "Members size must be atleast one or more")]
fn guild_info_assert_member_size() {
    let data = GuildInfo {
        ceo_id: account_vecno!(),
        ceo_share: 0,
        board_size: 0,
        board_share: 0,
        members_size: 0,
        members_share: 0,
    };
    data.assert_valid();
}

#[test]
#[should_panic(expected = "Board size can not be larger than members size")]
fn guild_info_assert_board_size() {
    let data = GuildInfo {
        ceo_id: account_vecno!(),
        ceo_share: 0,
        board_size: 2,
        board_share: 0,
        members_size: 1,
        members_share: 0,
    };
    data.assert_valid();
}

#[test]
#[should_panic(expected = "CEO share can not be more than 100_00 basis points")]
fn guild_info_assert_max_ceo_share() {
    let data = GuildInfo {
        ceo_id: account_nodra!(),
        ceo_share: 10001,
        board_size: 0,
        board_share: 0,
        members_size: 1,
        members_share: 0,
    };
    data.assert_valid();
}
#[test]
#[should_panic(expected = "Board share can not be more than 100_00 basis points")]
fn guild_info_assert_max_board_share() {
    let data = GuildInfo {
        ceo_id: account_vecno!(),
        ceo_share: 0,
        board_size: 0,
        board_share: 10001,
        members_size: 1,
        members_share: 0,
    };
    data.assert_valid();
}
#[test]
#[should_panic(expected = "Members share can not be more than 100_00 basis points")]
fn guild_info_assert_max_members_share() {
    let data = GuildInfo {
        ceo_id: account_nodra!(),
        ceo_share: 0,
        board_size: 0,
        board_share: 0,
        members_size: 1,
        members_share: 10001,
    };
    data.assert_valid();
}

#[test]
#[should_panic(expected = "Total shares can not be more than 100_00 basis points")]
fn guild_info_assert_total_shares_ceo() {
    let data = GuildInfo {
        ceo_id: account_vecno!(),
        ceo_share: 3335,
        board_size: 0,
        board_share: 3333,
        members_size: 1,
        members_share: 3333,
    };
    data.assert_valid();
}
#[test]
#[should_panic(expected = "Total shares can not be more than 100_00 basis points")]
fn guild_info_assert_total_shares_board() {
    let data = GuildInfo {
        ceo_id: account_nodra!(),
        ceo_share: 3333,
        board_size: 0,
        board_share: 3335,
        members_size: 1,
        members_share: 3333,
    };
    data.assert_valid();
}
#[test]
#[should_panic(expected = "Total shares can not be more than 100_00 basis points")]
fn guild_info_assert_total_shares_members() {
    let data = GuildInfo {
        ceo_id: account_vecno!(),
        ceo_share: 3333,
        board_size: 0,
        board_share: 3333,
        members_size: 1,
        members_share: 3335,
    };
    data.assert_valid();
}

// ==== Guild Board ====

#[test]
fn guild_board_assert_new() {
    let board = BoardMembers {
        list: UnorderedMap::new(TestStorageKeys::KeyA.try_to_vec().unwrap()),
    };
    board.assert_valid(0);
    let mut board = BoardMembers {
        list: UnorderedMap::new(TestStorageKeys::KeyB.try_to_vec().unwrap()),
    };
    board.list.insert(&account_vecno!(), &5000);
    board.assert_valid(2);
    let mut board = BoardMembers {
        list: UnorderedMap::new(TestStorageKeys::KeyC.try_to_vec().unwrap()),
    };
    board.list.insert(&account_vecno!(), &5000);
    board.list.insert(&account_nodra!(), &5000);
    board.assert_valid(2);
}

#[test]
#[should_panic(expected = "The board can not have more then 1 members")]
fn guild_board_assert_max_members() {
    let mut board = BoardMembers {
        list: UnorderedMap::new(TestStorageKeys::KeyA.try_to_vec().unwrap()),
    };
    board.list.insert(&account_vecno!(), &5000);
    board.list.insert(&account_nodra!(), &5001);
    board.assert_valid(1);
}

#[test]
#[should_panic(expected = "Total shares can not be more than 100_00 basis points")]
fn guild_board_assert_total_shares() {
    let mut board = BoardMembers {
        list: UnorderedMap::new(TestStorageKeys::KeyA.try_to_vec().unwrap()),
    };
    board.list.insert(&account_vecno!(), &5000);
    board.list.insert(&account_nodra!(), &5001);
    board.assert_valid(2);
}

// ==== Guild Members ====

#[test]
fn guild_members_assert_new() {
    let mut board = GuildMembers {
        value: 5000,
        list: UnorderedMap::new(TestStorageKeys::KeyA.try_to_vec().unwrap()),
    };
    board.list.insert(&account_vecno!(), &5000);
    board.assert_valid(2);
    let mut board = GuildMembers {
        value: 10000,
        list: UnorderedMap::new(TestStorageKeys::KeyB.try_to_vec().unwrap()),
    };
    board.list.insert(&account_vecno!(), &5000);
    board.list.insert(&account_nodra!(), &5000);
    board.assert_valid(2);
}

#[test]
#[should_panic(expected = "Guild members size must be atleast one or more")]
fn guild_members_assert_min_members() {
    let board = GuildMembers {
        value: 0,
        list: UnorderedMap::new(TestStorageKeys::KeyA.try_to_vec().unwrap()),
    };
    board.assert_valid(0);
}

#[test]
#[should_panic(expected = "The guild can not have more then 1 members")]
fn guild_members_assert_max_members() {
    let mut board = GuildMembers {
        value: 10000,
        list: UnorderedMap::new(TestStorageKeys::KeyA.try_to_vec().unwrap()),
    };
    board.list.insert(&account_vecno!(), &5000);
    board.list.insert(&account_nodra!(), &5000);
    board.assert_valid(1);
}

#[test]
#[should_panic(expected = "Total value must be the sum of all member values")]
fn guild_members_assert_total_value() {
    let mut board = GuildMembers {
        value: 50000,
        list: UnorderedMap::new(TestStorageKeys::KeyA.try_to_vec().unwrap()),
    };
    board.list.insert(&account_vecno!(), &5000);
    board.list.insert(&account_nodra!(), &5000);
    board.assert_valid(2);
}
