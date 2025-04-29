use serde::Serialize;
use snapbox::assert_data_eq;
use snapbox::str;

#[derive(Debug, Serialize, PartialEq)]
enum TheEnum {
    Plain,
    Tuple(i64, bool),
    NewType(String),
    Struct { value: i64 },
}

#[derive(Debug, Serialize, PartialEq)]
struct Val {
    val: TheEnum,
}

#[derive(Debug, Serialize, PartialEq)]
struct Multi {
    enums: Vec<TheEnum>,
}

mod enum_unit {
    use super::*;

    #[test]
    fn to_string_value() {
        let expected = str![[r#""Plain""#]];
        let input = TheEnum::Plain;
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn nested_to_string_value() {
        let expected = str![[r#"{ val = "Plain" }"#]];
        let input = Val {
            val: TheEnum::Plain,
        };
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn to_string() {
        let expected = str![[r#"
val = "Plain"

"#]];
        let input = Val {
            val: TheEnum::Plain,
        };
        let result = crate::to_string_pretty(&input);
        assert_data_eq!(result.unwrap(), expected);
    }
}

mod enum_tuple {
    use super::*;

    #[test]
    fn to_string_value() {
        let expected = str!["[-123, true]"];
        let input = TheEnum::Tuple(-123, true);
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn nested_to_string_value() {
        let expected = str!["{ val = { Tuple = [-123, true] } }"];
        let input = Val {
            val: TheEnum::Tuple(-123, true),
        };
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn to_string() {
        let expected = str![[r#"
[val]
Tuple = [
    -123,
    true,
]

"#]];
        let input = Val {
            val: TheEnum::Tuple(-123, true),
        };
        let result = crate::to_string_pretty(&input);
        assert_data_eq!(result.unwrap(), expected);
    }
}

mod enum_newtype {
    use super::*;

    #[test]
    fn to_string_value() {
        let expected = str![[r#"{ NewType = "value" }"#]];
        let input = TheEnum::NewType("value".to_owned());
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn nested_to_string_value() {
        let expected = str![[r#"{ val = { NewType = "value" } }"#]];
        let input = Val {
            val: TheEnum::NewType("value".to_owned()),
        };
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn to_string() {
        let expected = str![[r#"
[val]
NewType = "value"

"#]];
        let input = Val {
            val: TheEnum::NewType("value".to_owned()),
        };
        let result = crate::to_string_pretty(&input);
        assert_data_eq!(result.unwrap(), expected);
    }
}

mod enum_struct {
    use super::*;

    #[test]
    #[should_panic]
    fn to_string_value() {
        let expected = str!["{ Struct = { value = -123 } }"];
        let input = TheEnum::Struct { value: -123 };
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn nested_to_string_value() {
        let expected = str!["{ val = { Struct = { value = -123 } } }"];
        let input = Val {
            val: TheEnum::Struct { value: -123 },
        };
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn to_string() {
        let expected = str![[r#"
[val.Struct]
value = -123

"#]];
        let input = Val {
            val: TheEnum::Struct { value: -123 },
        };
        let result = crate::to_string_pretty(&input);
        assert_data_eq!(result.unwrap(), expected);
    }
}

mod array_enum {
    use super::*;

    #[test]
    fn to_string_value() {
        let expected = str![[
            r#"{ enums = ["Plain", { Tuple = [-123, true] }, { NewType = "value" }, { Struct = { value = -123 } }] }"#
        ]];
        let input = Multi {
            enums: {
                vec![
                    TheEnum::Plain,
                    TheEnum::Tuple(-123, true),
                    TheEnum::NewType("value".to_owned()),
                    TheEnum::Struct { value: -123 },
                ]
            },
        };
        let result = crate::to_string_value(&input);
        assert_data_eq!(result.unwrap(), expected);
    }

    #[test]
    fn to_string() {
        let expected = str![[r#"
enums = [
    "Plain",
    { Tuple = [
    -123,
    true,
] },
    { NewType = "value" },
    { Struct = { value = -123 } },
]

"#]];
        let input = Multi {
            enums: {
                vec![
                    TheEnum::Plain,
                    TheEnum::Tuple(-123, true),
                    TheEnum::NewType("value".to_owned()),
                    TheEnum::Struct { value: -123 },
                ]
            },
        };
        let result = crate::to_string_pretty(&input);
        assert_data_eq!(result.unwrap(), expected);
    }
}
