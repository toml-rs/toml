// TL;DR don't mem::swap the tables
// it's unsafe
extern crate toml_edit;

use toml_edit::{Document, Key};
use std::mem;

macro_rules! parse_doc {
    ($toml:ident) => (
        {
            let doc = $toml.parse::<Document>();
            assert!(doc.is_ok());
            doc.unwrap()
        }
    );
}


macro_rules! as_table {
    ($entry:ident) => (
        {
            assert!($entry.is_table());
            $entry.as_table_mut().unwrap()
        }
    );
}

macro_rules! parse_key {
    ($s:expr) => ($s.parse::<Key>().unwrap());
}

#[test]
fn test_safety_issue() {

    let toml1 = r#"
[a]
b = 2
[a.c]
b = 3
[d]
"#;
    let toml2 = r#"
[b.c]
a = 3
[b]
a = 2
[e]
"#;
    let mut doc1 = parse_doc!(toml1);
    let mut doc2 = parse_doc!(toml2);

    {
        let mut r1 = doc1.root_mut();
        let mut r2 = doc2.root_mut();
        {
            let mut a = r1.entry("a");
            let mut a = as_table!(a);
            let mut ac = a.entry("c");
            let mut ac = as_table!(ac);
            ac.insert_table(parse_key!("ac"));

            let mut b = r2.entry("b");
            let mut b = as_table!(b);
            let mut bc = b.entry("c");
            let mut bc = as_table!(bc);
            bc.insert_table(parse_key!("bc"));

            mem::swap(ac, bc); // now both documents are invalid (duplicate keys) :(
        }

        // what's even worse,
        // `a.c` is now pointing to `[b.c]`
        // also, the trailing tables list is swapped
        let mut a = r1.entry("a");
        let mut a = as_table!(a);
        let mut ac = a.entry("c");
        let mut ac = as_table!(ac);
        ac.insert_table(parse_key!("'i am in [b.c]'"));

        // same for `b.c`
        let mut b = r2.entry("b");
        let mut b = as_table!(b);
        let mut bc = b.entry("c");
        let mut bc = as_table!(bc);
        bc.insert_table(parse_key!("'i am in [a.c]'"));
    }

    assert_eq!(
        doc1.to_string(),
        r#"
[a]
b = 2

[b.c]
a = 3

[b.c.bc]

[b.c.'i am in [b.c]']
[b]
a = 2
[e]
"#
    );
    assert_eq!(
        doc2.to_string(),
        r#"[a.c]
b = 3

[a.c.ac]

[a.c.'i am in [a.c]']
[d]
"#
    );
}
