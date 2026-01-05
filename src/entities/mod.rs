macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                serializer.serialize_str(match *self {
                    $( $name::$variant => $str, )*
                })
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "a string for {}", stringify!($name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                                &format!("unknown {} variant: {}", stringify!($name), value)
                            ), &self)),
                        }
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $str.to_string(), )*
                }
            }
        }
    }
}

macro_rules! format_params {
    ($variant:ident : String) => {
        $variant
    };
    ($variant:ident : i64) => {
        format!("{}", $variant)
    };
    ($variant:ident : $type_id:ident) => {
        String::from($variant)
    };
}

macro_rules! query_params {
    ($name:ident { $($variant:ident($str:expr,$type_id:ident), )* }) => {
        #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
        pub struct $name {
            $(pub $variant: Option<$type_id>,)*
        }
        impl $name {
            pub(crate) fn queries(self) -> Vec<(String, String)> {
                let mut result = vec![];
                $(
                if let Some($variant) = self.$variant {
                    result.push(($str.to_owned(), format_params!($variant: $type_id)));
                }
                )*
                result
            }
            $(
            pub fn $variant(mut self, $variant: $type_id) -> Self {
                self.$variant = Some($variant);
                self
            }
            )*
        }
    };
}

mod app;
mod bundle_id;
mod certificate;
mod common;
mod device;
mod profile;
mod user;

pub use app::*;
pub use bundle_id::*;
pub use certificate::*;
pub use common::*;
pub use device::*;
pub use profile::*;
pub use user::*;
