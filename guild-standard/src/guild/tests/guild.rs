use crate::*;

#[test]
fn test_guild_new() {
    let data = Guild::new();
    data.assert_valid();
}
