#![doc(hidden)]
#![deprecated(since = "0.18.0", note = "Replaced with `toml::Value`")]
#![allow(deprecated)]

mod datetime;

#[doc(hidden)]
pub mod macros;
pub mod map;
pub mod value;

pub use crate::de;
pub use crate::ser;
pub use crate::toml;
#[doc(no_inline)]
pub use de::{from_document, from_slice, from_str, Deserializer};
#[doc(no_inline)]
pub use ser::{to_document, to_string, to_string_pretty, to_vec, ValueSerializer};
pub use serde_spanned::Spanned;
#[doc(no_inline)]
pub use value::Value;
