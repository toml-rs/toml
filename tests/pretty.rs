#![cfg(feature = "easy")]

use pretty_assertions::assert_eq;

const PRETTY_STD: &str = r#"[example]
array = [
    "item 1",
    "item 2",
]
empty = []
one = ["one"]
oneline = "this has no newlines."
text = """
this is the first line
this is the second line
"""
"#;

#[test]
fn pretty_std() {
    let toml = PRETTY_STD;
    let value: toml_edit::easy::Value = toml_edit::easy::from_str(toml).unwrap();
    let result = toml_edit::easy::to_string_pretty(&value).unwrap();
    println!("EXPECTED:\n{}", toml);
    println!("\nRESULT:\n{}", result);
    assert_eq!(toml, &result);
}

const PRETTY_TRICKY: &str = r##"[example]
f = "\f"
glass = """
Nothing too unusual, except that I can eat glass in:
- Greek: Μπορώ να φάω σπασμένα γυαλιά χωρίς να πάθω τίποτα. 
- Polish: Mogę jeść szkło, i mi nie szkodzi. 
- Hindi: मैं काँच खा सकता हूँ, मुझे उस से कोई पीडा नहीं होती. 
- Japanese: 私はガラスを食べられます。それは私を傷つけません。 
"""
r = "\r"
r_newline = """
\r
"""
single = "this is a single line but has \"\" cuz it\"s tricky"
single_tricky = "single line with ''' in it"
tabs = """
this is pretty standard
\texcept for some \ttabs right here
"""
text = """
this is the first line.
This has a ''' in it and \"\"\" cuz it's tricky yo
Also ' and \" because why not
this is the fourth line
"""
"##;

#[test]
fn pretty_tricky() {
    let toml = PRETTY_TRICKY;
    let value: toml_edit::easy::Value = toml_edit::easy::from_str(toml).unwrap();
    let result = toml_edit::easy::to_string_pretty(&value).unwrap();
    println!("EXPECTED:\n{}", toml);
    println!("\nRESULT:\n{}", result);
    assert_eq!(toml, &result);
}

const PRETTY_TABLE_ARRAY: &str = r##"[[array]]
key = "foo"

[[array]]
key = "bar"

[abc]
doc = "this is a table"

[example]
single = "this is a single line string"
"##;

#[test]
fn pretty_table_array() {
    let toml = PRETTY_TABLE_ARRAY;
    let value: toml_edit::easy::Value = toml_edit::easy::from_str(toml).unwrap();
    let result = toml_edit::easy::to_string_pretty(&value).unwrap();
    println!("EXPECTED:\n{}", toml);
    println!("\nRESULT:\n{}", result);
    assert_eq!(toml, &result);
}

const TABLE_ARRAY: &str = r##"[[array]]
key = "foo"

[[array]]
key = "bar"

[abc]
doc = "this is a table"

[example]
single = "this is a single line string"
"##;

#[test]
fn table_array() {
    let toml = TABLE_ARRAY;
    let value: toml_edit::easy::Value = toml_edit::easy::from_str(toml).unwrap();
    let result = toml_edit::easy::to_string_pretty(&value).unwrap();
    println!("EXPECTED:\n{}", toml);
    println!("\nRESULT:\n{}", result);
    assert_eq!(toml, &result);
}

const PRETTY_EMPTY_TABLE: &str = r#"[example]
"#;

#[test]
fn pretty_empty_table() {
    let toml = PRETTY_EMPTY_TABLE;
    let value: toml_edit::easy::Value = toml_edit::easy::from_str(toml).unwrap();
    let result = toml_edit::easy::to_string_pretty(&value).unwrap();
    println!("EXPECTED:\n{}", toml);
    println!("\nRESULT:\n{}", result);
    assert_eq!(toml, &result);
}

#[test]
fn error_includes_key() {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Package {
        name: String,
        version: String,
        authors: Vec<String>,
        profile: Profile,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Profile {
        dev: Dev,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Dev {
        debug: U32OrBool,
    }

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
    #[serde(untagged, expecting = "expected a boolean or an integer")]
    pub enum U32OrBool {
        U32(u32),
        Bool(bool),
    }

    let raw = r#"name = "foo"
version = "0.0.0"
authors = []

[profile.dev]
debug = true
"#;

    let pkg: Package = toml_edit::de::from_str(raw).unwrap();
    let pretty = toml_edit::ser::to_string_pretty(&pkg).unwrap();
    assert_eq!(raw, pretty,);
}
