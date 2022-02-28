use crate::*;

use near_sdk::{ext_contract, Gas};

const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);

pub trait NftApproval {
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;

    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);

    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_nft_receiver)]
trait NftReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NftApproval for Contract {
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        //get the token info for the provided token id or panic with message
        let token = self.tokens_by_id.get(&token_id).expect("Token not found");

        //if there is an aproval id stored for the provided account id:
        if let Some(approval) = token.approved_accounts.get(&approved_account_id) {
            //then return true or validate the provided id
            if let Some(approval_id) = approval_id {
                approval_id == *approval
            } else {
                true
            }
        } else {
            false
        }
    }

    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        assert_min_one_yocto();
        //get the token info for the provided token id or panic with message
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");

        //validate the caller is the owner
        assert_eq!(
            &env::predecessor_account_id(),
            &token.owner_id,
            "Signer must be the token owner."
        );

        //increment the approval id for the token
        let approval_id: u64 = token.approval_index;
        token.approval_index += 1;

        //update the approved accounts list
        let is_new_approval = token
            .approved_accounts
            .insert(account_id.clone(), approval_id)
            .is_none();
        self.tokens_by_id.insert(&token_id, &token);

        //withhold storage cost when needed, else refund all
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        } else {
            0
        };
        refund_storage_deposit(storage_used);

        //if message, call on approval
        if let Some(msg) = msg {
            ext_nft_receiver::nft_on_approve(
                token_id,
                token.owner_id,
                approval_id,
                msg,
                account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
            )
            .as_return();
        }
    }

    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        assert_one_yocto();
        //get the token info for the provided token id or panic with message
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");

        //validate the caller is the owner
        let sender_id = env::predecessor_account_id();
        assert_eq!(
            &sender_id, &token.owner_id,
            "Signer must be the token owner."
        );

        if token.approved_accounts.remove(&account_id).is_some() {
            refund_approved_account_ids_iter(sender_id, [account_id].iter());
            self.tokens_by_id.insert(&token_id, &token);
        }
    }

    fn nft_revoke_all(&mut self, token_id: TokenId) {
        assert_one_yocto();
        //get the token info for the provided token id or panic with message
        let mut token = self.tokens_by_id.get(&token_id).expect("Token not found");

        //validate the caller is the owner
        let sender_id = env::predecessor_account_id();
        assert_eq!(
            &sender_id, &token.owner_id,
            "Signer must be the token owner."
        );

        if !token.approved_accounts.is_empty() {
            refund_approved_accounts(sender_id, &token.approved_accounts);
            token.approved_accounts.clear();
            self.tokens_by_id.insert(&token_id, &token);
        }
    }
}
