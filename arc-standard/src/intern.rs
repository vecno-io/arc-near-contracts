use crate::*;

use std::collections::HashMap;
use std::mem::size_of;

pub const NO_DEPOSIT: Balance = 0;
pub const MAX_BASE_POINTS_TOTAL: u16 = 10000;

pub const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);
pub const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
pub const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(35_000_000_000_000);
pub const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);

//used to make sure the user attached exactly 1 yoctoNEAR
pub fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yocto",
    )
}

//used to make sure the user attached exactly 1 yoctoNEAR
pub fn assert_min_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yocto",
    )
}

//used to generate a unique fixed size prefix for storage
pub fn hash_storage_key(bytes: &[u8]) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(bytes));
    hash
}

//refund the initial deposit based on the amount of storage that was used up
pub fn refund_storage_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();
    let finalized_refund = attached_deposit - required_cost;
    //make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yocto to cover storage cost",
        required_cost,
    );

    if finalized_refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(finalized_refund);
    }
}

//calculate how many bytes the account ID is taking up
pub fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    // The extra 4 bytes are coming from Borsh serialization to store the length of the string.
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

//refund the storage taken up by passed in approved account IDs and send the funds to the passed in account ID.
pub fn refund_approved_account_ids_iter<'a, I>(
    account_id: AccountId,
    approved_accounts: I,
) -> Promise
where
    I: Iterator<Item = &'a AccountId>,
{
    //get the storage total by going through and summing all the bytes for each approved account IDs
    let storage_released: u64 = approved_accounts.map(bytes_for_approved_account_id).sum();
    Promise::new(account_id).transfer(Balance::from(storage_released) * env::storage_byte_cost())
}

//refund a map of approved account IDs and send the funds to the passed in account ID
pub fn refund_approved_accounts(
    account_id: AccountId,
    approved_accounts: &HashMap<AccountId, u64>,
) -> Promise {
    //call the refund_approved_account_ids_iter with the approved account IDs as keys
    refund_approved_account_ids_iter(account_id, approved_accounts.keys())
}

//convert the royalty percentage and amount to pay into a payout (U128)
pub fn royalty_to_payout(royalty_percentage: u16, amount_to_pay: Balance) -> U128 {
    U128(royalty_percentage as u128 * amount_to_pay / MAX_BASE_POINTS_TOTAL as u128)
}
