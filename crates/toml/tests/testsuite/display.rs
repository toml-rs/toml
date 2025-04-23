use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

use toml::map::Map;
use toml::Value::{Array, Boolean, Float, Integer, String, Table};

macro_rules! map( ($($k:expr => $v:expr),*) => ({
    let mut _m = Map::new();
    $(_m.insert($k.to_owned(), $v);)*
    _m
}) );

#[test]
fn value() {
    assert_data_eq!(
        String("foo".to_owned()).to_string(),
        str![[r#""foo""#]].raw()
    );
    assert_data_eq!(Integer(10).to_string(), str!["10"].raw());
    assert_data_eq!(Float(10.0).to_string(), str!["10.0"].raw());
    assert_data_eq!(Float(2.4).to_string(), str!["2.4"].raw());
    assert_data_eq!(Boolean(true).to_string(), str!["true"].raw());
    assert_data_eq!(Array(vec![]).to_string(), str!["[]"].raw());
    assert_data_eq!(
        Array(vec![Integer(1), Integer(2)]).to_string(),
        str!["[1, 2]"].raw()
    );
    assert_data_eq!(
        Table(map! {"test" => Integer (2), "test2" => Integer(3)}).to_string(),
        str!["{ test = 2, test2 = 3 }"].raw()
    );
}

#[test]
fn table() {
    assert_data_eq!(map! {}.to_string(), "");
    assert_data_eq!(
        map! {
        "test" => Integer(2),
        "test2" => Integer(3) }
        .to_string(),
        str![[r#"
test = 2
test2 = 3

"#]]
        .raw()
    );
    assert_data_eq!(
        map! {
             "test" => Integer(2),
             "test2" => Table(map! {
                 "test" => String("wut".to_owned())
             })
        }
        .to_string(),
        str![[r#"
test = 2

[test2]
test = "wut"

"#]]
        .raw()
    );
    assert_data_eq!(
        map! {
             "test" => Integer(2),
             "test2" => Array(vec![Table(map! {
                 "test" => String("wut".to_owned())
             })])
        }
        .to_string(),
        str![[r#"
test = 2

[[test2]]
test = "wut"

"#]]
        .raw()
    );
}
