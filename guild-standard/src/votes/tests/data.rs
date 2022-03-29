use crate::*;

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

fn get_option_agree() -> HashMap<VoteId, VoteInfo> {
    let mut options = HashMap::new();
    options.insert(
        "V:01".to_string().into(),
        VoteInfo {
            title: "I agree".to_string(),
            details: None,
            reference: None,
            reference_hash: None,
        },
    );
    options
}

fn get_option_duality() -> HashMap<VoteId, VoteInfo> {
    let mut options = HashMap::new();
    options.insert(
        "V:NO".to_string().into(),
        VoteInfo {
            title: "no".to_string(),
            details: None,
            reference: None,
            reference_hash: None,
        },
    );
    options.insert(
        "V:YES".to_string().into(),
        VoteInfo {
            title: "yes".to_string(),
            details: None,
            reference: None,
            reference_hash: None,
        },
    );
    options
}

// ==== Type IDs ====

mod vote_id {
    use crate::*;

    impl_string_id_tests!("vote", VoteId);
}

mod motion_id {
    use crate::*;

    impl_string_id_tests!("motion", MotionId);
}

#[test]
fn vote_info_assert_new() {
    let base = VoteInfo {
        title: "title".to_string(),
        details: None,
        reference: None,
        reference_hash: None,
    };
    base.assert_valid();
    let extra = VoteInfo {
        title: "123456789-123456789-12345678".to_string(),
        details: Some("details".to_string()),
        reference: None,
        reference_hash: None,
    };
    extra.assert_valid();
    let detail = VoteInfo {
        title: "t".to_string(),
        details: Some("123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-12345678".to_string()),
        reference: None,
        reference_hash: None,
    };
    detail.assert_valid();
    let reference = VoteInfo {
        title: "title".to_string(),
        details: Some("".to_string()),
        reference: Some("ipfs://QmWcSUSvv9Pq2n72V9aoPQUwaMqWeMZt7W1Quvm6RisSja".to_string()),
        reference_hash: Some(
            "0ae5c849b217f388fb5de38c3f28dd52ab3c4ea03aee780e1e24092084c4c528".to_string(),
        ),
    };
    reference.assert_valid();
}

#[test]
#[should_panic(expected = "Vote info requires a title is required")]
fn vote_info_assert_title() {
    let info = VoteInfo {
        title: "".to_string(),
        details: None,
        reference: None,
        reference_hash: None,
    };
    info.assert_valid();
}

#[test]
#[should_panic(expected = "Maximum title length is 28 characters")]
fn vote_info_assert_title_max() {
    let info = VoteInfo {
        title: "123456789-123456789-123456789".to_string(),
        details: None,
        reference: None,
        reference_hash: None,
    };
    info.assert_valid();
}

#[test]
#[should_panic(expected = "Maximum details length is 128 characters")]
fn vote_info_assert_details_max() {
    let info = VoteInfo {
        title: "title".to_string(),
        details: Some("123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789".to_string()),
        reference: None,
        reference_hash: None,
    };
    info.assert_valid();
}

#[test]
#[should_panic(expected = "Reference hash is required to verify reference integrity")]
fn vote_info_assert_reference() {
    let info = VoteInfo {
        title: "title".to_string(),
        details: None,
        reference: Some("ipfs://QmWcSUSvv9Pq2n72V9aoPQUwaMqWeMZt7W1Quvm6RisSja".to_string()),
        reference_hash: None,
    };
    info.assert_valid();
}

#[test]
#[should_panic(expected = "Reference hash has to be hex encoded string (64 bytes)")]
fn vote_info_assert_reference_hash() {
    let info = VoteInfo {
        title: "title".to_string(),
        details: None,
        reference: Some("ipfs://QmWcSUSvv9Pq2n72V9aoPQUwaMqWeMZt7W1Quvm6RisSja".to_string()),
        reference_hash: Some("0b5de38c3f28dd52ab3c4ea03aee780e1e24092084c4c528".to_string()),
    };
    info.assert_valid();
}

#[test]
fn motion_info_assert_new() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
    let extra = MotionInfo {
        title: "123456789-123456789-12345678".to_string(),
        details: "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-12345678".to_string(),
        issued_at: 1,
        starts_at: 2,
        expires_at: 3,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_duality(),
    };
    extra.assert_valid();
    let exec = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: Some(account_nodra!()),
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    exec.assert_valid();
    let exec = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 4,
        starts_at: 4,
        expires_at: 6,
        executor: Some(account_vecno!()),
        media: Some("ipfs://QmdCT4XXrRqSZHrY3KSnbXMNWWqw4PFLHCTdretVm5DetQ".to_string()),
        media_hash: Some(
            "3314183bf8882abf6b262de8ed81d9677906a36fe688c08cd604e6518a3587ac".to_string(),
        ),
        reference: Some("ipfs://QmWcSUSvv9Pq2n72V9aoPQUwaMqWeMZt7W1Quvm6RisSja".to_string()),
        reference_hash: Some(
            "0ae5c849b217f388fb5de38c3f28dd52ab3c4ea03aee780e1e24092084c4c528".to_string(),
        ),
        vote_options: get_option_duality(),
    };
    exec.assert_valid();
}

#[test]
#[should_panic(expected = "Motion info requires a title is required")]
fn motion_info_assert_title() {
    let base = MotionInfo {
        title: "".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "Maximum title length is 28 characters")]
fn motion_info_assert_title_max() {
    let base = MotionInfo {
        title: "123456789-123456789-123456789".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "Maximum details length is 128 characters")]
fn motion_info_assert_details() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "Voting can not start before issuing the motion")]
fn motion_info_assert_time_start() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 2,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "The motion must expire after the voting started")]
fn motion_info_assert_time_expires() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 1,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "The motion requires at least one option to vote on")]
fn motion_info_assert_no_options() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: HashMap::new(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "Media hash is required to verify media integrity")]
fn motion_info_assert_media() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: Some("ipfs://QmWcSUSvv9Pq2n72V9aoPQUwaMqWeMZt7W1Quvm6RisSja".to_string()),
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "Media hash has to be hex encoded string (64 bytes)")]
fn motion_info_assert_media_hash() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: Some("ipfs://QmWcSUSvv9Pq2n72V9aoPQUwaMqWeMZt7W1Quvm6RisSja".to_string()),
        media_hash: Some("0b5de38c3f28dd52ab3c4ea03aee780e1e24092084c4c528".to_string()),
        reference: None,
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "Reference hash is required to verify reference integrity")]
fn motion_info_assert_reference() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: None,
        media_hash: None,
        reference: Some("ipfs://QmWcSUSvv9Pq2n72V9aoPQUwaMqWeMZt7W1Quvm6RisSja".to_string()),
        reference_hash: None,
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}

#[test]
#[should_panic(expected = "Reference hash has to be hex encoded string (64 bytes)")]
fn motion_info_assert_reference_hash() {
    let base = MotionInfo {
        title: "title".to_string(),
        details: "details".to_string(),
        issued_at: 0,
        starts_at: 1,
        expires_at: 2,
        executor: None,
        media: None,
        media_hash: None,
        reference: Some("ipfs://QmWcSUSvv9Pq2n72V9aoPQUwaMqWeMZt7W1Quvm6RisSja".to_string()),
        reference_hash: Some("0b5de38c3f28dd52ab3c4ea03aee780e1e24092084c4c528".to_string()),
        vote_options: get_option_agree(),
    };
    base.assert_valid();
}
