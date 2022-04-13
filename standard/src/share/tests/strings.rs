// ==== Type IDs ====

mod guild_id {
    use crate::share::GuildId;
    use crate::*;

    impl_string_id_tests!("guild", GuildId);
}

mod motion_id {
    use crate::share::MotionId;
    use crate::*;

    impl_string_id_tests!("motion", MotionId);
}

mod token_id {
    use crate::share::TokenId;
    use crate::*;

    impl_string_id_tests!("token", TokenId);
}

mod vote_id {
    use crate::share::VoteId;
    use crate::*;

    impl_string_id_tests!("vote", VoteId);
}
