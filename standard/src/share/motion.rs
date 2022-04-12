use crate::*;

pub const EXPIRE_12H: u64 = 43200;
pub const EXPIRE_24H: u64 = 86400;
pub const EXPIRE_48H: u64 = 172800;

#[inline(always)]
pub fn get_vote_id_no() -> VoteId {
    return "V:N".to_string().into();
}

#[inline(always)]
pub fn get_vote_id_yes() -> VoteId {
    return "V:Y".to_string().into();
}

#[inline(always)]
pub fn get_vote_id_agree() -> VoteId {
    return "V:A".to_string().into();
}

pub fn new_motion_to_lock_contract(details: &String) -> MotionInfo {
    let mut options = HashMap::new();
    options.insert(
        get_vote_id_agree(),
        VoteInfo {
            title: "agree".to_string(),
            details: Some("You agree to lock the contract.".to_string()),
            reference: None,
            reference_hash: None,
        },
    );
    MotionInfo {
        title: "Contract Lockdown".to_string(),
        details: details.clone(),
        issued_at: env::block_timestamp(),
        starts_at: env::block_timestamp(),
        expires_at: env::block_timestamp() + EXPIRE_12H,
        executor: Some(env::current_account_id()),
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: options,
    }
}

pub fn new_motion_to_unlock_contract(details: &String) -> MotionInfo {
    let mut options = HashMap::new();
    options.insert(
        get_vote_id_agree(),
        VoteInfo {
            title: "agree".to_string(),
            details: Some("You agree to unlock the contract.".to_string()),
            reference: None,
            reference_hash: None,
        },
    );
    MotionInfo {
        title: "Contract Unlock".to_string(),
        details: details.clone(),
        issued_at: env::block_timestamp(),
        starts_at: env::block_timestamp(),
        expires_at: env::block_timestamp() + EXPIRE_48H,
        executor: Some(env::current_account_id()),
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: options,
    }
}

pub fn new_motion_to_lock_guild(details: String) -> MotionInfo {
    let mut options = HashMap::new();
    options.insert(
        get_vote_id_agree(),
        VoteInfo {
            title: "agree".to_string(),
            details: Some("You agree to lock the guild.".to_string()),
            reference: None,
            reference_hash: None,
        },
    );
    MotionInfo {
        title: "Guild Lockdown".to_string(),
        details: details.clone(),
        issued_at: env::block_timestamp(),
        starts_at: env::block_timestamp(),
        expires_at: env::block_timestamp() + EXPIRE_12H,
        executor: Some(env::current_account_id()),
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: options,
    }
}

pub fn new_motion_to_unlock_guild(details: String) -> MotionInfo {
    let mut options = HashMap::new();
    options.insert(
        get_vote_id_agree(),
        VoteInfo {
            title: "agree".to_string(),
            details: Some("You agree to unlock the guild.".to_string()),
            reference: None,
            reference_hash: None,
        },
    );
    MotionInfo {
        title: "Guild Unlock".to_string(),
        details: details.clone(),
        issued_at: env::block_timestamp(),
        starts_at: env::block_timestamp(),
        expires_at: env::block_timestamp() + EXPIRE_48H,
        executor: Some(env::current_account_id()),
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: options,
    }
}
