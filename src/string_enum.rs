macro_rules! string_extensible_enum {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $const_name:ident = $value:literal;
            )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $vis struct $name(std::borrow::Cow<'static, str>);

        impl $name {
            #[must_use]
            pub const fn from_static(value: &'static str) -> Self {
                Self(std::borrow::Cow::Borrowed(value))
            }

            #[must_use]
            pub fn new(value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
                Self(value.into())
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            $(
                $(#[$variant_meta])*
                pub const $const_name: Self = Self(std::borrow::Cow::Borrowed($value));
            )*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(std::borrow::Cow::Owned(value.to_string()))
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(std::borrow::Cow::Owned(value))
            }
        }

        impl From<&String> for $name {
            fn from(value: &String) -> Self {
                Self(std::borrow::Cow::Owned(value.clone()))
            }
        }

        impl PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }
    };
}

pub(crate) use string_extensible_enum;
