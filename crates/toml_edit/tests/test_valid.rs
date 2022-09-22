use pretty_assertions::assert_eq;
use serde_json::Map as JsonMap;
use serde_json::Value as Json;

use toml_edit::{Document, Item, Iter, Value};

fn pair_to_json((key, value): (&str, Item)) -> (String, Json) {
    fn typed_json(s: &str, json: Json) -> Json {
        let mut map = JsonMap::new();
        map.insert("type".to_owned(), Json::String(s.into()));
        map.insert("value".to_owned(), json);
        Json::Object(map)
    }
    fn value_to_json(value: &Value) -> Json {
        match *value {
            Value::String(ref s) => typed_json("string", Json::String(s.value().clone())),
            Value::Integer(ref i) => typed_json("integer", Json::String(format!("{}", i.value()))),
            Value::Float(ref f) => typed_json("float", Json::String(format!("{}", f.value()))),
            Value::Boolean(ref b) => typed_json("bool", Json::String(b.to_repr().as_raw().into())),
            Value::Datetime(ref d) => {
                typed_json("datetime", Json::String(d.to_repr().as_raw().into()))
            }
            Value::Array(ref a) => {
                let json = Json::Array(a.iter().map(value_to_json).collect::<Vec<_>>());
                typed_json("array", json)
            }
            Value::InlineTable(ref t) => {
                to_json(Box::new(t.iter().map(|(k, v)| (k, Item::Value(v.clone())))))
            }
        }
    }
    let json = match value {
        Item::Value(ref v) => value_to_json(v),
        Item::ArrayOfTables(ref arr) => Json::Array(
            arr.iter()
                .map(|t| to_json(iter_to_owned(t.iter())))
                .collect::<Vec<_>>(),
        ),
        Item::Table(ref table) => to_json(iter_to_owned(table.iter())),
        Item::None => Json::Null,
    };
    (key.to_owned(), json)
}

fn iter_to_owned(iter: Iter<'_>) -> OwnedIter<'_> {
    Box::new(iter.map(|(k, v)| (k, v.clone())))
}

type OwnedIter<'s> = Box<dyn Iterator<Item = (&'s str, Item)> + 's>;

fn to_json(iter: OwnedIter<'_>) -> Json {
    Json::Object(iter.map(pair_to_json).collect())
}

fn run(json: &str, toml: &str) {
    let doc = toml.parse::<Document>();
    assert!(doc.is_ok());
    let doc = doc.unwrap();

    let json: Json = serde_json::from_str(json).unwrap();
    let toml_json = to_json(iter_to_owned(doc.iter()));
    // compare structure with jsons
    assert_eq!(json, toml_json);

    // check round-trip equality
    let toml = doc.to_string();
    let doc = toml.parse::<Document>();
    assert!(doc.is_ok());
    let doc = doc.unwrap();
    assert_eq!(doc.to_string(), toml);
}

macro_rules! t(
    ($name:ident, $json:expr, $toml:expr) => (
        #[test]
        fn $name() {
            run(include_str!($json),
                include_str!($toml));
        }
    )
);

#[test]
fn test_key_unification() {
    let toml = r#"
[a]
[a.'b'.c]
[a."b".c.e]
[a.b.c.d]
"#;
    let expected = r#"
[a]
[a.'b'.c]
[a.'b'.c.e]
[a.'b'.c.d]
"#;
    let doc = toml.parse::<Document>();
    assert!(doc.is_ok());
    let doc = doc.unwrap();

    assert_eq!(doc.to_string(), expected);
}

t!(
    test_arrays,
    "fixtures/valid/arrays.json",
    "fixtures/valid/arrays.toml"
);
t!(
    test_comment_at_eof2,
    "fixtures/valid/comments-at-eof2.json",
    "fixtures/valid/comments-at-eof2.toml"
);
t!(
    test_comments_everywhere,
    "fixtures/valid/comments-everywhere.json",
    "fixtures/valid/comments-everywhere.toml"
);
t!(
    test_datetime,
    "fixtures/valid/datetime.json",
    "fixtures/valid/datetime.toml"
);
t!(
    test_float_exponent,
    "fixtures/valid/float-exponent.json",
    "fixtures/valid/float-exponent.toml"
);
t!(
    test_inline_table,
    "fixtures/valid/inline-table.json",
    "fixtures/valid/inline-table.toml"
);
t!(
    test_integer_underscore,
    "fixtures/valid/integer-underscore.json",
    "fixtures/valid/integer-underscore.toml"
);
t!(
    test_long_integer,
    "fixtures/valid/long-integer.json",
    "fixtures/valid/long-integer.toml"
);
t!(
    test_multiline_string,
    "fixtures/valid/multiline-string.json",
    "fixtures/valid/multiline-string.toml"
);
t!(
    test_raw_multiline_string,
    "fixtures/valid/raw-multiline-string.json",
    "fixtures/valid/raw-multiline-string.toml"
);
t!(
    test_string_escapes,
    "fixtures/valid/string-escapes.json",
    "fixtures/valid/string-escapes.toml"
);
t!(
    test_table_no_eol,
    "fixtures/valid/table-no-eol.json",
    "fixtures/valid/table-no-eol.toml"
);
t!(
    test_windows_path,
    "fixtures/valid/windows-path.json",
    "fixtures/valid/windows-path.toml"
);
