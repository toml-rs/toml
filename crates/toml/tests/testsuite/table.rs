use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

use toml::map::Map;
use toml::Value::{Array, Integer, String, Table};

#[test]
fn display() {
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
