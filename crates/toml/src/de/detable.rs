use crate::de::DeString;
use crate::de::DeValue;
use crate::map::Map;

/// Type representing a TOML table, payload of the `Value::Table` variant.
///
/// By default it entries are stored in
/// [lexicographic order](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str)
/// of the keys. Enable the `preserve_order` feature to store entries in the order they appear in
/// the source file.
pub type DeTable<'i> = Map<DeString<'i>, DeValue<'i>>;
