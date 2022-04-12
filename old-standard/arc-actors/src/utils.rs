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
pub fn require_one_yocto() {
    require!(
        env::attached_deposit() == 1,
        "Requires attached deposit of exactly 1 yocto",
    )
}

//used to make sure the user attached exactly 1 yoctoNEAR
pub fn require_min_one_yocto() {
    require!(
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
    require!(
        required_cost <= attached_deposit,
        format!("Must attach {} yocto to cover storage cost", required_cost),
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
#[inline(always)]
pub fn refund_approved_accounts(
    account_id: AccountId,
    approved_accounts: &HashMap<AccountId, u64>,
) -> Promise {
    //call the refund_approved_account_ids_iter with the approved account IDs as keys
    refund_approved_account_ids_iter(account_id, approved_accounts.keys())
}

/// Converts the input into an amount to pay out.
///
/// Note: It does not validate, the caller needs to ensure values are valid.
#[inline(always)]
pub fn royalty_to_payout(royalty_percentage: u16, amount_to_pay: Balance) -> U128 {
    U128(royalty_percentage as u128 * amount_to_pay / MAX_BASE_POINTS_TOTAL as u128)
}

#[macro_export]
macro_rules! impl_item_is_owned {
    ($struct: ident, $key: ident, $store: ident) => {
        impl $struct {
            pub fn add_to_owner(&mut self, owner: AccountKey, id: &$key) {
                let mut owner_set = self.list_per_owner.get(&owner).unwrap_or_else(|| {
                    UnorderedSet::new(
                        StorageKey::$store {
                            owner_key: owner.clone(),
                        }
                        .try_to_vec()
                        .unwrap(),
                    )
                });
                owner_set.insert(id);
                self.list_per_owner.insert(&owner, &owner_set);
            }

            pub fn remove_from_owner(&mut self, owner: AccountKey, id: &$key) {
                let mut owner_set = self
                    .list_per_owner
                    .get(&owner)
                    .expect("owner set not found");
                if owner_set.len() == 1 {
                    self.list_per_owner.remove(&owner);
                } else {
                    owner_set.remove(id);
                    self.list_per_owner.insert(&owner, &owner_set);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_macro_royalty_to_payout {
        ($share: expr, $total: expr, $expect: expr) => {
            let amount = royalty_to_payout($share, $total);
            assert_eq!(amount, $expect.into());
        };
    }

    #[test]
    fn test_royalty_to_payout() {
        test_macro_royalty_to_payout!(0, 100, 0);

        test_macro_royalty_to_payout!(5, 1000, 0);
        test_macro_royalty_to_payout!(25, 1000, 2);
        test_macro_royalty_to_payout!(125, 1000, 12);

        test_macro_royalty_to_payout!(125, 100, 1);
        test_macro_royalty_to_payout!(125, 1000, 12);
        test_macro_royalty_to_payout!(125, 10000, 125);

        test_macro_royalty_to_payout!(10000, 100, 100);
    }
}
