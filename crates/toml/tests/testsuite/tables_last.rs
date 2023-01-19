use std::collections::HashMap;

use serde::Deserialize;
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

#[test]
fn vec_of_vec_issue_387() {
    #[derive(Deserialize, Serialize, Debug)]
    struct Glyph {
        components: Vec<Component>,
        contours: Vec<Contour>,
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct Point {
        x: f64,
        y: f64,
        pt_type: String,
    }

    type Contour = Vec<Point>;

    #[derive(Deserialize, Serialize, Debug)]
    struct Component {
        base: String,
        transform: (f64, f64, f64, f64, f64, f64),
    }

    let comp1 = Component {
        base: "b".to_string(),
        transform: (1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
    };
    let comp2 = Component {
        base: "c".to_string(),
        transform: (1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
    };
    let components = vec![comp1, comp2];

    let contours = vec![
        vec![
            Point {
                x: 3.0,
                y: 4.0,
                pt_type: "line".to_string(),
            },
            Point {
                x: 5.0,
                y: 6.0,
                pt_type: "line".to_string(),
            },
        ],
        vec![
            Point {
                x: 0.0,
                y: 0.0,
                pt_type: "move".to_string(),
            },
            Point {
                x: 7.0,
                y: 9.0,
                pt_type: "offcurve".to_string(),
            },
            Point {
                x: 8.0,
                y: 10.0,
                pt_type: "offcurve".to_string(),
            },
            Point {
                x: 11.0,
                y: 12.0,
                pt_type: "curve".to_string(),
            },
        ],
    ];
    let g1 = Glyph {
        contours,
        components,
    };

    let s = toml::to_string_pretty(&g1).unwrap();
    let _g2: Glyph = toml::from_str(&s).unwrap();
}