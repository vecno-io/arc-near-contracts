use crate::*;

// ==== Type IDs ====

impl_string_id!("vote", VoteId, VoteIdParseError);
impl_string_id!("motion", MotionId, MotionIdParseError);

// ==== Vote Info ====

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteInfo {
    pub title: String,
    pub details: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
}

impl VoteInfo {
    pub fn assert_valid(&self) {
        require!(
            self.title.len() > 0,
            "Vote info requires a title is required"
        );
        require!(
            self.title.len() <= 28,
            "Maximum title length is 28 characters"
        );
        if let Some(details) = &self.details {
            require!(
                details.len() <= 128,
                "Maximum details length is 128 characters"
            );
        }

        require!(
            self.reference.is_some() == self.reference_hash.is_some(),
            "Reference hash is required to verify reference integrity"
        );
        if let Some(reference_hash) = &self.reference_hash {
            require!(
                reference_hash.len() == 64,
                "Reference hash has to be hex encoded string (64 bytes)"
            );
        }
    }
}

// ==== Motion Info ====

#[derive(Clone, Default, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MotionInfo {
    pub title: String,
    pub details: String,
    pub issued_at: u64,
    pub starts_at: u64,
    pub expires_at: u64,
    pub executor: Option<AccountId>,
    pub media: Option<String>,
    pub media_hash: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
    pub vote_options: HashMap<VoteId, VoteInfo>,
}

impl MotionInfo {
    pub fn assert_valid(&self) {
        require!(
            self.title.len() > 0,
            "Motion info requires a title is required"
        );
        require!(
            self.title.len() <= 28,
            "Maximum title length is 28 characters"
        );
        require!(
            self.details.len() <= 128,
            "Maximum details length is 128 characters"
        );

        require!(
            self.issued_at <= self.starts_at,
            "Voting can not start before issuing the motion"
        );

        require!(
            self.starts_at < self.expires_at,
            "The motion must expire after the voting started"
        );

        require!(
            self.vote_options.len() > 0,
            "The motion requires at least one option to vote on"
        );

        require!(
            self.media.is_some() == self.media_hash.is_some(),
            "Media hash is required to verify media integrity"
        );
        if let Some(media_hash) = &self.media_hash {
            require!(
                media_hash.len() == 64,
                "Media hash has to be hex encoded string (64 bytes)"
            );
        }
        require!(
            self.reference.is_some() == self.reference_hash.is_some(),
            "Reference hash is required to verify reference integrity"
        );
        if let Some(reference_hash) = &self.reference_hash {
            require!(
                reference_hash.len() == 64,
                "Reference hash has to be hex encoded string (64 bytes)"
            );
        }
    }
}

// ==== Motion State ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MotionState {
    pub info: MotionInfo,
    pub executed: bool,
}

// ==== Motion Voice ====

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MotionVoices {
    pub tally: HashMap<VoteId, u128>,
    pub votes: UnorderedMap<AccountId, VoteId>,
}
