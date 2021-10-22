macro_rules! impl_wrapper {
    ( $name:ident, $doc:expr$(,)? ) => {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[doc = $doc]
        pub struct $name(String);

        impl $name {
            /// Create a new id.
            pub fn new<T>(id: T) -> Self
            where
                T: Into<String>,
            {
                $name(id.into())
            }

            /// The value of the id.
            #[must_use]
            pub fn value(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(f: String) -> Self {
                Self(f)
            }
        }

        impl From<$name> for String {
            fn from(f: $name) -> Self {
                f.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
