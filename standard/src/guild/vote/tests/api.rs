use crate::guild::vote::*;

use near_sdk::{test_utils::VMContextBuilder, testing_env};

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

fn get_option_agree(id: VoteId) -> HashMap<VoteId, VoteInfo> {
    let mut options = HashMap::new();
    options.insert(
        id,
        VoteInfo {
            title: "I agree".to_string(),
            details: None,
            reference: None,
            reference_hash: None,
        },
    );
    options
}

fn get_option_duality(idn: VoteId, idy: VoteId) -> HashMap<VoteId, VoteInfo> {
    let mut options = HashMap::new();
    options.insert(
        idn,
        VoteInfo {
            title: "no".to_string(),
            details: None,
            reference: None,
            reference_hash: None,
        },
    );
    options.insert(
        idy,
        VoteInfo {
            title: "yes".to_string(),
            details: None,
            reference: None,
            reference_hash: None,
        },
    );
    options
}

fn get_motion_alpha(time: u64, options: &HashMap<VoteId, VoteInfo>) -> MotionInfo {
    MotionInfo {
        title: "Motion Alpha".to_string(),
        details: " A alpha unit testing motion".to_string(),
        issued_at: time,
        starts_at: time,
        expires_at: time + 1,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: options.clone(),
    }
}

fn get_motion_beta(time: u64, options: &HashMap<VoteId, VoteInfo>) -> MotionInfo {
    MotionInfo {
        title: "Motion Beta".to_string(),
        details: " A beta unit testing motion".to_string(),
        issued_at: time,
        starts_at: time,
        expires_at: time + 1,
        executor: None,
        media: None,
        media_hash: None,
        reference: None,
        reference_hash: None,
        vote_options: options.clone(),
    }
}

// ==== Standard Implementation ====

#[test]
fn votes_new() {
    let data = Votes::new();
    assert_eq!(data.motion_map.len(), 0);
}

#[test]
fn votes_issue() {
    let mid: &MotionId = &"M:01".to_string().into();
    let vid: VoteId = "V:01".to_string().into();
    let time = env::block_timestamp();

    let mut data = Votes::new();
    let options = get_option_agree(vid);
    let motion = get_motion_alpha(time, &options.clone());
    data.issue(mid, &motion.clone());

    let state = data
        .motion_map
        .get(mid)
        .expect("motion_map motion state not found");

    assert_eq!(
        motion.try_to_vec().unwrap(),
        state.info.try_to_vec().unwrap()
    );
    require!(!state.executed, "the member_map value is incorrect");

    let voices = data
        .voices_map
        .get(mid)
        .expect("voices_map motion voices not found");

    require!(
        options.len() == voices.tally.len(),
        "the tally should have the same length as options"
    );

    for (vote_id, _value) in options.iter() {
        let val = voices
            .tally
            .get(vote_id)
            .expect("option not found in tally")
            .clone();
        require!(!val.ceo, "initial tally needs to be zero");
        require!(val.board == 0, "initial tally.board needs to be zero");
        require!(val.members == 0, "initial tally.members needs to be zero");
    }
    require!(0 == voices.votes.len(), "the votes should be zero");
}

#[test]
#[should_panic(expected = "The provided motion id is already in use")]
fn votes_issue_motion_id() {
    let mid: &MotionId = &"M:01".to_string().into();
    let vid: VoteId = "V:01".to_string().into();
    let time = env::block_timestamp();

    let mut data = Votes::new();
    data.issue(mid, &get_motion_beta(time, &get_option_agree(vid.clone())));
    data.issue(mid, &get_motion_alpha(time, &get_option_agree(vid)));
}

#[test]
#[should_panic(expected = "Voting can not start before the motion is published")]
fn votes_issue_start_time() {
    let mut context = VMContextBuilder::new().build();
    context.block_timestamp = 10;
    testing_env!(context);

    let mid: &MotionId = &"M:01".to_string().into();
    let vid: VoteId = "V:01".to_string().into();
    let time = env::block_timestamp();

    let mut data = Votes::new();

    let mut motion = get_motion_alpha(time, &get_option_agree(vid));
    motion.issued_at = time - 1;
    motion.starts_at = time - 1;
    data.issue(mid, &motion);
}

#[test]
fn votes_voice() {
    let ida: &MotionId = &"M:01".to_string().into();
    let idb: &MotionId = &"M:02".to_string().into();
    let idn: VoteId = "V:NO".to_string().into();
    let idy: VoteId = "V:YES".to_string().into();
    let time = env::block_timestamp();

    let mut data = Votes::new();
    data.issue(ida, &get_motion_beta(time, &get_option_agree(idn.clone())));
    data.issue(
        idb,
        &get_motion_alpha(time, &get_option_duality(idn.clone(), idy.clone())),
    );

    require!(
        data.motion_map.len() == 2,
        "voices_map length hsould be two"
    );

    let mut voices = data
        .voices_map
        .get(ida)
        .expect("missing voices on motion a.a");

    require!(voices.votes.len() == 0, "voices.a length hsould be 0");

    data.vote_ceo(ida, idn.clone(), account_nodra!());
    data.vote_board(ida, idn.clone(), account_vecno!());

    voices = data
        .voices_map
        .get(ida)
        .expect("missing voices on motion a.b");

    require!(voices.votes.len() == 2, "voices.a length hsould be 2");
    let voice_nodra = voices
        .votes
        .get(&account_nodra!())
        .expect("missing voice nodra");
    let voice_vecno = voices
        .votes
        .get(&account_vecno!())
        .expect("missing voice vecno");
    require!(voice_nodra == idn.clone(), "voice_nodra should be no");
    require!(voice_vecno == idn.clone(), "voice_vecno should be no");

    let tally_no = voices.tally.get(&idn.clone()).expect("missing tally no");

    require!(tally_no.ceo, "tally_no needs to be true");
    require!(tally_no.board == 1, "tally_no.board should be 1");
    require!(tally_no.members == 2, "tally_no.members should be 2");

    voices = data
        .voices_map
        .get(idb)
        .expect("missing voices on motion b.a");

    require!(voices.votes.len() == 0, "voices.b length hsould be 0");

    data.vote_board(idb, idn.clone(), account_nodra!());
    data.vote_member(idb, idy.clone(), account_vecno!());

    voices = data
        .voices_map
        .get(idb)
        .expect("missing voices on motion b.b");

    require!(voices.votes.len() == 2, "voices.b length hsould be 2");

    let voice_vecno = voices
        .votes
        .get(&account_vecno!())
        .expect("missing voice vecno");
    require!(voice_vecno == idy.clone(), "voice_vecno should be yes");

    let tally_no = voices.tally.get(&idn.clone()).expect("missing tally no");
    let tally_yes = voices.tally.get(&idy.clone()).expect("missing tally yes");

    require!(!tally_no.ceo, "tally_no needs to be false");
    require!(tally_no.board == 1, "tally_no.board should be 1");
    require!(tally_no.members == 1, "tally_no.members should be 1");

    require!(!tally_yes.ceo, "tally_yes needs to be false");
    require!(tally_yes.board == 0, "tally_yes.board should be 0");
    require!(tally_yes.members == 1, "tally_yes.members should be 1");
}

#[test]
#[should_panic(expected = "missing motion vote")]
fn votes_voice_motion() {
    let mid: &MotionId = &"M:01".to_string().into();
    let vid: VoteId = "V:01".to_string().into();
    let eid: VoteId = "V:02".to_string().into();
    let time = env::block_timestamp();

    let mut data = Votes::new();
    let options = get_option_agree(vid);
    let motion = get_motion_alpha(time, &options.clone());
    data.issue(mid, &motion.clone());

    data.vote_ceo(mid, eid.clone(), account_vecno!());
}

#[test]
#[should_panic(expected = "Can not vote on an expired motion")]
fn votes_voice_expired() {
    let mid: &MotionId = &"M:01".to_string().into();
    let vid: VoteId = "V:01".to_string().into();
    let time = env::block_timestamp();

    let mut data = Votes::new();
    let options = get_option_agree(vid.clone());
    let motion = get_motion_alpha(time, &options.clone());
    data.issue(mid, &motion.clone());

    let mut context = VMContextBuilder::new().build();
    context.block_timestamp = 10;
    testing_env!(context);

    data.vote_member(mid, vid.clone(), account_vecno!());
}

#[test]
#[should_panic(expected = "Can not vote on an executed motion")]
fn votes_voice_executed() {
    let mid: &MotionId = &"M:01".to_string().into();
    let vid: VoteId = "V:01".to_string().into();
    let time = env::block_timestamp();

    let mut data = Votes::new();
    let options = get_option_agree(vid.clone());
    let motion = get_motion_alpha(time, &options.clone());
    data.issue(mid, &motion);

    let mut state = data.motion_map.get(mid).expect("err: missing motion");
    state.executed = true;
    data.motion_map.insert(mid, &state);

    data.vote_member(mid, vid.clone(), account_vecno!());
}

#[test]
#[should_panic(expected = "The account has already voted on the motion")]
fn votes_voice_account() {
    let mid: &MotionId = &"M:01".to_string().into();
    let vid: VoteId = "V:01".to_string().into();
    let time = env::block_timestamp();

    let mut data = Votes::new();
    let options = get_option_agree(vid.clone());
    let motion = get_motion_alpha(time, &options.clone());
    data.issue(mid, &motion.clone());

    data.vote_board(mid, vid.clone(), account_vecno!());
    data.vote_member(mid, vid.clone(), account_vecno!());
}
