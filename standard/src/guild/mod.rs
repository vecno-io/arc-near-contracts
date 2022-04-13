use crate::*;

use crate::share::*;

pub mod api;
pub mod data;
pub mod vote;

pub use self::api::*;
pub use self::data::*;
pub use self::vote::*;

// ==== Guild contract ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GuildContract {
    votes: Votes,
    guilds: Guilds,
}

impl GuildContract {
    // ==== lock contract ====

    fn act_lock_contract_board(&mut self, details: &String) -> Option<MotionId> {
        // crate a new threshold motion for the board to lock the contract
        let id: MotionId = format!("LC:{}", env::block_height()).into();
        let motion = new_motion_to_lock_contract(details);

        // senders voice agrees by default
        self.votes.issue(&id, &motion);
        // returns a threshold vote
        return Some(id);
    }

    fn exec_lock_contract_ceo(&mut self, id: &MotionId, sender: AccountId) -> Option<u64> {
        self.votes.vote_ceo(&id, get_vote_id_agree(), sender);

        let mut motion = self.votes.motion_map.get(id).expect("missing motion");
        // Validate the motion state before marking it as executed
        require!(!motion.executed, "motion already closed");
        motion.executed = true;

        // Save the motion as executed
        self.votes.motion_map.insert(id, &motion);
        // And return a 24 hour timeout
        return Some(env::block_timestamp() + EXPIRE_24H);
    }

    fn exec_lock_contract_board(
        &mut self,
        id: &MotionId,
        sender: AccountId,
        board_count: u64,
    ) -> Option<u64> {
        let voices = self.votes.vote_board(id, get_vote_id_agree(), sender);

        let mut motion = self.votes.motion_map.get(id).expect("missing motion");
        // Validate the motion state before marking it as executed
        require!(!motion.executed, "motion already closed");
        motion.executed = true;

        // Load threshold tally
        let tally = voices
            .tally
            .get(&get_vote_id_agree())
            .expect("missing tally");
        // When passed the 80% threshold
        if tally.board < ((board_count * 8000) / 10000) {
            // Save the motion as executed
            self.votes.motion_map.insert(id, &motion);
            // And return a 24 hour timeout
            return Some(env::block_timestamp() + EXPIRE_24H);
        }
        return None;
    }

    // ==== unlock contract ====

    fn act_unlock_contract(&mut self, details: &String) -> Option<MotionId> {
        // crate a new threshold motion for the board to unlock the contract
        let id: MotionId = format!("UC:{}", env::block_height()).into();
        let motion = new_motion_to_unlock_contract(details);
        // senders voice agrees by default
        self.votes.issue(&id, &motion);
        // returns a threshold vote
        return Some(id);
    }

    fn exec_unlock_contract(
        &mut self,
        id: &MotionId,
        voices: &MotionVoices,
        board_count: u64,
        member_count: u64,
    ) -> Option<u64> {
        // Load threshold tally
        let tally = voices
            .tally
            .get(&get_vote_id_agree())
            .expect("missing tally");

        // Set the target thresholds to pass at 80%
        let board_target = (board_count * 8000) / 10000;
        let members_target = (member_count * 8000) / 10000;

        // if ceo + (board | members) threshold OR if board & members threshold
        if (tally.ceo && (tally.board >= board_target || tally.members >= members_target))
            || (tally.board >= board_target && tally.members >= members_target)
        {
            let mut motion = self.votes.motion_map.get(id).expect("missing motion");
            motion.executed = true;
            self.votes.motion_map.insert(id, &motion);
            return Some(env::block_timestamp() + EXPIRE_24H);
        }

        return None;
    }
}

impl ContractLocking for GuildContract {
    // ==== lock contract ====

    fn act_lock_contract(&mut self, details: Option<String>) {
        let mut state = self.guilds.state.get().expect("missing guilds state");
        require!(state.lock == LockedFor::None, "contract is not unlocked");
        require!(state.vote.is_none(), "other state motion is active");
        let sender = env::predecessor_account_id();

        if let Some(timeout) = state.time {
            require!(env::block_timestamp() > timeout, "timeout still active");
        }

        // CEO -> insta > locked emergency
        let exec = self
            .guilds
            .guild_map
            .get(&state.exec)
            .expect("missing executive guild");
        if exec.info.ceo_id == sender {
            state.time = None;
            state.vote = None;
            state.lock = LockedFor::Emergency;
            self.guilds.state.set(&state);
            return;
        }
        // Board -> vote > threshold locking
        let board = self
            .guilds
            .board_map
            .get(&state.exec)
            .expect("missing executive board");
        if let Some(_member) = board.list.get(&sender) {
            let details = details.expect("details needed");
            state.time = None;
            state.vote = self.act_lock_contract_board(&details);
            state.lock = LockedFor::Locking;
            self.guilds.state.set(&state);
            return;
        }
        // Error -> caller is not an executive
        env::panic_str("unauthorized call");
    }

    fn vote_lock_contract(&mut self) {
        let mut state = self.guilds.state.get().expect("missing guilds state");
        require!(state.lock == LockedFor::Locking, "contract is not locking");
        let motion = state.vote.expect("missing motion id");
        let sender = env::predecessor_account_id();

        // CEO -> insta > locked emergency
        let exec = self
            .guilds
            .guild_map
            .get(&state.exec)
            .expect("missing executive guild");
        if exec.info.ceo_id == sender {
            state.time = self.exec_lock_contract_ceo(&motion, sender);
            state.vote = None;
            state.lock = LockedFor::Emergency;
            self.guilds.state.set(&state);
            return;
        }
        // Board -> vote > threshold locking
        let board = self
            .guilds
            .board_map
            .get(&state.exec)
            .expect("missing executive board");
        if let Some(_member) = board.list.get(&sender) {
            let board_count = board.list.len();
            // When vote tally is over the threshold > locked emergency
            if let Some(time) = self.exec_lock_contract_board(&motion, sender, board_count) {
                state.time = Some(time);
                state.vote = None;
                state.lock = LockedFor::Emergency;
                self.guilds.state.set(&state);
            }
            return;
        }
        // Error -> caller is not an executive
        env::panic_str("unauthorized call");
    }

    // ==== unlock contract ====

    fn act_unlock_contract(&mut self, details: String) {
        let mut state = self.guilds.state.get().expect("missing guilds state");
        require!(state.lock == LockedFor::Emergency, "contract is not locked");
        require!(state.vote.is_none(), "unlock motion is active");

        if let Some(timeout) = state.time {
            require!(env::block_timestamp() > timeout, "timeout still active");
            state.time = None;
        }
        let sender = env::predecessor_account_id();

        // CEO -> vote > ceo support
        let exec = self
            .guilds
            .guild_map
            .get(&state.exec)
            .expect("missing executive guild");
        if exec.info.ceo_id == sender {
            state.time = None;
            state.vote = self.act_unlock_contract(&details);
            state.lock = LockedFor::Emergency;
            self.guilds.state.set(&state);
            return;
        }
        // Board -> vote > threshold + ceo | members
        let board = self
            .guilds
            .board_map
            .get(&state.exec)
            .expect("missing executive board");
        if let Some(_member) = board.list.get(&sender) {
            state.time = None;
            state.vote = self.act_unlock_contract(&details);
            state.lock = LockedFor::Emergency;
            self.guilds.state.set(&state);
            return;
        }
        // Error -> caller is not authorized
        env::panic_str("unauthorized call");
    }

    fn vote_unlock_contract(&mut self) {
        let mut state = self.guilds.state.get().expect("missing guilds state");
        require!(state.lock == LockedFor::Emergency, "contract is not locked");
        require!(state.vote.is_some(), "unlock motion is mot active");

        let motion = state.vote.expect("missing motion id");
        let sender = env::predecessor_account_id();

        let board = self
            .guilds
            .board_map
            .get(&state.exec)
            .expect("missing executive board");
        let board_count = board.list.len();
        let member_count = self.guilds.account_map.len();

        // CEO -> vote > ceo support
        let exec = self
            .guilds
            .guild_map
            .get(&state.exec)
            .expect("missing executive guild");
        if exec.info.ceo_id == sender {
            let voices = self.votes.vote_ceo(&motion, get_vote_id_agree(), sender);
            if let Some(time) =
                self.exec_unlock_contract(&motion, &voices, board_count, member_count)
            {
                state.time = Some(time);
                state.vote = None;
                state.lock = LockedFor::None;
                self.guilds.state.set(&state);
            }
            return;
        }
        // Board -> vote > threshold + ceo | members
        if let Some(_member) = board.list.get(&sender) {
            let voices = self.votes.vote_board(&motion, get_vote_id_agree(), sender);
            if let Some(time) =
                self.exec_unlock_contract(&motion, &voices, board_count, member_count)
            {
                state.time = Some(time);
                state.vote = None;
                state.lock = LockedFor::None;
                self.guilds.state.set(&state);
            }
            return;
        }
        // Member Accounts -> Vote -> threshold + ceo | board
        if let Some(member_set) = self.guilds.account_map.get(&sender) {
            if !member_set.store.is_empty() {
                let voices = self.votes.vote_member(&motion, get_vote_id_agree(), sender);
                if let Some(time) =
                    self.exec_unlock_contract(&motion, &voices, board_count, member_count)
                {
                    state.time = Some(time);
                    state.vote = None;
                    state.lock = LockedFor::None;
                    self.guilds.state.set(&state);
                }
                return;
            }
        }
        // Error -> caller is not authorized
        env::panic_str("unauthorized call");
    }
}

// impl ContractChallenging for CoreContract {
//     fn act_challenge_contract_exec(&mut self, details: String) {
//         todo!()
//     }

//     fn vote_challenge_contract_exec(&mut self) {
//         todo!()
//     }

//     fn act_replace_contract_exec(&mut self, details: String) {
//         todo!()
//     }

//     fn vote_replace_contract_exec(&mut self) {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    mod api;
    mod data;
    mod locks;
}
