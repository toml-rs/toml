extern crate toml_edit;

use toml_edit::{Document, Key};

macro_rules! test {
    ($before:expr, $root:ident, $ops:expr, $after:expr) => (
        let before = $before;
        let after = $after;

        let doc = Document::parse(before);
        assert!(doc.is_ok());
        let mut doc = doc.unwrap();

        {
            let $root = doc.root_mut();
            $ops
        }

        assert_eq!(doc.to_string(), after);
    );
}

macro_rules! parse_key {
    ($s:expr) => (
        {
            let key = $s.parse::<Key>();
            assert!(key.is_ok());
            key.unwrap()
        }
    );
}

// insertion

#[test]
fn test_insert_leaf_table() {
    test!(
        r#"
[servers]

[servers.alpha]
ip = "10.0.0.1"
dc = "eqdc10"

[other.table]
"#,
        root,
        {
            let mut servers = root.entry("servers");
            assert!(servers.is_table());
            let servers = servers.as_table_mut().unwrap();
            let mut beta = servers.insert_table(parse_key!("beta"));
            assert!(beta.is_table());
            let beta = beta.as_table_mut().unwrap();
            beta.append_value(parse_key!("ip"), "10.0.0.2");
            beta.append_value(parse_key!("dc"), "eqdc10");
        },
        r#"
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
fn test_insert_nonleaf_table() {
    test!(
        r#"
[other.table]
"#,
        root,
        {
            let mut servers = root.append_table(parse_key!("servers"));
            assert!(servers.is_table());
            let servers = servers.as_table_mut().unwrap();
            let mut alpha = servers.insert_table(parse_key!("alpha"));
            assert!(alpha.is_table());
            let alpha = alpha.as_table_mut().unwrap();
            alpha.append_value(parse_key!("ip"), "10.0.0.1");
            alpha.append_value(parse_key!("dc"), "eqdc10");
        },
        r#"
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
    test!(
        r#"
[package]
title = "withoutarray"
"#,
        root,
        {
            let mut array = root.insert_array(parse_key!("bin"));
            assert!(array.is_array());
            let array = array.as_array_mut().unwrap();
            {
                let mut first = array.append();
                first.append_value(parse_key!("hello"), "world");
            }
            array.append();
        },
        r#"
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
    test!(
        r#"
[tbl]
"#,
        root,
        {
            let mut table = root.entry("tbl");
            let table = table.as_table_mut().unwrap();
            table.append_value(parse_key!("key1"), "value1");
            table.append_value(parse_key!("\"key2\""), 42);
            table.append_value(parse_key!("'key3'"), 8.1415926);
        },

        r#"
[tbl]
key1 = "value1"
"key2" = 42
'key3' = 8.1415926
"#
    );
}

// removal

#[test]
fn test_remove_leaf_table() {
    test!(
        r#"
    [servers]

    # Indentation (tabs and/or spaces) is allowed but not required
[servers.alpha]
    ip = "10.0.0.1"
    dc = "eqdc10"

    [servers.beta]
    ip = "10.0.0.2"
    dc = "eqdc10"
"#,
        root,
        {
            let mut servers = root.entry("servers");
            assert!(servers.is_table());
            let servers = servers.as_table_mut().unwrap();
            assert!(servers.remove_table("alpha"));
        },
        r#"
    [servers]

    [servers.beta]
    ip = "10.0.0.2"
    dc = "eqdc10"
"#
    );
}


#[test]
fn test_remove_nonleaf_table() {
    test!(
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
        root,
        {
            assert!(root.remove_table("a"));
        },
        r#"
title = "not relevant"
# comment 2
[b] # comment 2.1
key2 = 2 # comment 2.2

   # comment on some other table
   [some.other.table]


"#
    );
}

#[test]
fn test_remove_array_entry() {
    test!(
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
    path = "src/bin/dmp/main.rs"
"#,
        root,
        {
            let mut dmp = root.entry("bin");
            assert!(dmp.is_array());
            let dmp = dmp.as_array_mut().unwrap();
            assert_eq!(dmp.len(), 2);
            dmp.remove(1);
        },
        r#"
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
    test!(
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
    path = "src/bin/dmp/main.rs"
"#,
        root,
        {
            assert!(root.remove_array("bin"));
        },
        r#"
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
    test!(
        r#"
    name = "hello"
    # delete this
    version = "1.0.0" # please
    documentation = "https://docs.rs/hello"
"#,
        root,
        {
            let value = root.remove_value("version");
            assert!(value.is_some());
            let value = value.unwrap();
            assert!(value.is_str());
            let value = value.as_str().unwrap();
            assert_eq!(value, "1.0.0");
        },
        r#"
    name = "hello"
    documentation = "https://docs.rs/hello"
"#
    );
}

// values

#[test]
fn test_sort_values() {
    test!(
        r#"
[a.z]

[a]
# this comment is attached to b
b = 2 # as well as this
a = 1
c = 3

[a.y]
"#,
        root,
        {
            let mut a = root.entry("a");
            assert!(a.is_table());
            let a = a.as_table_mut().unwrap();
            a.sort_values();
        },
        r#"
[a.z]

[a]
a = 1
# this comment is attached to b
b = 2 # as well as this
c = 3

[a.y]
"#
    );
}

macro_rules! as_array {
    ($entry:ident) => (
        {
            assert!($entry.is_value());
            let mut a = $entry.as_value_mut().unwrap();
            assert!(a.is_array());
            a.as_array_mut().unwrap()
        }
    );
}

#[test]
fn test_insert_into_array() {
    test!(
        r#"
a = [1,2,3]
b = []
"#,
        root,
        {
            {
                let mut a = root.entry("a");
                let mut a = as_array!(a);
                assert_eq!(a.len(), 3);
                assert!(a.get(2).is_some());
                assert!(a.push(4));
                assert_eq!(a.len(), 4);
                a.fmt();
            }
            let mut b = root.entry("b");
            let mut b = as_array!(b);
            assert!(b.is_empty());
            assert!(b.push("hello"));
            assert_eq!(b.len(), 1);
        },
        r#"
a = [1, 2, 3, 4]
b = ["hello"]
"#
    );
}

#[test]
fn test_remove_from_array() {
    test!(
        r#"
a = [1, 2, 3, 4]
b = ["hello"]
"#,
        root,
        {
            {
                let mut a = root.entry("a");
                let mut a = as_array!(a);
                assert_eq!(a.len(), 4);
                assert!(a.remove(3).is_integer());
                assert_eq!(a.len(), 3);
            }
            let mut b = root.entry("b");
            let mut b = as_array!(b);
            assert_eq!(b.len(), 1);
            assert!(b.remove(0).is_str());
            assert!(b.is_empty());
        },
        r#"
a = [1, 2, 3]
b = []
"#
    );
}

macro_rules! as_table {
    ($entry:ident) => (
        {
            assert!($entry.is_value());
            let mut a = $entry.as_value_mut().unwrap();
            assert!(a.is_inline_table());
            a.as_inline_table_mut().unwrap()
        }
    );
}

#[test]
fn test_insert_into_inline_table() {
    test!(
        r#"
a = {a=2,  c = 3}
b = {}
"#,
        root,
        {
            {
                let mut a = root.entry("a");
                let mut a = as_table!(a);
                assert_eq!(a.len(), 2);
                assert!(a.contains_key("a") && a.get("c").is_some());
                assert!(a.insert(parse_key!("b"), 42).is_none());
                assert_eq!(a.len(), 3);
                a.fmt();
            }
            let mut b = root.entry("b");
            let mut b = as_table!(b);
            assert!(b.is_empty());
            assert!(b.insert(parse_key!("'hello'"), "world").is_none());
            assert_eq!(b.len(), 1);
            b.fmt()
        },
        r#"
a = { a = 2, c = 3, b = 42 }
b = { 'hello' = "world" }
"#
    );
}

#[test]
fn test_remove_from_inline_table() {
    test!(
        r#"
a = {a=2,  c = 3, b = 42}
b = {'hello' = "world"}
"#,
        root,
        {
            {
                let mut a = root.entry("a");
                let mut a = as_table!(a);
                assert_eq!(a.len(), 3);
                assert!(a.remove("c").is_some());
                assert_eq!(a.len(), 2);
            }
            let mut b = root.entry("b");
            let mut b = as_table!(b);
            assert_eq!(b.len(), 1);
            assert!(b.remove("hello").is_some());
            assert!(b.is_empty());
        },
        r#"
a = {a=2, b = 42}
b = {}
"#
    );
}
