use crate::*;

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
            require!(
                // Note: In theory this is imposible, but check
                voices.tally.insert(vote_id.clone(), 0).is_none(),
                format!("Duplicated vote entry found for {}", vote_id)
            );
        }
        self.voices_map.insert(id, &voices);
    }

    pub fn voice(&mut self, id: &MotionId, vote: VoteId, account: AccountId) {
        let motion = self.motion_map.get(&id).expect("missing motion info");
        require!(
            motion.info.expires_at > env::block_timestamp(),
            "Can not vote on an expired motion"
        );

        let mut voices = self.voices_map.get(&id).expect("missing motion voices");
        let tally = voices
            .tally
            .get(&vote)
            .expect("missing motion vote")
            .clone();

        require!(
            voices.votes.insert(&account, &vote).is_none(),
            "The account has already voted on the motion"
        );

        voices.tally.insert(vote, tally + 1);
        self.voices_map.insert(&id, &voices);
    }

    // pub fn execute(&mut self, id: MotionId) {
    //     // TODO Implement cross contract call
    //     // when the motion has an executor
    // }
}
