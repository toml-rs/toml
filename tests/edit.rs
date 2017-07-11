#[macro_use]
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
"#);
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
"#);
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
"#);
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
            table.append_value(parse_key!("'key3'"), 3.1415926);
        },

        r#"
[tbl]
key1 = "value1"
"key2" = 42
'key3' = 3.1415926
"#);
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
"#);
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

[[a.b.c.trololololololo]]
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


"#);
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
"#);
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
"#);
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
"#);
}
