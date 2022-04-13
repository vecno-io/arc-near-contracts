use crate::token::*;

macro_rules! new_account_id {
    ($str: expr) => {
        $str.parse::<AccountId>().unwrap()
    };
}

#[test]
fn test_token_payout_assert_valid() {
    let mut alpha = TokenPayout::new();
    alpha.accounts.insert(new_account_id!("alpha.near"), 2500);
    alpha.accounts.insert(new_account_id!("beta.near"), 2500);
    alpha.accounts.insert(new_account_id!("gamma.near"), 2500);
    alpha.accounts.insert(new_account_id!("delta.near"), 2500);
    alpha.require_valid();

    let mut beta = TokenPayout::new();
    beta.accounts.insert(new_account_id!("alpha.near"), 2000);
    beta.accounts.insert(new_account_id!("beta.near"), 2000);
    beta.accounts.insert(new_account_id!("gamma.near"), 2000);
    beta.accounts.insert(new_account_id!("delta.near"), 2000);
    beta.guild = 2000;
    beta.require_valid();
}

#[test]
#[should_panic(expected = "Cannot add more than 4 payouts per token")]
fn test_token_payout_assert_valid_len() {
    let mut alpha = TokenPayout::new();
    alpha.accounts.insert(new_account_id!("alpha.near"), 100);
    alpha.accounts.insert(new_account_id!("beta.near"), 100);
    alpha.accounts.insert(new_account_id!("gamma.near"), 100);
    alpha.accounts.insert(new_account_id!("delta.near"), 100);
    alpha.accounts.insert(new_account_id!("epsi.near"), 100);
    alpha.require_valid();
}

#[test]
#[should_panic(expected = "The total for payouts can not be more than 10000, got 10002")]
fn test_token_payout_assert_valid_total() {
    let mut alpha = TokenPayout::new();
    alpha.accounts.insert(new_account_id!("alpha.near"), 5001);
    alpha.accounts.insert(new_account_id!("beta.near"), 5001);
    alpha.require_valid();
}

#[test]
#[should_panic(expected = "The total for payouts can not be more than 10000, got 12002")]
fn test_token_payout_assert_valid_total_guild() {
    let mut alpha = TokenPayout::new();
    alpha.accounts.insert(new_account_id!("alpha.near"), 5001);
    alpha.accounts.insert(new_account_id!("beta.near"), 5001);
    alpha.guild = 2000;
    alpha.require_valid();
}
