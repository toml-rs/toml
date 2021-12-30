#![cfg(feature = "easy")]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use toml_edit::easy::map::Map;
use toml_edit::easy::Value;
use toml_edit::easy::Value::{Array, Float, Integer, Table};

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

macro_rules! equivalent {
    ($literal:expr, $toml:expr,) => {{
        let toml = $toml;
        let literal = $literal;

        // In/out of Value is equivalent
        println!("try_from");
        assert_eq!(t!(Value::try_from(literal.clone())), toml);
        println!("try_into");
        assert_eq!(literal, t!(toml.clone().try_into()));

        // Through a string equivalent
        println!("to_string(literal)");
        assert_eq!(
            t!(toml_edit::easy::to_string_pretty(&literal)),
            toml.to_string()
        );
        println!("to_string(toml)");
        assert_eq!(
            t!(toml_edit::easy::to_string_pretty(&toml)),
            toml.to_string()
        );
        println!("literal, from_str(toml)");
        assert_eq!(literal, t!(toml_edit::easy::from_str(&toml.to_string())));
        println!("toml, from_str(toml)");
        assert_eq!(toml, t!(toml_edit::easy::from_str(&toml.to_string())));
    }};
}

macro_rules! error {
    ($ty:ty, $toml:expr, $msg_parse:expr, $msg_decode:expr) => {{
        println!("attempting parsing");
        match toml_edit::easy::from_str::<$ty>(&$toml.to_string()) {
            Ok(_) => panic!("successful"),
            Err(e) => assert_eq!(e.to_string(), $msg_parse),
        }

        println!("attempting toml decoding");
        match $toml.try_into::<$ty>() {
            Ok(_) => panic!("successful"),
            Err(e) => assert_eq!(e.to_string(), $msg_decode),
        }
    }};
}

macro_rules! map( ($($k:ident: $v:expr),*) => ({
    let mut _m = Map::new();
    $(_m.insert(stringify!($k).to_string(), $v);)*
    _m
}) );

#[test]
fn smoke() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: isize,
    }

    equivalent!(Foo { a: 2 }, Table(map! { a: Integer(2) }),);
}

#[test]
fn smoke_hyphen() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a_b: isize,
    }

    equivalent! {
        Foo { a_b: 2 },
        Table(map! { a_b: Integer(2) }),
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo2 {
        #[serde(rename = "a-b")]
        a_b: isize,
    }

    let mut m = Map::new();
    m.insert("a-b".to_string(), Integer(2));
    equivalent! {
        Foo2 { a_b: 2 },
        Table(m),
    }
}

#[test]
fn nested() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: isize,
        b: Bar,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Bar {
        a: String,
    }

    equivalent! {
        Foo { a: 2, b: Bar { a: "test".to_string() } },
        Table(map! {
            a: Integer(2),
            b: Table(map! {
                a: Value::String("test".to_string())
            })
        }),
    }
}

#[test]
fn array() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: Vec<isize>,
    }

    equivalent! {
        Foo { a: vec![1, 2, 3, 4] },
        Table(map! {
            a: Array(vec![
                Integer(1),
                Integer(2),
                Integer(3),
                Integer(4)
            ])
        }),
    };
}

#[test]
fn inner_structs_with_options() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: Option<Box<Foo>>,
        b: Bar,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Bar {
        a: String,
        b: f64,
    }

    equivalent! {
        Foo {
            a: Some(Box::new(Foo {
                a: None,
                b: Bar { a: "foo".to_string(), b: 4.5 },
            })),
            b: Bar { a: "bar".to_string(), b: 1.0 },
        },
        Table(map! {
            a: Table(map! {
                b: Table(map! {
                    a: Value::String("foo".to_string()),
                    b: Float(4.5)
                })
            }),
            b: Table(map! {
                a: Value::String("bar".to_string()),
                b: Float(1.0)
            })
        }),
    }
}

#[test]
fn table_array() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: Vec<Bar>,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Bar {
        a: isize,
    }

    equivalent! {
        Foo { a: vec![Bar { a: 1 }, Bar { a: 2 }] },
        Table(map! {
            a: Array(vec![
                Table(map!{ a: Integer(1) }),
                Table(map!{ a: Integer(2) }),
            ])
        }),
    }
}

#[test]
fn type_errors() {
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Foo {
        bar: isize,
    }

    error! {
        Foo,
        Table(map! {
            bar: Value::String("a".to_string())
        }),
        "invalid type: string \"a\", expected isize for key `bar`",
        "invalid type: string \"a\", expected isize for key `bar`"
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Bar {
        foo: Foo,
    }

    error! {
        Bar,
        Table(map! {
            foo: Table(map! {
                bar: Value::String("a".to_string())
            })
        }),
        "invalid type: string \"a\", expected isize for key `foo.bar`",
        "invalid type: string \"a\", expected isize for key `foo.bar`"
    }
}

#[test]
fn missing_errors() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Foo {
        bar: isize,
    }

    error! {
        Foo,
        Table(map! { }),
        "missing field `bar`",
        "missing field `bar`"
    }
}

#[test]
fn parse_enum() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: E,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    #[serde(untagged)]
    enum E {
        Bar(isize),
        Baz(String),
        Last(Foo2),
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo2 {
        test: String,
    }

    equivalent! {
        Foo { a: E::Bar(10) },
        Table(map! { a: Integer(10) }),
    }

    equivalent! {
        Foo { a: E::Baz("foo".to_string()) },
        Table(map! { a: Value::String("foo".to_string()) }),
    }

    equivalent! {
        Foo { a: E::Last(Foo2 { test: "test".to_string() }) },
        Table(map! { a: Table(map! { test: Value::String("test".to_string()) }) }),
    }
}

#[test]
fn parse_enum_string() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: Sort,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    #[serde(rename_all = "lowercase")]
    enum Sort {
        Asc,
        Desc,
    }

    equivalent! {
        Foo { a: Sort::Desc },
        Table(map! { a: Value::String("desc".to_string()) }),
    }
}

#[test]
fn empty_arrays() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: Vec<Bar>,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Bar;

    equivalent! {
        Foo { a: vec![] },
        Table(map! {a: Array(Vec::new())}),
    }
}

#[test]
fn empty_arrays2() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Foo {
        a: Option<Vec<Bar>>,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Bar;

    equivalent! {
        Foo { a: None },
        Table(map! {}),
    }

    equivalent! {
        Foo { a: Some(vec![]) },
        Table(map! { a: Array(vec![]) }),
    }
}

#[test]
fn extra_keys() {
    #[derive(Serialize, Deserialize)]
    struct Foo {
        a: isize,
    }

    let toml = Table(map! { a: Integer(2), b: Integer(2) });
    assert!(toml.clone().try_into::<Foo>().is_ok());
    assert!(toml_edit::de::from_str::<Foo>(&toml.to_string()).is_ok());
}

#[test]
fn newtypes() {
    #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
    struct A {
        b: B,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
    struct B(u32);

    equivalent! {
        A { b: B(2) },
        Table(map! { b: Integer(2) }),
    }
}

#[test]
fn newtypes2() {
    #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
    struct A {
        b: B,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
    struct B(Option<C>);

    #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
    struct C {
        x: u32,
        y: u32,
        z: u32,
    }

    equivalent! {
        A { b: B(Some(C { x: 0, y: 1, z: 2 })) },
        Table(map! {
            b: Table(map! {
                x: Integer(0),
                y: Integer(1),
                z: Integer(2)
            })
        }),
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
struct CanBeEmpty {
    a: Option<String>,
    b: Option<String>,
}

#[test]
fn table_structs_empty() {
    let text = "bar = {}\nbaz = {}\nbazv = { a = \"foo\" }\nfoo = {}\n";
    let value: BTreeMap<String, CanBeEmpty> = toml_edit::de::from_str(text).unwrap();
    let mut expected: BTreeMap<String, CanBeEmpty> = BTreeMap::new();
    expected.insert("bar".to_string(), CanBeEmpty::default());
    expected.insert("baz".to_string(), CanBeEmpty::default());
    expected.insert(
        "bazv".to_string(),
        CanBeEmpty {
            a: Some("foo".to_string()),
            b: None,
        },
    );
    expected.insert("foo".to_string(), CanBeEmpty::default());
    assert_eq!(value, expected);
    assert_eq!(toml_edit::ser::to_string(&value).unwrap(), text);
}

#[test]
fn fixed_size_array() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Entity {
        pos: [i32; 2],
    }

    equivalent! {
        Entity { pos: [1, 2] },
        Table(map! {
            pos: Array(vec![
                Integer(1),
                Integer(2),
            ])
        }),
    }
}

#[test]
fn homogeneous_tuple() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    struct Collection {
        elems: (i64, i64, i64),
    }

    equivalent! {
        Collection { elems: (0, 1, 2) },
        Table(map! {
            elems: Array(vec![
                Integer(0),
                Integer(1),
                Integer(2),
            ])
        }),
    }
}

#[test]
fn json_interoperability() {
    #[derive(Serialize, Deserialize)]
    struct Foo {
        any: toml_edit::easy::Value,
    }

    let _foo: Foo = serde_json::from_str(
        r#"
        {"any":1}
    "#,
    )
    .unwrap();
}

#[test]
fn error_includes_key() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Package {
        name: String,
        version: String,
        authors: Vec<String>,
        profile: Profile,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Profile {
        dev: Dev,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Dev {
        debug: U32OrBool,
    }

    #[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
    #[serde(untagged, expecting = "expected a boolean or an integer")]
    pub enum U32OrBool {
        U32(u32),
        Bool(bool),
    }

    let res: Result<Package, _> = toml_edit::de::from_str(
        r#"
[package]
name = "foo"
version = "0.0.0"
authors = []

[profile.dev]
debug = 'a'
"#,
    );
    let err = res.unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected a boolean or an integer for key `profile.dev.debug`"
    );

    let res: Result<Package, _> = toml_edit::de::from_str(
        r#"
[package]
name = "foo"
version = "0.0.0"
authors = []

[profile]
dev = { debug = 'a' }
"#,
    );
    let err = res.unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected a boolean or an integer for key `profile.dev.debug`"
    );
}
