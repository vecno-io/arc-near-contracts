use crate::*;

use near_sdk::json_types::U128;
use near_sdk::Gas;
use std::collections::HashMap;
use std::mem::size_of;

pub(crate) const NO_DEPOSIT: Balance = 0;
pub(crate) const MAX_TOTAL_ROYALTIES: u32 = 10000;

pub(crate) const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
pub(crate) const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(35_000_000_000_000);
pub(crate) const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);

//used to make sure the user attached exactly 1 yoctoNEAR
pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yocto",
    )
}

//used to make sure the user attached exactly 1 yoctoNEAR
pub(crate) fn assert_min_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yocto",
    )
}

//used to generate a unique fixed size prefix for storage
pub(crate) fn hash_storage_key(bytes: &[u8]) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(bytes));
    hash
}

//refund the initial deposit based on the amount of storage that was used up
pub(crate) fn refund_storage_deposit(storage_used: u64) {
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
pub(crate) fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    // The extra 4 bytes are coming from Borsh serialization to store the length of the string.
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

//refund the storage taken up by passed in approved account IDs and send the funds to the passed in account ID.
pub(crate) fn refund_approved_account_ids_iter<'a, I>(
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
pub(crate) fn refund_approved_accounts(
    account_id: AccountId,
    approved_accounts: &HashMap<AccountId, u64>,
) -> Promise {
    //call the refund_approved_account_ids_iter with the approved account IDs as keys
    refund_approved_account_ids_iter(account_id, approved_accounts.keys())
}

//convert the royalty percentage and amount to pay into a payout (U128)
pub(crate) fn royalty_to_payout(royalty_percentage: u32, amount_to_pay: Balance) -> U128 {
    U128(royalty_percentage as u128 * amount_to_pay / MAX_TOTAL_ROYALTIES as u128)
}

impl Contract {
    pub(crate) fn add_token_to_owner(&mut self, token_id: &TokenId, owner_id: &AccountId) -> bool {
        let mut created = false;
        let owner_key = hash_storage_key(owner_id.as_bytes());
        let mut tokens_set = self.tokens_per_owner.get(&owner_key).unwrap_or_else(|| {
            created = true;
            UnorderedSet::new(
                StorageKey::TokensPerOwnerSet { owner_key }
                    .try_to_vec()
                    .unwrap(),
            )
        });

        tokens_set.insert(token_id);
        self.tokens_per_owner.insert(&owner_key, &tokens_set);

        created
    }

    pub(crate) fn remove_token_from_owner(&mut self, token_id: &TokenId, account_id: &AccountId) {
        let owner_key = hash_storage_key(account_id.as_bytes());

        let mut tokens_set = self
            .tokens_per_owner
            .get(&owner_key)
            .expect("Sender must own the token");

        tokens_set.remove(token_id);

        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(&owner_key);
        } else {
            self.tokens_per_owner.insert(&owner_key, &tokens_set);
        }
    }

    pub(crate) fn transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Token {
        //get the token info for the provided token id or panic with message
        let token = self
            .tokens_by_id
            .get(token_id)
            .expect("Token info not found");

        //if the sender doesn't equal the owner, we check if the sender is in the approval list
        if sender_id != &token.owner_id {
            //if the token's approved account IDs doesn't contain the sender, we panic
            if !token.approved_accounts.contains_key(sender_id) {
                env::panic_str("Unauthorized transfer");
            }

            // If they included an approval_id, check if the sender's actual approval_id is the same as the one included
            if let Some(enforced_approval_id) = approval_id {
                //get the actual approval ID
                let actual_approval_id = token
                    .approved_accounts
                    .get(sender_id)
                    //if the sender isn't in the map, we panic
                    .expect("Sender is not authorized to transfer");

                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "Sender provided an invalid approval id",
                );
            }
        }

        //we make sure that the sender isn't sending the token to themselves
        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        //remove the token fro mthe old owner and add it to the new owner
        self.remove_token_from_owner(token_id, &token.owner_id);
        self.add_token_to_owner(token_id, receiver_id);

        //create the token and store it
        let new_token = Token {
            owner_id: receiver_id.clone(),
            approved_accounts: Default::default(),
            approval_index: token.approval_index,
            royalty: token.royalty.clone(),
        };
        self.tokens_by_id.insert(token_id, &new_token);

        //log the memo message if one is provided
        if let Some(memo) = memo.as_ref() {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }

        //log an event message for the transfer
        let mut authorized_id = None;
        if approval_id.is_some() {
            authorized_id = Some(sender_id.to_string());
        }
        let nft_transfer_log: EventLog = EventLog {
            standard: EVENT_NFT_STANDARD_NAME.to_string(),
            version: EVENT_NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                authorized_id,
                old_owner_id: token.owner_id.to_string(),
                new_owner_id: receiver_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo,
            }]),
        };
        env::log_str(&nft_transfer_log.to_string());

        token
    }

    pub(crate) fn payouts(&self, token: &Token, amount: u128, max_payouts: u32) -> Payout {
        //gas might be a limiting factor, panic if needed
        assert!(
            token.royalty.len() as u32 <= max_payouts,
            "The request cannot payout all royalties"
        );
        //track the total perpetual royalties
        let mut total_perpetual = 0;
        let mut payout_object = Payout {
            payout: HashMap::new(),
        };
        //add all royalties to the payout list
        for (k, v) in token.royalty.iter() {
            let key = k.clone();
            if key != token.owner_id {
                payout_object
                    .payout
                    .insert(key, royalty_to_payout(*v, amount));
                total_perpetual += *v;
            }
        }
        //payout the remaining amount to the owner
        payout_object.payout.insert(
            token.owner_id.clone(),
            royalty_to_payout(MAX_TOTAL_ROYALTIES - total_perpetual, amount),
        );
        payout_object
    }
}
