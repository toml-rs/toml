use std::collections::HashMap;

use serde::Serialize;

#[test]
fn always_works() {
    // Ensure this works without the removed "toml::ser::tables_last"
    #[derive(Serialize)]
    struct A {
        vals: HashMap<&'static str, Value>,
    }

    #[derive(Serialize)]
    #[serde(untagged)]
    enum Value {
        Map(HashMap<&'static str, &'static str>),
        Int(i32),
    }

    let mut a = A {
        vals: HashMap::new(),
    };
    a.vals.insert("foo", Value::Int(0));

    let mut sub = HashMap::new();
    sub.insert("foo", "bar");
    a.vals.insert("bar", Value::Map(sub));

    toml::to_string(&a).unwrap();
}
