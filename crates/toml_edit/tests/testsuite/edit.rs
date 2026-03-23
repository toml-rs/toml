use std::iter::FromIterator;

use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;
use toml_edit::{DocumentMut, Item, Key, Table, Value, array, table, value};

macro_rules! parse_key {
    ($s:expr) => {{
        let key = $s.parse::<Key>();
        assert!(key.is_ok());
        key.unwrap()
    }};
}

macro_rules! as_table {
    ($e:ident) => {{
        assert!($e.is_table());
        $e.as_table_mut().unwrap()
    }};
}

struct Test {
    doc: DocumentMut,
}

fn given(input: &str) -> Test {
    let doc = input.parse::<DocumentMut>();
    assert!(doc.is_ok());
    Test { doc: doc.unwrap() }
}

impl Test {
    fn running<F>(&mut self, func: F) -> &mut Self
    where
        F: Fn(&mut Table),
    {
        {
            let root = self.doc.as_table_mut();
            func(root);
        }
        self
    }
    fn running_on_doc<F>(&mut self, func: F) -> &mut Self
    where
        F: Fn(&mut DocumentMut),
    {
        {
            func(&mut self.doc);
        }
        self
    }

    #[track_caller]
    fn produces_display(&self, expected: snapbox::data::Inline) -> &Self {
        assert_data_eq!(self.doc.to_string(), expected.raw());
        self
    }
}

#[test]
fn assign_whitespace() {
    let input = r#"
 # top comment
 [ grandparent . parent ]  # table comment
 # table-value sep comment
 key . child = 'value' # key comment
 key . inline-empty = { }  # empty inline-table comment
 key . inline = { inline . child = 'inline-value' }  # inline-table comment
 key . array-empty = [  # inside empty array comment
 ] # after empty array comment
 key . array = [ # start of array comment
   'one' , # one comment
   'two' , # two comment
] # after array comment
 # table-table sep comment
 [ another . table ]  # table comment
 # final comment
"#;
    assert_data_eq!(
        input.parse::<DocumentMut>().unwrap().to_debug(),
        str![[r#"
DocumentMut {
    root: Table(
        Table {
            decor: Decor {
                prefix: "default",
                suffix: "default",
            },
            implicit: false,
            dotted: false,
            doc_position: None,
            span: None,
            items: {
                Key {
                    key: "grandparent",
                    repr: Some(
                        "grandparent",
                    ),
                    leaf_decor: Decor {
                        prefix: "default",
                        suffix: "default",
                    },
                    dotted_decor: Decor {
                        prefix: empty,
                        suffix: " ",
                    },
                }: Table(
                    Table {
                        decor: Decor {
                            prefix: "default",
                            suffix: "default",
                        },
                        implicit: true,
                        dotted: false,
                        doc_position: None,
                        span: None,
                        items: {
                            Key {
                                key: "parent",
                                repr: Some(
                                    "parent",
                                ),
                                leaf_decor: Decor {
                                    prefix: " ",
                                    suffix: " ",
                                },
                                dotted_decor: Decor {
                                    prefix: " ",
                                    suffix: empty,
                                },
                            }: Table(
                                Table {
                                    decor: Decor {
                                        prefix: "\n # top comment\n ",
                                        suffix: "  # table comment",
                                    },
                                    implicit: false,
                                    dotted: false,
                                    doc_position: Some(
                                        1,
                                    ),
                                    span: None,
                                    items: {
                                        Key {
                                            key: "key",
                                            repr: Some(
                                                "key",
                                            ),
                                            leaf_decor: Decor {
                                                prefix: "default",
                                                suffix: "default",
                                            },
                                            dotted_decor: Decor {
                                                prefix: empty,
                                                suffix: " ",
                                            },
                                        }: Table(
                                            Table {
                                                decor: Decor {
                                                    prefix: "default",
                                                    suffix: "default",
                                                },
                                                implicit: true,
                                                dotted: true,
                                                doc_position: None,
                                                span: None,
                                                items: {
                                                    Key {
                                                        key: "child",
                                                        repr: Some(
                                                            "child",
                                                        ),
                                                        leaf_decor: Decor {
                                                            prefix: " # table-value sep comment\n ",
                                                            suffix: " ",
                                                        },
                                                        dotted_decor: Decor {
                                                            prefix: " ",
                                                            suffix: empty,
                                                        },
                                                    }: Value(
                                                        String(
                                                            Formatted {
                                                                value: "value",
                                                                repr: "'value'",
                                                                decor: Decor {
                                                                    prefix: " ",
                                                                    suffix: " # key comment",
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Key {
                                                        key: "inline-empty",
                                                        repr: Some(
                                                            "inline-empty",
                                                        ),
                                                        leaf_decor: Decor {
                                                            prefix: " ",
                                                            suffix: " ",
                                                        },
                                                        dotted_decor: Decor {
                                                            prefix: " ",
                                                            suffix: empty,
                                                        },
                                                    }: Value(
                                                        InlineTable(
                                                            InlineTable {
                                                                trailing: " ",
                                                                trailing_comma: false,
                                                                implicit: false,
                                                                decor: Decor {
                                                                    prefix: " ",
                                                                    suffix: "  # empty inline-table comment",
                                                                },
                                                                span: None,
                                                                dotted: false,
                                                                items: {},
                                                            },
                                                        ),
                                                    ),
                                                    Key {
                                                        key: "inline",
                                                        repr: Some(
                                                            "inline",
                                                        ),
                                                        leaf_decor: Decor {
                                                            prefix: " ",
                                                            suffix: " ",
                                                        },
                                                        dotted_decor: Decor {
                                                            prefix: " ",
                                                            suffix: empty,
                                                        },
                                                    }: Value(
                                                        InlineTable(
                                                            InlineTable {
                                                                trailing: empty,
                                                                trailing_comma: false,
                                                                implicit: false,
                                                                decor: Decor {
                                                                    prefix: " ",
                                                                    suffix: "  # inline-table comment",
                                                                },
                                                                span: None,
                                                                dotted: false,
                                                                items: {
                                                                    Key {
                                                                        key: "inline",
                                                                        repr: Some(
                                                                            "inline",
                                                                        ),
                                                                        leaf_decor: Decor {
                                                                            prefix: "default",
                                                                            suffix: "default",
                                                                        },
                                                                        dotted_decor: Decor {
                                                                            prefix: empty,
                                                                            suffix: " ",
                                                                        },
                                                                    }: Value(
                                                                        InlineTable(
                                                                            InlineTable {
                                                                                trailing: empty,
                                                                                trailing_comma: false,
                                                                                implicit: true,
                                                                                decor: Decor {
                                                                                    prefix: "default",
                                                                                    suffix: "default",
                                                                                },
                                                                                span: None,
                                                                                dotted: true,
                                                                                items: {
                                                                                    Key {
                                                                                        key: "child",
                                                                                        repr: Some(
                                                                                            "child",
                                                                                        ),
                                                                                        leaf_decor: Decor {
                                                                                            prefix: " ",
                                                                                            suffix: " ",
                                                                                        },
                                                                                        dotted_decor: Decor {
                                                                                            prefix: " ",
                                                                                            suffix: empty,
                                                                                        },
                                                                                    }: Value(
                                                                                        String(
                                                                                            Formatted {
                                                                                                value: "inline-value",
                                                                                                repr: "'inline-value'",
                                                                                                decor: Decor {
                                                                                                    prefix: " ",
                                                                                                    suffix: " ",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Key {
                                                        key: "array-empty",
                                                        repr: Some(
                                                            "array-empty",
                                                        ),
                                                        leaf_decor: Decor {
                                                            prefix: " ",
                                                            suffix: " ",
                                                        },
                                                        dotted_decor: Decor {
                                                            prefix: " ",
                                                            suffix: empty,
                                                        },
                                                    }: Value(
                                                        Array(
                                                            Array {
                                                                trailing: "  # inside empty array comment\n ",
                                                                trailing_comma: false,
                                                                decor: Decor {
                                                                    prefix: " ",
                                                                    suffix: " # after empty array comment",
                                                                },
                                                                span: None,
                                                                values: [],
                                                            },
                                                        ),
                                                    ),
                                                    Key {
                                                        key: "array",
                                                        repr: Some(
                                                            "array",
                                                        ),
                                                        leaf_decor: Decor {
                                                            prefix: " ",
                                                            suffix: " ",
                                                        },
                                                        dotted_decor: Decor {
                                                            prefix: " ",
                                                            suffix: empty,
                                                        },
                                                    }: Value(
                                                        Array(
                                                            Array {
                                                                trailing: " # two comment\n",
                                                                trailing_comma: true,
                                                                decor: Decor {
                                                                    prefix: " ",
                                                                    suffix: " # after array comment",
                                                                },
                                                                span: None,
                                                                values: [
                                                                    Value(
                                                                        String(
                                                                            Formatted {
                                                                                value: "one",
                                                                                repr: "'one'",
                                                                                decor: Decor {
                                                                                    prefix: " # start of array comment\n   ",
                                                                                    suffix: " ",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                    Value(
                                                                        String(
                                                                            Formatted {
                                                                                value: "two",
                                                                                repr: "'two'",
                                                                                decor: Decor {
                                                                                    prefix: " # one comment\n   ",
                                                                                    suffix: " ",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                ],
                                                            },
                                                        ),
                                                    ),
                                                },
                                            },
                                        ),
                                    },
                                },
                            ),
                        },
                    },
                ),
                Key {
                    key: "another",
                    repr: Some(
                        "another",
                    ),
                    leaf_decor: Decor {
                        prefix: "default",
                        suffix: "default",
                    },
                    dotted_decor: Decor {
                        prefix: empty,
                        suffix: " ",
                    },
                }: Table(
                    Table {
                        decor: Decor {
                            prefix: "default",
                            suffix: "default",
                        },
                        implicit: true,
                        dotted: false,
                        doc_position: None,
                        span: None,
                        items: {
                            Key {
                                key: "table",
                                repr: Some(
                                    "table",
                                ),
                                leaf_decor: Decor {
                                    prefix: " ",
                                    suffix: " ",
                                },
                                dotted_decor: Decor {
                                    prefix: " ",
                                    suffix: empty,
                                },
                            }: Table(
                                Table {
                                    decor: Decor {
                                        prefix: " # table-table sep comment\n ",
                                        suffix: "  # table comment",
                                    },
                                    implicit: false,
                                    dotted: false,
                                    doc_position: Some(
                                        2,
                                    ),
                                    span: None,
                                    items: {},
                                },
                            ),
                        },
                    },
                ),
            },
        },
    ),
    trailing: " # final comment\n",
}

"#]]
        .raw()
    );
}

#[test]
fn test_add_root_decor() {
    given(
        r#"[package]
name = "hello"
version = "1.0.0"

[[bin]]
name = "world"
path = "src/bin/world/main.rs"

[dependencies]
nom = "4.0" # future is here

[[bin]]
name = "delete me please"
path = "src/bin/dmp/main.rs""#,
    )
    .running_on_doc(|document| {
        document.decor_mut().set_prefix("# Some Header\n\n");
        document.decor_mut().set_suffix("# Some Footer");
        document.set_trailing("\n\ntrailing...");
    })
    .produces_display(str![[r#"
# Some Header

[package]
name = "hello"
version = "1.0.0"

[[bin]]
name = "world"
path = "src/bin/world/main.rs"

[dependencies]
nom = "4.0" # future is here

[[bin]]
name = "delete me please"
path = "src/bin/dmp/main.rs"
# Some Footer

trailing...
"#]]);
}

/// Tests that default decor is None for both suffix and prefix and that this means empty strings
#[test]
fn test_no_root_decor() {
    given(
        r#"[package]
name = "hello"
version = "1.0.0"

[[bin]]
name = "world"
path = "src/bin/world/main.rs"

[dependencies]
nom = "4.0" # future is here

[[bin]]
name = "delete me please"
path = "src/bin/dmp/main.rs""#,
    )
    .running_on_doc(|document| {
        assert!(document.decor().prefix().is_none());
        assert!(document.decor().suffix().is_none());
        document.set_trailing("\n\ntrailing...");
    })
    .produces_display(str![[r#"
[package]
name = "hello"
version = "1.0.0"

[[bin]]
name = "world"
path = "src/bin/world/main.rs"

[dependencies]
nom = "4.0" # future is here

[[bin]]
name = "delete me please"
path = "src/bin/dmp/main.rs"


trailing...
"#]]);
}

// insertion

#[test]
fn test_insert_leaf_table() {
    given(
        r#"[servers]

        [servers.alpha]
        ip = "10.0.0.1"
        dc = "eqdc10"

        [other.table]"#,
    )
    .running(|root| {
        root["servers"]["beta"] = table();
        root["servers"]["beta"]["ip"] = value("10.0.0.2");
        root["servers"]["beta"]["dc"] = value("eqdc10");
    })
    .produces_display(str![[r#"
[servers]

        [servers.alpha]
        ip = "10.0.0.1"
        dc = "eqdc10"

[servers.beta]
ip = "10.0.0.2"
dc = "eqdc10"

        [other.table]

"#]]);
}

#[test]
fn test_inserted_leaf_table_goes_after_last_sibling() {
    given(
        r#"
        [package]
        [dependencies]
        [[example]]
        [dependencies.opencl]
        [dev-dependencies]"#,
    )
    .running(|root| {
        root["dependencies"]["newthing"] = table();
    })
    .produces_display(str![[r#"

        [package]
        [dependencies]
        [[example]]
        [dependencies.opencl]

[dependencies.newthing]
        [dev-dependencies]

"#]]);
}

#[test]
fn test_inserting_tables_from_different_parsed_docs() {
    given("[a]")
        .running(|root| {
            let other = "[b]".parse::<DocumentMut>().unwrap();
            root["b"] = other["b"].clone();
        })
        .produces_display(str![[r#"
[a]
[b]

"#]]);
}
#[test]
fn test_insert_nonleaf_table() {
    given(
        r#"
        [other.table]"#,
    )
    .running(|root| {
        root["servers"] = table();
        root["servers"]["alpha"] = table();
        root["servers"]["alpha"]["ip"] = value("10.0.0.1");
        root["servers"]["alpha"]["dc"] = value("eqdc10");
    })
    .produces_display(str![[r#"

        [other.table]

[servers]

[servers.alpha]
ip = "10.0.0.1"
dc = "eqdc10"

"#]]);
}

#[test]
fn test_insert_array() {
    given(
        r#"
        [package]
        title = "withoutarray""#,
    )
    .running(|root| {
        root["bin"] = array();
        assert!(root["bin"].is_array_of_tables());
        let array = root["bin"].as_array_of_tables_mut().unwrap();
        {
            let mut table = Table::new();
            table["hello"] = value("world");
            array.push(table);
        }
        array.push(Table::new());
    })
    .produces_display(str![[r#"

        [package]
        title = "withoutarray"

[[bin]]
hello = "world"

[[bin]]

"#]]);
}

#[test]
fn test_insert_values() {
    given(
        r#"
        [tbl.son]"#,
    )
    .running(|root| {
        root["tbl"]["key1"] = value("value1");
        root["tbl"]["key2"] = value(42);
        root["tbl"]["key3"] = value(8.1415926);
    })
    .produces_display(str![[r#"
[tbl]
key1 = "value1"
key2 = 42
key3 = 8.1415926

        [tbl.son]

"#]]);
}

#[test]
fn test_insert_key_with_quotes() {
    given(
        r#"
        [package]
        name = "foo"

        [target]
        "#,
    )
    .running(|root| {
        root["target"]["cfg(target_os = \"linux\")"] = table();
        root["target"]["cfg(target_os = \"linux\")"]["dependencies"] = table();
        root["target"]["cfg(target_os = \"linux\")"]["dependencies"]["name"] = value("dep");
    })
    .produces_display(str![[r#"

        [package]
        name = "foo"

        [target]

[target.'cfg(target_os = "linux")']

[target.'cfg(target_os = "linux")'.dependencies]
name = "dep"
        
"#]]);
}

// removal

#[test]
fn test_remove_leaf_table() {
    given(
        r#"
        [servers]

        # Indentation (tabs and/or spaces) is allowed but not required
[servers.alpha]
        ip = "10.0.0.1"
        dc = "eqdc10"

        [servers.beta]
        ip = "10.0.0.2"
        dc = "eqdc10""#,
    )
    .running(|root| {
        let servers = root.get_mut("servers").unwrap();
        let servers = as_table!(servers);
        assert!(servers.remove("alpha").is_some());
    })
    .produces_display(str![[r#"

        [servers]

        [servers.beta]
        ip = "10.0.0.2"
        dc = "eqdc10"

"#]]);
}

#[test]
fn test_remove_nonleaf_table() {
    given(
        r#"
        title = "not relevant"

        # comment 1
        [a.b.c] # comment 1.1
        key1 = 1 # comment 1.2
        # comment 2
        [b] # comment 2.1
        key2 = 2 # comment 2.2

        # comment 3
        [a] # comment 3.1
        key3 = 3 # comment 3.2
        [[a.'array']]
        b = 1

        [[a.b.c.trololololololo]] # ohohohohoho
        c = 2
        key3 = 42

           # comment on some other table
           [some.other.table]




        # comment 4
        [a.b] # comment 4.1
        key4 = 4 # comment 4.2
        key41 = 41 # comment 4.3


    "#,
    )
    .running(|root| {
        assert!(root.remove("a").is_some());
    })
    .produces_display(str![[r#"

        title = "not relevant"
        # comment 2
        [b] # comment 2.1
        key2 = 2 # comment 2.2

           # comment on some other table
           [some.other.table]


    "#]]);
}

#[test]
fn test_remove_array_entry() {
    given(
        r#"
        [package]
        name = "hello"
        version = "1.0.0"

        [[bin]]
        name = "world"
        path = "src/bin/world/main.rs"

        [dependencies]
        nom = "4.0" # future is here

        [[bin]]
        name = "delete me please"
        path = "src/bin/dmp/main.rs""#,
    )
    .running(|root| {
        let dmp = root.get_mut("bin").unwrap();
        assert!(dmp.is_array_of_tables());
        let dmp = dmp.as_array_of_tables_mut().unwrap();
        assert_eq!(dmp.len(), 2);
        dmp.remove(1);
        assert_eq!(dmp.len(), 1);
    })
    .produces_display(str![[r#"

        [package]
        name = "hello"
        version = "1.0.0"

        [[bin]]
        name = "world"
        path = "src/bin/world/main.rs"

        [dependencies]
        nom = "4.0" # future is here

"#]]);
}

#[test]
fn test_remove_array() {
    given(
        r#"
        [package]
        name = "hello"
        version = "1.0.0"

        [[bin]]
        name = "world"
        path = "src/bin/world/main.rs"

        [dependencies]
        nom = "4.0" # future is here

        [[bin]]
        name = "delete me please"
        path = "src/bin/dmp/main.rs""#,
    )
    .running(|root| {
        assert!(root.remove("bin").is_some());
    })
    .produces_display(str![[r#"

        [package]
        name = "hello"
        version = "1.0.0"

        [dependencies]
        nom = "4.0" # future is here

"#]]);
}

#[test]
fn test_remove_value() {
    given(
        r#"
        name = "hello"
        # delete this
        version = "1.0.0" # please
        documentation = "https://docs.rs/hello""#,
    )
    .running(|root| {
        let value = root.remove("version");
        assert!(value.is_some());
        let value = value.unwrap();
        assert!(value.is_value());
        let value = value.as_value().unwrap();
        assert!(value.is_str());
        let value = value.as_str().unwrap();
        assert_data_eq!(value, str!["1.0.0"].raw());
    })
    .produces_display(str![[r#"

        name = "hello"
        documentation = "https://docs.rs/hello"

"#]]);
}

#[test]
fn test_remove_last_value_from_implicit() {
    given(
        r#"
        [a]
        b = 1"#,
    )
    .running(|root| {
        let a = root.get_mut("a").unwrap();
        assert!(a.is_table());
        let a = as_table!(a);
        a.set_implicit(true);
        let value = a.remove("b");
        assert!(value.is_some());
        let value = value.unwrap();
        assert!(value.is_value());
        let value = value.as_value().unwrap();
        assert_eq!(value.as_integer(), Some(1));
    })
    .produces_display(str![]);
}

// values

#[test]
fn test_sort_values() {
    given(
        r#"
        [a.z]

        [a]
        # this comment is attached to b
        b = 2 # as well as this
        a = 1
        c = 3

        [a.y]"#,
    )
    .running(|root| {
        let a = root.get_mut("a").unwrap();
        let a = as_table!(a);
        a.sort_values();
    })
    .produces_display(str![[r#"

        [a.z]

        [a]
        a = 1
        # this comment is attached to b
        b = 2 # as well as this
        c = 3

        [a.y]

"#]]);
}

#[test]
fn test_sort_values_by() {
    given(
        r#"
        [a.z]

        [a]
        # this comment is attached to b
        b = 2 # as well as this
        a = 1
        "c" = 3

        [a.y]"#,
    )
    .running(|root| {
        let a = root.get_mut("a").unwrap();
        let a = as_table!(a);
        // Sort by the representation, not the value. So "\"c\"" sorts before "a" because '"' sorts
        // before 'a'.
        a.sort_values_by(|k1, _, k2, _| k1.display_repr().cmp(&k2.display_repr()));
    })
    .produces_display(str![[r#"

        [a.z]

        [a]
        "c" = 3
        a = 1
        # this comment is attached to b
        b = 2 # as well as this

        [a.y]

"#]]);
}

#[test]
fn test_set_position() {
    given(
        r#"
        [package]
        [dependencies]
        [dependencies.opencl]
        [dev-dependencies]"#,
    )
    .running(|root| {
        for (header, table) in root.iter_mut() {
            if header == "dependencies" {
                let tab = as_table!(table);
                tab.set_position(Some(0));
                let (_, segmented) = tab.iter_mut().next().unwrap();
                as_table!(segmented).set_position(Some(5));
            }
        }
    })
    .produces_display(str![[r#"
        [dependencies]

        [package]
        [dev-dependencies]
        [dependencies.opencl]

"#]]);
}

#[test]
fn test_multiple_zero_positions() {
    given(
        r#"
        [package]
        [dependencies]
        [dependencies.opencl]
        a=""
        [dev-dependencies]"#,
    )
    .running(|root| {
        for (_, table) in root.iter_mut() {
            as_table!(table).set_position(Some(0));
        }
    })
    .produces_display(str![[r#"

        [package]
        [dependencies]
        [dev-dependencies]
        [dependencies.opencl]
        a=""

"#]]);
}

#[test]
fn test_multiple_max_usize_positions() {
    given(
        r#"
        [package]
        [dependencies]
        [dependencies.opencl]
        a=""
        [dev-dependencies]"#,
    )
    .running(|root| {
        for (_, table) in root.iter_mut() {
            as_table!(table).set_position(Some(isize::MAX));
        }
    })
    .produces_display(str![[r#"
        [dependencies.opencl]
        a=""

        [package]
        [dependencies]
        [dev-dependencies]

"#]]);
}

macro_rules! as_array {
    ($entry:ident) => {{
        assert!($entry.is_value());
        let a = $entry.as_value_mut().unwrap();
        assert!(a.is_array());
        a.as_array_mut().unwrap()
    }};
}

#[test]
fn test_insert_replace_into_array() {
    given(
        r#"
        a = [1,2,3]
        b = []"#,
    )
    .running(|root| {
        {
            let a = root.get_mut("a").unwrap();
            let a = as_array!(a);
            assert_eq!(a.len(), 3);
            assert!(a.get(2).is_some());
            a.push(4);
            assert_eq!(a.len(), 4);
            a.fmt();
        }
        let b = root.get_mut("b").unwrap();
        let b = as_array!(b);
        assert!(b.is_empty());
        b.push("hello");
        assert_eq!(b.len(), 1);

        b.push_formatted(Value::from("world").decorated("\n", "\n"));
        b.push_formatted(Value::from("test").decorated("", ""));

        b.insert(1, "beep");
        b.insert_formatted(2, Value::from("boop").decorated("   ", "   "));

        // This should preserve formatting.
        assert_eq!(b.replace(2, "zoink").as_str(), Some("boop"));
        // This should replace formatting.
        assert_eq!(
            b.replace_formatted(4, Value::from("yikes").decorated("  ", ""))
                .as_str(),
            Some("test")
        );
        dbg!(root);
    })
    .produces_display(str![[r#"

        a = [1, 2, 3, 4]
        b = ["hello", "beep",   "zoink"   ,
"world"
,  "yikes"]

"#]]);
}

#[test]
fn test_remove_from_array() {
    given(
        r#"
        a = [1, 2, 3, 4]
        b = ["hello"]"#,
    )
    .running(|root| {
        {
            let a = root.get_mut("a").unwrap();
            let a = as_array!(a);
            assert_eq!(a.len(), 4);
            assert!(a.remove(3).is_integer());
            assert_eq!(a.len(), 3);
        }
        let b = root.get_mut("b").unwrap();
        let b = as_array!(b);
        assert_eq!(b.len(), 1);
        assert!(b.remove(0).is_str());
        assert!(b.is_empty());
    })
    .produces_display(str![[r#"

        a = [1, 2, 3]
        b = []

"#]]);
}

#[test]
fn test_format_array() {
    given(
        r#"
    a = [
      1,
            "2",
      3.0,
    ]
    "#,
    )
    .running(|root| {
        for (_, v) in root.iter_mut() {
            if let Item::Value(Value::Array(array)) = v {
                array.fmt();
            }
        }
    })
    .produces_display(str![[r#"

    a = [1, "2", 3.0]
    "#]]);
}

macro_rules! as_inline_table {
    ($entry:ident) => {{
        assert!($entry.is_value());
        let a = $entry.as_value_mut().unwrap();
        assert!(a.is_inline_table());
        a.as_inline_table_mut().unwrap()
    }};
}

#[test]
fn test_insert_into_inline_table() {
    given(
        r#"
        a = {a=2,  c = 3}
        b = {}"#,
    )
    .running(|root| {
        {
            let a = root.get_mut("a").unwrap();
            let a = as_inline_table!(a);
            assert_eq!(a.len(), 2);
            assert!(a.contains_key("a") && a.get("c").is_some() && a.get_mut("c").is_some());
            a.get_or_insert("b", 42);
            assert_eq!(a.len(), 3);
            a.fmt();
        }
        let b = root.get_mut("b").unwrap();
        let b = as_inline_table!(b);
        assert!(b.is_empty());
        b.get_or_insert("hello", "world");
        assert_eq!(b.len(), 1);
        b.fmt();
    })
    .produces_display(str![[r#"

        a = { a = 2, c = 3, b = 42 }
        b = { hello = "world" }

"#]]);
}

#[test]
fn test_remove_from_inline_table() {
    given(
        r#"
        a = {a=2,  c = 3, b = 42}
        b = {'hello' = "world"}"#,
    )
    .running(|root| {
        {
            let a = root.get_mut("a").unwrap();
            let a = as_inline_table!(a);
            assert_eq!(a.len(), 3);
            assert!(a.remove("c").is_some());
            assert_eq!(a.len(), 2);
        }
        let b = root.get_mut("b").unwrap();
        let b = as_inline_table!(b);
        assert_eq!(b.len(), 1);
        assert!(b.remove("hello").is_some());
        assert!(b.is_empty());
    })
    .produces_display(str![[r#"

        a = {a=2, b = 42}
        b = {}

"#]]);
}

#[test]
fn test_as_table_like() {
    given(
        r#"
        a = {a=2,  c = 3, b = 42}
        x = {}
        [[bin]]
        [b]
        x = "y"
        [empty]"#,
    )
    .running(|root| {
        let a = root["a"].as_table_like();
        assert!(a.is_some());
        let a = a.unwrap();
        assert_eq!(a.iter().count(), 3);
        assert_eq!(a.len(), 3);
        assert_eq!(a.get("a").and_then(Item::as_integer), Some(2));

        let b = root["b"].as_table_like();
        assert!(b.is_some());
        let b = b.unwrap();
        assert_eq!(b.iter().count(), 1);
        assert_eq!(b.len(), 1);
        assert_eq!(b.get("x").and_then(Item::as_str), Some("y"));

        assert_eq!(root["x"].as_table_like().map(|t| t.iter().count()), Some(0));
        assert_eq!(
            root["empty"].as_table_like().map(|t| t.is_empty()),
            Some(true)
        );

        assert!(root["bin"].as_table_like().is_none());
    });
}

#[test]
fn test_inline_table_append() {
    let mut a = Value::from_iter(vec![
        (parse_key!("a"), 1),
        (parse_key!("b"), 2),
        (parse_key!("c"), 3),
    ]);
    let a = a.as_inline_table_mut().unwrap();

    let mut b = Value::from_iter(vec![
        (parse_key!("c"), 4),
        (parse_key!("d"), 5),
        (parse_key!("e"), 6),
    ]);
    let b = b.as_inline_table_mut().unwrap();

    a.extend(b.iter());
    assert_eq!(a.len(), 5);
    assert!(a.contains_key("e"));
    assert_eq!(b.len(), 3);
}

#[test]
fn test_insert_dotted_into_std_table() {
    given("")
        .running(|root| {
            root["nixpkgs"] = table();

            root["nixpkgs"]["src"] = table();
            root["nixpkgs"]["src"]
                .as_table_like_mut()
                .unwrap()
                .set_dotted(true);
            root["nixpkgs"]["src"]["git"] = value("https://github.com/nixos/nixpkgs");
        })
        .produces_display(str![[r#"
[nixpkgs]
src.git = "https://github.com/nixos/nixpkgs"

"#]]);
}

#[test]
fn test_insert_dotted_into_implicit_table() {
    given("")
        .running(|root| {
            root["nixpkgs"] = table();

            root["nixpkgs"]["src"]["git"] = value("https://github.com/nixos/nixpkgs");
            root["nixpkgs"]["src"]
                .as_table_like_mut()
                .unwrap()
                .set_dotted(true);
        })
        .produces_display(str![[r#"
[nixpkgs]
src.git = "https://github.com/nixos/nixpkgs"

"#]]);
}

#[test]
fn sorting_with_references() {
    let values = vec!["foo", "qux", "bar"];
    let mut array = toml_edit::Array::from_iter(values);
    array.sort_by(|lhs, rhs| lhs.as_str().cmp(&rhs.as_str()));
}

#[test]
fn table_str_key_whitespace() {
    let mut document = "bookmark = 1010".parse::<DocumentMut>().unwrap();

    let key: &str = "bookmark";

    document.insert(key, array());
    let table = document[key].as_array_of_tables_mut().unwrap();

    let mut bookmark_table = Table::new();
    bookmark_table["name"] = value("test.swf".to_owned());
    table.push(bookmark_table);

    assert_data_eq!(
        document.to_string(),
        str![[r#"
[[bookmark]]
name = "test.swf"

"#]]
    );
}

#[test]
fn table_key_decor_whitespace() {
    let mut document = "bookmark = 1010".parse::<DocumentMut>().unwrap();

    let key = Key::parse("  bookmark   ").unwrap().remove(0);

    document.insert_formatted(&key, array());
    let table = document[&key].as_array_of_tables_mut().unwrap();

    let mut bookmark_table = Table::new();
    bookmark_table["name"] = value("test.swf".to_owned());
    table.push(bookmark_table);

    assert_data_eq!(
        document.to_string(),
        str![[r#"
[[  bookmark   ]]
name = "test.swf"

"#]]
    );
}

#[test]
fn table_into_inline() {
    let toml = r#"
[table]
string = "value"
array = [1, 2, 3]
inline = { "1" = 1, "2" = 2 }

[table.child]
other = "world"
"#;
    let mut doc = toml.parse::<DocumentMut>().unwrap();

    doc.get_mut("table").unwrap().make_value();

    let actual = doc.to_string();
    // `table=` is because we didn't re-format the table key, only the value
    assert_data_eq!(actual, str![[r#"
table= { string = "value", array = [1, 2, 3], inline = { "1" = 1, "2" = 2 }, child = { other = "world" } }

"#]].raw());
}

#[test]
fn inline_table_to_table() {
    let toml = r#"table = { string = "value", array = [1, 2, 3], inline = { "1" = 1, "2" = 2 }, child = { other = "world" } }
"#;
    let mut doc = toml.parse::<DocumentMut>().unwrap();

    let t = doc.remove("table").unwrap();
    let t = match t {
        Item::Value(Value::InlineTable(t)) => t,
        _ => unreachable!("Unexpected {:?}", t),
    };
    let t = t.into_table();
    doc.insert("table", Item::Table(t));

    let actual = doc.to_string();
    assert_data_eq!(
        actual,
        str![[r#"
[table]
string = "value"
array = [1, 2, 3]
inline = { "1" = 1, "2" = 2 }
child = { other = "world" }

"#]]
        .raw()
    );
}

#[test]
fn array_of_tables_to_array() {
    let toml = r#"
[[table]]
string = "value"
array = [1, 2, 3]
inline = { "1" = 1, "2" = 2 }

[table.child]
other = "world"

[[table]]
string = "value"
array = [1, 2, 3]
inline = { "1" = 1, "2" = 2 }

[table.child]
other = "world"
"#;
    let mut doc = toml.parse::<DocumentMut>().unwrap();

    doc.get_mut("table").unwrap().make_value();

    let actual = doc.to_string();
    // `table=` is because we didn't re-format the table key, only the value
    assert_data_eq!(actual, str![[r#"
table= [{ string = "value", array = [1, 2, 3], inline = { "1" = 1, "2" = 2 }, child = { other = "world" } }, { string = "value", array = [1, 2, 3], inline = { "1" = 1, "2" = 2 }, child = { other = "world" } }]

"#]].raw());
}

#[test]
fn test_key_from_str() {
    macro_rules! test_key {
        ($s:expr, $expected:expr) => {{
            let key = $s.parse::<Key>();
            match key {
                Ok(key) => assert_eq!($expected, key.get(), ""),
                Err(err) => panic!("failed with {err}"),
            }
        }};
    }

    test_key!("a", "a");
    test_key!(r#"'hello key'"#, "hello key");
    test_key!(
        r#""Jos\u00E9\U000A0000\n\t\r\f\b\"""#,
        "Jos\u{00E9}\u{A0000}\n\t\r\u{c}\u{8}\""
    );
    test_key!("\"\"", "");
    test_key!("\"'hello key'bla\"", "'hello key'bla");
    test_key!(
        "'C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\cargo-edit-test.YizxPxxElXn9'",
        "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\cargo-edit-test.YizxPxxElXn9"
    );
}

#[test]
fn despan_keys() {
    let mut doc = r#"aaaaaa = 1"#.parse::<DocumentMut>().unwrap();
    let key = "bbb".parse::<Key>().unwrap();
    let table = doc.as_table_mut();
    table.insert_formatted(
        &key,
        Item::Value(Value::Integer(toml_edit::Formatted::new(2))),
    );

    assert_eq!(doc.to_string(), "aaaaaa = 1\nbbb = 2\n");
}

#[test]
fn key_repr_roundtrip() {
    assert_key_repr_roundtrip(r#""""#, str![[r#""""#]]);
    assert_key_repr_roundtrip(r#""a""#, str![[r#""a""#]]);

    assert_key_repr_roundtrip(r#""tab \t tab""#, str![[r#""tab \t tab""#]]);
    assert_key_repr_roundtrip(r#""lf \n lf""#, str![[r#""lf \n lf""#]]);
    assert_key_repr_roundtrip(r#""crlf \r\n crlf""#, str![[r#""crlf \r\n crlf""#]]);
    assert_key_repr_roundtrip(r#""bell \b bell""#, str![[r#""bell \b bell""#]]);
    assert_key_repr_roundtrip(r#""feed \f feed""#, str![[r#""feed \f feed""#]]);
    assert_key_repr_roundtrip(
        r#""backslash \\ backslash""#,
        str![[r#""backslash \\ backslash""#]],
    );

    assert_key_repr_roundtrip(r#""squote ' squote""#, str![[r#""squote ' squote""#]]);
    assert_key_repr_roundtrip(
        r#""triple squote ''' triple squote""#,
        str![[r#""triple squote ''' triple squote""#]],
    );
    assert_key_repr_roundtrip(r#""end squote '""#, str![[r#""end squote '""#]]);

    assert_key_repr_roundtrip(r#""quote \" quote""#, str![[r#""quote \" quote""#]]);
    assert_key_repr_roundtrip(
        r#""triple quote \"\"\" triple quote""#,
        str![[r#""triple quote \"\"\" triple quote""#]],
    );
    assert_key_repr_roundtrip(r#""end quote \"""#, str![[r#""end quote \"""#]]);
    assert_key_repr_roundtrip(
        r#""quoted \"content\" quoted""#,
        str![[r#""quoted \"content\" quoted""#]],
    );
    assert_key_repr_roundtrip(
        r#""squoted 'content' squoted""#,
        str![[r#""squoted 'content' squoted""#]],
    );
    assert_key_repr_roundtrip(
        r#""mixed quoted \"start\" 'end'' mixed quote""#,
        str![[r#""mixed quoted \"start\" 'end'' mixed quote""#]],
    );
}

#[track_caller]
fn assert_key_repr_roundtrip(input: &str, expected: impl IntoData) {
    let value: Key = input.parse().unwrap();
    let actual = value.to_string();
    let _: Key = actual.parse().unwrap_or_else(|_err| {
        panic!(
            "invalid `Key`:
```
{actual}
```
"
        )
    });
    assert_data_eq!(actual, expected.raw());
}

#[test]
fn key_value_roundtrip() {
    assert_key_value_roundtrip(r#""""#, str![[r#""""#]]);
    assert_key_value_roundtrip(r#""a""#, str!["a"]);

    assert_key_value_roundtrip(r#""tab \t tab""#, str![[r#""tab \t tab""#]]);
    assert_key_value_roundtrip(r#""lf \n lf""#, str![[r#""lf \n lf""#]]);
    assert_key_value_roundtrip(r#""crlf \r\n crlf""#, str![[r#""crlf \r\n crlf""#]]);
    assert_key_value_roundtrip(r#""bell \b bell""#, str![[r#""bell \b bell""#]]);
    assert_key_value_roundtrip(r#""feed \f feed""#, str![[r#""feed \f feed""#]]);
    assert_key_value_roundtrip(
        r#""backslash \\ backslash""#,
        str![[r#"'backslash \ backslash'"#]],
    );

    assert_key_value_roundtrip(r#""squote ' squote""#, str![[r#""squote ' squote""#]]);
    assert_key_value_roundtrip(
        r#""triple squote ''' triple squote""#,
        str![[r#""triple squote ''' triple squote""#]],
    );
    assert_key_value_roundtrip(r#""end squote '""#, str![[r#""end squote '""#]]);

    assert_key_value_roundtrip(r#""quote \" quote""#, str![[r#"'quote " quote'"#]]);
    assert_key_value_roundtrip(
        r#""triple quote \"\"\" triple quote""#,
        str![[r#"'triple quote """ triple quote'"#]],
    );
    assert_key_value_roundtrip(r#""end quote \"""#, str![[r#"'end quote "'"#]]);
    assert_key_value_roundtrip(
        r#""quoted \"content\" quoted""#,
        str![[r#"'quoted "content" quoted'"#]],
    );
    assert_key_value_roundtrip(
        r#""squoted 'content' squoted""#,
        str![[r#""squoted 'content' squoted""#]],
    );
    assert_key_value_roundtrip(
        r#""mixed quoted \"start\" 'end'' mixed quote""#,
        str![[r#""mixed quoted \"start\" 'end'' mixed quote""#]],
    );
}

#[track_caller]
fn assert_key_value_roundtrip(input: &str, expected: impl IntoData) {
    let value: Key = input.parse().unwrap();
    let value = Key::new(value.get()); // Remove repr
    let actual = value.to_string();
    let _: Key = actual.parse().unwrap_or_else(|_err| {
        panic!(
            "invalid `Key`:
```
{actual}
```
"
        )
    });
    assert_data_eq!(actual, expected.raw());
}

#[test]
fn table_under_inline() {
    let mut doc = DocumentMut::new();
    doc["tool"]["typst-test"] = table();
    doc["tool"]["typst-test"]["tests"] = value("tests");

    assert_data_eq!(
        doc.to_string(),
        str![[r#"
tool = { typst-test.tests = "tests" }

"#]]
    );
}
