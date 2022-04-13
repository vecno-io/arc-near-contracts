use super::*;

// ==== Standard Implementation ====

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Votes {
    pub motion_map: UnorderedMap<MotionId, MotionState>,
    pub voices_map: LookupMap<MotionId, MotionVoices>,
}

impl Votes {
    pub fn new() -> Self {
        Self {
            motion_map: UnorderedMap::new(StorageKey::VotesMotionMap.try_to_vec().unwrap()),
            voices_map: LookupMap::new(StorageKey::VotesVoicesMap.try_to_vec().unwrap()),
        }
    }

    pub fn issue(&mut self, id: &MotionId, motion: &MotionInfo) {
        motion.assert_valid();

        require!(
            env::block_timestamp() <= motion.starts_at,
            "Voting can not start before the motion is published"
        );

        let state = MotionState {
            info: motion.clone(),
            executed: false,
        };
        require!(
            self.motion_map.insert(id, &state).is_none(),
            "The provided motion id is already in use"
        );

        let mut voices = MotionVoices {
            tally: HashMap::new(),
            votes: UnorderedMap::new(
                StorageKey::VotesVoicesMapList { id: id.clone() }
                    .try_to_vec()
                    .unwrap(),
            ),
        };
        for (vote_id, _value) in motion.vote_options.iter() {
            // Note: In theory this is imposible, but check
            require!(
                voices
                    .tally
                    .insert(
                        vote_id.clone(),
                        MotionTally {
                            ceo: false,
                            board: 0,
                            members: 0,
                        }
                    )
                    .is_none(),
                format!("Duplicated vote entry found for {}", vote_id)
            );
        }
        self.voices_map.insert(id, &voices);
    }

    pub fn vote_ceo(&mut self, id: &MotionId, vote: VoteId, account: AccountId) -> MotionVoices {
        let mut voices = self.assert_voices(id);
        let tally = self.assert_tally(&vote, &account, &mut voices);

        voices.tally.insert(
            vote,
            MotionTally {
                ceo: true,
                board: tally.board,
                members: tally.members + 1,
            },
        );
        self.voices_map.insert(&id, &voices);

        voices
    }

    pub fn vote_board(&mut self, id: &MotionId, vote: VoteId, account: AccountId) -> MotionVoices {
        let mut voices = self.assert_voices(id);
        let tally = self.assert_tally(&vote, &account, &mut voices);

        voices.tally.insert(
            vote,
            MotionTally {
                ceo: tally.ceo,
                board: tally.board + 1,
                members: tally.members + 1,
            },
        );

        self.voices_map.insert(&id, &voices);
        voices
    }

    pub fn vote_member(&mut self, id: &MotionId, vote: VoteId, account: AccountId) -> MotionVoices {
        let mut voices = self.assert_voices(id);
        let tally = self.assert_tally(&vote, &account, &mut voices);

        voices.tally.insert(
            vote,
            MotionTally {
                ceo: tally.ceo,
                board: tally.board,
                members: tally.members + 1,
            },
        );
        self.voices_map.insert(&id, &voices);

        voices
    }

    fn assert_tally(
        &mut self,
        vote: &VoteId,
        account: &AccountId,
        voices: &mut MotionVoices,
    ) -> MotionTally {
        let tally = voices.tally.get(vote).expect("missing motion vote");
        require!(
            voices.votes.insert(&account, &vote).is_none(),
            "The account has already voted on the motion"
        );
        MotionTally {
            ceo: tally.ceo,
            board: tally.board,
            members: tally.members,
        }
    }

    fn assert_voices(&mut self, id: &MotionId) -> MotionVoices {
        let motion = self.motion_map.get(&id).expect("missing motion info");
        require!(!motion.executed, "Can not vote on an executed motion");
        require!(
            motion.info.expires_at > env::block_timestamp(),
            "Can not vote on an expired motion"
        );

        let voices = self.voices_map.get(&id).expect("missing motion voices");
        voices
    }
}
