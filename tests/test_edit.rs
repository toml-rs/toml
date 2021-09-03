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

// rusfmt, U Can't Touch This
#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use toml_edit::{Document, Key, Value, Table, Item, value, table, array};
    use std::iter::FromIterator;
    use std::fmt;
    use pretty_assertions::assert_eq;

    // Copied from https://github.com/colin-kiegel/rust-pretty-assertions/issues/24
    /// Wrapper around string slice that makes debug output `{:?}` to print string same way as `{}`.
    /// Used in different `assert*!` macros in combination with `pretty_assertions` crate to make
    /// test failures to show nice diffs.
    #[derive(PartialEq, Eq)]
    struct PrettyString<'a>(pub &'a str);
    /// Make diff to display string as multi-line string
    impl<'a> fmt::Debug for PrettyString<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    struct Test {
        doc: Document,
    }

    fn given(input: &str) -> Test {
        let doc = input.parse::<Document>();
        assert!(doc.is_ok());
        Test {
            doc: doc.unwrap(),
        }
    }

    impl Test {
        fn running<F>(&mut self, func: F) -> &mut Self
            where F: Fn(&mut Table)
        {
            {
                let root = self.doc.as_table_mut();
                func(root);
            }
            self
        }

        fn produces_display(&self, expected: &str) -> &Self {
            assert_eq!(
                PrettyString(expected),
                PrettyString(&self.doc.to_string()));
            self
        }
    }

// insertion

#[test]
fn test_insert_leaf_table() {
    given(r#"
        [servers]

        [servers.alpha]
        ip = "10.0.0.1"
        dc = "eqdc10"

        [other.table]"#
    ).running(|root| {
        root["servers"]["beta"] = table();
        root["servers"]["beta"]["ip"] = value("10.0.0.2");
        root["servers"]["beta"]["dc"] = value("eqdc10");
    }).produces_display(r#"
        [servers]

        [servers.alpha]
        ip = "10.0.0.1"
        dc = "eqdc10"

[servers.beta]
ip = "10.0.0.2"
dc = "eqdc10"

        [other.table]
"#
    );
}

#[test]
fn test_inserted_leaf_table_goes_after_last_sibling() {
    given(r#"
        [package]
        [dependencies]
        [[example]]
        [dependencies.opencl]
        [dev-dependencies]"#
    ).running(|root| {
        root["dependencies"]["newthing"] = table();
    }).produces_display(r#"
        [package]
        [dependencies]
        [[example]]
        [dependencies.opencl]

[dependencies.newthing]
        [dev-dependencies]
"#);
}

#[test]
fn test_inserting_tables_from_different_parsed_docs() {
    given(
        "[a]"
    ).running(|root| {
        let other = "[b]".parse::<Document>().unwrap();
        root["b"] = other["b"].clone();
    }).produces_display(
        "[a]\n[b]\n"
    );
}
#[test]
fn test_insert_nonleaf_table() {
    given(r#"
        [other.table]"#
    ).running(|root| {
        root["servers"] = table();
        root["servers"]["alpha"] = table();
        root["servers"]["alpha"]["ip"] = value("10.0.0.1");
        root["servers"]["alpha"]["dc"] = value("eqdc10");
    }).produces_display(r#"
        [other.table]

[servers]

[servers.alpha]
ip = "10.0.0.1"
dc = "eqdc10"
"#
    );
}

#[test]
fn test_insert_array() {
    given(r#"
        [package]
        title = "withoutarray""#
    ).running(|root| {
        root["bin"] = array();
        assert!(root["bin"].is_array_of_tables());
        let array = root["bin"].as_array_of_tables_mut().unwrap();
        {
            let first = array.append(Table::new());
            first["hello"] = value("world");
        }
        array.append(Table::new());
    }).produces_display(r#"
        [package]
        title = "withoutarray"

[[bin]]
hello = "world"

[[bin]]
"#
    );
}


#[test]
fn test_insert_values() {
    given(r#"
        [tbl.son]"#
    ).running(|root| {
        root["tbl"]["key1"] = value("value1");
        root["tbl"]["key2"] = value(42);
        root["tbl"]["key3"] = value(8.1415926);
    }).produces_display(r#"
[tbl]
key1 = "value1"
key2 = 42
key3 = 8.1415926

        [tbl.son]
"#
    );
}

// removal

#[test]
fn test_remove_leaf_table() {
    given(r#"
        [servers]

        # Indentation (tabs and/or spaces) is allowed but not required
[servers.alpha]
        ip = "10.0.0.1"
        dc = "eqdc10"

        [servers.beta]
        ip = "10.0.0.2"
        dc = "eqdc10""#
    ).running(|root| {
        let servers = root.get_mut("servers").unwrap();
        let servers = as_table!(servers);
        assert!(servers.remove("alpha").is_some());
    }).produces_display(r#"
        [servers]

        [servers.beta]
        ip = "10.0.0.2"
        dc = "eqdc10"
"#
    );
}


#[test]
fn test_remove_nonleaf_table() {
    given(r#"
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


    "#).running(|root| {
        assert!(root.remove("a").is_some());
    }).produces_display(r#"
        title = "not relevant"
        # comment 2
        [b] # comment 2.1
        key2 = 2 # comment 2.2

           # comment on some other table
           [some.other.table]


    "#);
}

#[test]
fn test_remove_array_entry() {
    given(r#"
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
        path = "src/bin/dmp/main.rs""#
    ).running(|root| {
        let dmp = root.get_mut("bin").unwrap();
        assert!(dmp.is_array_of_tables());
        let dmp = dmp.as_array_of_tables_mut().unwrap();
        assert_eq!(dmp.len(), 2);
        dmp.remove(1);
        assert_eq!(dmp.len(), 1);
    }).produces_display(r#"
        [package]
        name = "hello"
        version = "1.0.0"

        [[bin]]
        name = "world"
        path = "src/bin/world/main.rs"

        [dependencies]
        nom = "4.0" # future is here
"#
    );
}

#[test]
fn test_remove_array() {
    given(r#"
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
        path = "src/bin/dmp/main.rs""#
    ).running(|root| {
        assert!(root.remove("bin").is_some());
    }).produces_display(r#"
        [package]
        name = "hello"
        version = "1.0.0"

        [dependencies]
        nom = "4.0" # future is here
"#
    );
}


#[test]
fn test_remove_value() {
    given(r#"
        name = "hello"
        # delete this
        version = "1.0.0" # please
        documentation = "https://docs.rs/hello""#
    ).running(|root| {
        let value = root.remove("version");
        assert!(value.is_some());
        let value = value.unwrap();
        assert!(value.is_value());
        let value = value.as_value().unwrap();
        assert!(value.is_str());
        let value = value.as_str().unwrap();
        assert_eq!(value, "1.0.0");
    }).produces_display(r#"
        name = "hello"
        documentation = "https://docs.rs/hello"
"#
    );
}

#[test]
fn test_remove_last_value_from_implicit() {
    given(r#"
        [a]
        b = 1"#
    ).running(|root| {
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
    }).produces_display(r#""#);
}

// values

#[test]
fn test_sort_values() {
    given(r#"
        [a.z]

        [a]
        # this comment is attached to b
        b = 2 # as well as this
        a = 1
        c = 3

        [a.y]"#
    ).running(|root| {
        let a = root.get_mut("a").unwrap();
        let a = as_table!(a);
        a.sort_values();
    }).produces_display(r#"
        [a.z]

        [a]
        a = 1
        # this comment is attached to b
        b = 2 # as well as this
        c = 3

        [a.y]
"#);
}

#[test]
fn test_set_position() {
    given(r#"
        [package]
        [dependencies]
        [dependencies.opencl]
        [dev-dependencies]"#
    ).running(|root| {
        for (header, table) in root.iter_mut() {
            if header == "dependencies" {
                let tab = as_table!(table);
                tab.set_position(0);
                let (_, segmented) = tab.iter_mut().next().unwrap();
                as_table!(segmented).set_position(5)
            }
        }
    }).produces_display(r#"        [dependencies]

        [package]
        [dev-dependencies]
        [dependencies.opencl]
"#);
}

#[test]
fn test_multiple_zero_positions() {
    given(r#"
        [package]
        [dependencies]
        [dependencies.opencl]
        a=""
        [dev-dependencies]"#
    ).running(|root| {
        for (_, table) in root.iter_mut() {
            as_table!(table).set_position(0)
        }
    }).produces_display(r#"
        [package]
        [dependencies]
        [dev-dependencies]
        [dependencies.opencl]
        a=""
"#);
}

#[test]
fn test_multiple_max_usize_positions() {
    given(r#"
        [package]
        [dependencies]
        [dependencies.opencl]
        a=""
        [dev-dependencies]"#
    ).running(|root| {
        for (_, table) in root.iter_mut() {
            as_table!(table).set_position(usize::MAX)
        }
    }).produces_display(r#"        [dependencies.opencl]
        a=""

        [package]
        [dependencies]
        [dev-dependencies]
"#);
}

macro_rules! as_array {
    ($entry:ident) => (
        {
            assert!($entry.is_value());
            let a = $entry.as_value_mut().unwrap();
            assert!(a.is_array());
            a.as_array_mut().unwrap()
        }
    );
}

#[test]
fn test_insert_replace_into_array() {
    given(r#"
        a = [1,2,3]
        b = []"#
    ).running(|root| {
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
        assert_eq!(b.replace_formatted(4, Value::from("yikes").decorated("  ", "")).as_str(), Some("test"));

    }).produces_display(r#"
        a = [1, 2, 3, 4]
        b = ["hello", "beep",   "zoink"   ,
"world"
,  "yikes"]
"#
    );
}

#[test]
fn test_remove_from_array() {
    given(r#"
        a = [1, 2, 3, 4]
        b = ["hello"]"#
    ).running(|root| {
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
    }).produces_display(r#"
        a = [1, 2, 3]
        b = []
"#
    );
}

macro_rules! as_inline_table {
    ($entry:ident) => (
        {
            assert!($entry.is_value());
            let a = $entry.as_value_mut().unwrap();
            assert!(a.is_inline_table());
            a.as_inline_table_mut().unwrap()
        }
    );
}

#[test]
fn test_insert_into_inline_table() {
    given(r#"
        a = {a=2,  c = 3}
        b = {}"#
    ).running(|root| {
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
        b.fmt()
    }).produces_display(r#"
        a = { a = 2, c = 3, b = 42 }
        b = { hello = "world" }
"#
    );
}

#[test]
fn test_remove_from_inline_table() {
    given(r#"
        a = {a=2,  c = 3, b = 42}
        b = {'hello' = "world"}"#
    ).running(|root| {
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
    }).produces_display(r#"
        a = {a=2, b = 42}
        b = {}
"#
    );
}

#[test]
fn test_as_table_like() {
    given(r#"
        a = {a=2,  c = 3, b = 42}
        x = {}
        [[bin]]
        [b]
        x = "y"
        [empty]"#
    ).running(|root| {
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
        assert_eq!(root["empty"].as_table_like().map(|t| t.is_empty()), Some(true));

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

} // mod tests
