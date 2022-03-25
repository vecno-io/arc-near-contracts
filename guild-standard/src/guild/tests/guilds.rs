use crate::*;

#[test]
fn test_guilds_new() {
    let data = Guilds::new();
    data.assert_valid();
}