use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
struct A {
    #[serde(serialize_with = "toml::ser::tables_last")]
    vals: HashMap<&'static str, Value>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Value {
    Map(HashMap<&'static str, &'static str>),
    Int(i32),
}

#[test]
fn always_works() {
    let mut a = A {
        vals: HashMap::new(),
    };
    a.vals.insert("foo", Value::Int(0));

    let mut sub = HashMap::new();
    sub.insert("foo", "bar");
    a.vals.insert("bar", Value::Map(sub));

    toml::to_string(&a).unwrap();
}
