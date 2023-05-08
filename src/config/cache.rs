pub mod domain {
    pub const AUTH: &str = "auth";
}

pub mod id {
    use hextacy::cache::CacheIdentifier;
    #[derive(Debug, PartialEq, Eq)]
    pub enum Auth {
        /// For keeping track of login attempts
        LoginAttempts,
        /// For caching sessions
        Session,
    }

    impl CacheIdentifier for Auth {
        fn id(self) -> &'static str {
            match self {
                Self::LoginAttempts => "login_attempts",
                Self::Session => "session",
            }
        }
    }
}
