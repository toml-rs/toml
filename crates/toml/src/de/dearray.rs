use serde_spanned::Spanned;

use crate::de::DeValue;

/// Type representing a TOML array, payload of the `DeValue::Array` variant
pub type DeArray<'i> = Vec<Spanned<DeValue<'i>>>;
