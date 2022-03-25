use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guild {}

impl Guild {
	pub fn new() -> Self {
		Self {}
	}

	pub fn assert_valid(&self) {
        require!(true, "must be true");
	}
}

