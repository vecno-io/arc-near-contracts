use crate::*;

pub mod data;

pub use self::data::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Guilds {}

impl Guilds {
	pub fn new() -> Self {
		Self {}
	}

	pub fn assert_valid(&self) {
        require!(true, "must be true");
	}
}

#[cfg(test)]
mod tests {
    mod guild;
    mod guilds;
}