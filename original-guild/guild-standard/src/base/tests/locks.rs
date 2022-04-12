use crate::*;

// ==== act_lock_contract ====

#[test]
fn act_lock_contract_as_ceo() {}

#[test]
fn act_lock_contract_as_board() {}

#[test]
#[should_panic(expected = "")]
fn act_lock_contract_no_sate() {}

#[test]
#[should_panic(expected = "")]
fn act_lock_contract_active_lock() {}

#[test]
#[should_panic(expected = "")]
fn act_lock_contract_active_motion() {}

#[test]
#[should_panic(expected = "")]
fn act_lock_contract_active_timeout() {}

#[test]
#[should_panic(expected = "")]
fn act_lock_contract_missing_exec() {}

#[test]
#[should_panic(expected = "")]
fn act_lock_contract_missing_board() {}

#[test]
#[should_panic(expected = "")]
fn act_lock_contract_needs_details() {}

#[test]
#[should_panic(expected = "")]
fn act_lock_contract_not_authorized() {}

// ==== vote_lock_contract ====

#[test]
fn vote_lock_contract_as_ceo() {}

#[test]
fn vote_lock_contract_as_board() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_no_sate() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_not_locking() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_no_motion() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_missing_exec() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_missing_board() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_as_ceo_close() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_as_board_close() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_as_board_no_tally() {}

#[test]
#[should_panic(expected = "")]
fn vote_lock_contract_not_authorized() {}

// ==== act_unlock_contract ====

#[test]
fn act_unlock_contract_as_ceo() {}

#[test]
fn act_unlock_contract_as_board() {}

#[test]
#[should_panic(expected = "")]
fn act_unlock_contract_no_sate() {}

#[test]
#[should_panic(expected = "")]
fn act_unlock_contract_active_lock() {}

#[test]
#[should_panic(expected = "")]
fn act_unlock_contract_active_motion() {}

#[test]
#[should_panic(expected = "")]
fn act_unlock_contract_active_timeout() {}

#[test]
#[should_panic(expected = "")]
fn act_unlock_contract_missing_exec() {}

#[test]
#[should_panic(expected = "")]
fn act_unlock_contract_missing_board() {}

#[test]
#[should_panic(expected = "")]
fn act_unlock_contract_not_authorized() {}
