// ==== Is Owned ====

#[macro_export]
macro_rules! impl_is_owned {
    ($struct: ident, $key: ident, $store: ident) => {
        impl $struct {
            pub fn add_to_owner(&mut self, owner: AccountId, id: &$key) {
                let mut owner_set = self.list_per_owner.get(&owner).unwrap_or_else(|| {
                    UnorderedSet::new(
                        $crate::share::StorageKey::$store {
                            owner_key: owner.clone(),
                        }
                        .try_to_vec()
                        .unwrap(),
                    )
                });
                owner_set.insert(id);
                self.list_per_owner.insert(&owner, &owner_set);
            }

            pub fn remove_from_owner(&mut self, owner: AccountId, id: &$key) {
                let mut owner_set = self
                    .list_per_owner
                    .get(&owner)
                    .expect("owner set not found");
                if owner_set.len() == 1 {
                    self.list_per_owner.remove(&owner);
                } else {
                    owner_set.remove(id);
                    self.list_per_owner.insert(&owner, &owner_set);
                }
            }
        }
    };
}

// ==== String ID ====

#[macro_export]
macro_rules! impl_string_id {
    ($id_name: tt, $string_id: ident, $string_id_error: ident) => {
        #[derive(
            Hash,
            Clone,
            Debug,
            Eq,
            Ord,
            PartialEq,
            PartialOrd,
            Serialize,
            Deserialize,
            BorshSerialize,
            BorshDeserialize,
        )]
        #[serde(crate = "near_sdk::serde")]
        pub struct $string_id(String);

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $string_id_error {}

        impl $string_id {
            /// Returns reference to the ID bytes.
            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }
            /// Returns reference to the ID string.
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl AsRef<str> for $string_id {
            fn as_ref(&self) -> &str {
                self.0.as_str()
            }
        }

        impl From<String> for $string_id {
            fn from(item: String) -> Self {
                require!(
                    item.len() > 0 && item.len() <= 32,
                    format!("The string is not a valid ID")
                );
                $string_id { 0: item }
            }
        }

        impl fmt::Display for $string_id {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::str::FromStr for $string_id {
            type Err = $string_id_error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if value.len() > 0 && value.len() <= 32 {
                    Ok(Self(value.to_string()))
                } else {
                    Err($string_id_error {})
                }
            }
        }

        impl fmt::Display for $string_id_error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "the ID is invalid")
            }
        }

        impl std::error::Error for $string_id_error {}
    };
}

#[macro_export]
macro_rules! impl_string_id_tests {
    ($id_name: tt, $string_id: ident) => {
        const ID_MIN: &str = "1";
        const ID_MAX: &str = "123456789:123456789:123456789:12";

        const ID_ERR_NONE: &str = "";
        const ID_ERR_LONG: &str = "123456789:123456789:123456789:123";

        #[test]
        fn id_as_bytes() {
            let id = ID_MIN.parse::<$string_id>().unwrap();
            assert_eq!(id.as_bytes(), ID_MIN.as_bytes());
            let id = ID_MAX.parse::<$string_id>().unwrap();
            assert_eq!(id.as_bytes(), ID_MAX.as_bytes());
        }

        #[test]
        fn id_as_str() {
            let id = ID_MIN.parse::<$string_id>().unwrap();
            assert_eq!(id.as_str(), ID_MIN);
            let id = ID_MAX.parse::<$string_id>().unwrap();
            assert_eq!(id.as_str(), ID_MAX);
        }

        #[test]
        fn id_as_ref() {
            let id = ID_MIN.parse::<$string_id>().unwrap();
            assert_eq!(id.as_ref(), &ID_MIN.to_string());
            let id = ID_MAX.parse::<$string_id>().unwrap();
            assert_eq!(id.as_ref(), &ID_MAX.to_string());
        }

        #[test]
        fn id_from_str() {
            let id: $string_id = ID_MIN.to_string().into();
            assert_eq!(id.as_bytes(), ID_MIN.as_bytes());
            let id: $string_id = ID_MAX.to_string().into();
            assert_eq!(id.as_bytes(), ID_MAX.as_bytes());
        }

        #[test]
        #[should_panic(expected = "The string is not a valid ID")]
        fn id_from_str_panic_none() {
            let _id: $string_id = ID_ERR_NONE.to_string().into();
        }

        #[test]
        #[should_panic(expected = "The string is not a valid ID")]
        fn id_from_str_panic_long() {
            let _id: $string_id = ID_ERR_LONG.to_string().into();
        }

        #[test]
        #[should_panic(expected = "From<String>: the ID is invalid")]
        fn id_from_str_error_none() {
            let res = ID_ERR_NONE.parse::<$string_id>();
            let _id = match res {
                Ok(id) => id,
                Err(error) => panic!("From<String>: {}", error.to_string()),
            };
        }

        #[test]
        #[should_panic(expected = "From<String>: the ID is invalid")]
        fn id_from_str_error_long() {
            let res = ID_ERR_LONG.parse::<$string_id>();
            let _id = match res {
                Ok(id) => id,
                Err(error) => panic!("From<String>: {}", error.to_string()),
            };
        }
    };
}
