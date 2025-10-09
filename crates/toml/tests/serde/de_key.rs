use std::collections::BTreeMap;

use serde::Deserialize;
use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

#[derive(Debug, Deserialize, PartialEq)]
struct Document<K: Ord> {
    map: Map<K>,
}

type Map<K> = BTreeMap<K, String>;

mod string_key {
    use super::*;

    type Map = super::Map<String>;
    type Document = super::Document<String>;

    #[test]
    fn from_str() {
        let input = "key = 'value'";
        let expected = str![[r#"
{
    "key": "value",
}

"#]];
        let result = crate::from_str::<Map>(input);
        assert_data_eq!(result.unwrap().to_debug(), expected);
    }

    #[test]
    fn value_from_inline_table() {
        let input = "{ key = 'value' }";
        let expected = str![[r#"
{
    "key": "value",
}

"#]];
        let result = crate::value_from_str::<Map>(input);
        assert_data_eq!(result.unwrap().to_debug(), expected);
    }

    #[test]
    fn from_inline_table() {
        let input = "map = { key = 'value' }";
        let expected = str![[r#"
Document {
    map: {
        "key": "value",
    },
}

"#]];
        let result = crate::from_str::<Document>(input);
        assert_data_eq!(result.unwrap().to_debug(), expected);
    }

    #[test]
    fn from_std_table() {
        let input = "[map]
key = 'value'";
        let expected = str![[r#"
Document {
    map: {
        "key": "value",
    },
}

"#]];
        let result = crate::from_str::<Document>(input);
        assert_data_eq!(result.unwrap().to_debug(), expected);
    }
}

mod variant_key {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    enum Keys {
        #[allow(non_camel_case_types)]
        key,
    }

    type Map = super::Map<Keys>;
    type Document = super::Document<Keys>;

    #[test]
    fn from_str() {
        let input = "key = 'value'";
        let expected = str![[r#"
{
    key: "value",
}

"#]];
        let result = crate::from_str::<Map>(input);
        assert_data_eq!(result.unwrap().to_debug(), expected);
    }

    #[test]
    fn value_from_inline_table() {
        let input = "{ key = 'value' }";
        let expected = str![[r#"
{
    key: "value",
}

"#]];
        let result = crate::value_from_str::<Map>(input);
        assert_data_eq!(result.unwrap().to_debug(), expected);
    }

    #[test]
    fn from_inline_table() {
        let input = "map = { key = 'value' }";
        let expected = str![[r#"
Document {
    map: {
        key: "value",
    },
}

"#]];
        let result = crate::from_str::<Document>(input);
        assert_data_eq!(result.unwrap().to_debug(), expected);
    }

    #[test]
    fn from_std_table() {
        let input = "[map]
key = 'value'";
        let expected = str![[r#"
Document {
    map: {
        key: "value",
    },
}

"#]];
        let result = crate::from_str::<Document>(input);
        assert_data_eq!(result.unwrap().to_debug(), expected);
    }
}
