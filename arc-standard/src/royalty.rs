use crate::*;

use std::collections::HashMap;

//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

pub trait NftRoyalties {
    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout;

    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenKey,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

#[near_bindgen]
impl NftRoyalties for Contract {
    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout {
        //get the token info for the provided token id or panic with message
        let token = self
            .tokens
            .info_by_id
            .get(&token_id.into())
            .expect("Token not found");
        self.payouts(&token, u128::from(balance), max_len_payout)
    }

    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenKey,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        assert_one_yocto();
        let token = self.transfer(
            &env::predecessor_account_id(),
            &receiver_id,
            &token_id,
            Some(approval_id),
            Some(memo),
        );
        refund_approved_accounts(token.owner_id.clone(), &token.approved_accounts);
        self.payouts(&token, u128::from(balance), max_len_payout)
    }
}
