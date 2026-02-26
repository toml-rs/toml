#![allow(renamed_and_removed_lints)]
#![allow(clippy::blacklisted_name)]

use std::collections::BTreeMap;
use std::fmt::Debug;

use serde::de::{Deserializer, MapAccess};
use serde::Deserialize;
use serde_untagged::UntaggedEnumVisitor;
use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

use crate::Datetime;
use crate::Spanned;

#[test]
fn test_spanned_field() {
    #[derive(Deserialize, Debug)]
    struct Foo<T> {
        foo: Spanned<T>,
    }

    #[derive(Deserialize, Debug)]
    struct BareFoo<T> {
        foo: T,
    }

    #[track_caller]
    fn good<T>(input: &str, expected: impl IntoData, span: impl IntoData)
    where
        T: serde::de::DeserializeOwned + Debug + PartialEq,
    {
        dbg!(input);
        let foo: Foo<T> = crate::from_str(input).unwrap();
        dbg!(&foo);

        assert_data_eq!(&input[foo.foo.span()], expected,);
        assert_data_eq!(foo.foo.span().to_debug(), span);

        // Test for Spanned<> at the top level
        let foo_outer: Spanned<BareFoo<T>> = crate::from_str(input).unwrap();
        dbg!(&foo_outer);

        assert_eq!(&foo_outer.get_ref().foo, foo.foo.get_ref());
        assert_eq!(foo_outer.span(), 0..0);
    }

    good::<String>(
        "foo = \"foo\"",
        str![[r#""foo""#]],
        str![[r#"
6..11

"#]],
    );
    good::<u32>(
        "foo = 42",
        str!["42"],
        str![[r#"
6..8

"#]],
    );
    // leading plus
    good::<u32>(
        "foo = +42",
        str!["+42"],
        str![[r#"
6..9

"#]],
    );
    // table
    good::<BTreeMap<String, u32>>(
        "foo = {\"foo\" = 42, \"bar\" = 42}",
        str![[r#"{"foo" = 42, "bar" = 42}"#]],
        str![[r#"
6..30

"#]],
    );
    // array
    good::<Vec<u32>>(
        "foo = [0, 1, 2, 3, 4]",
        str!["[0, 1, 2, 3, 4]"],
        str![[r#"
6..21

"#]],
    );
    // datetime
    good::<String>(
        "foo = \"1997-09-09T09:09:09Z\"",
        str![[r#""1997-09-09T09:09:09Z""#]],
        str![[r#"
6..28

"#]],
    );

    let good_datetimes = [
        (
            "1997-09-09T09:09:09Z",
            str!["1997-09-09T09:09:09Z"],
            str![[r#"
6..26

"#]],
        ),
        (
            "1997-09-09T09:09:09+09:09",
            str!["1997-09-09T09:09:09+09:09"],
            str![[r#"
6..31

"#]],
        ),
        (
            "1997-09-09T09:09:09-09:09",
            str!["1997-09-09T09:09:09-09:09"],
            str![[r#"
6..31

"#]],
        ),
        (
            "1997-09-09T09:09:09",
            str!["1997-09-09T09:09:09"],
            str![[r#"
6..25

"#]],
        ),
        (
            "1997-09-09",
            str!["1997-09-09"],
            str![[r#"
6..16

"#]],
        ),
        (
            "09:09:09",
            str!["09:09:09"],
            str![[r#"
6..14

"#]],
        ),
        (
            "1997-09-09T09:09:09.09Z",
            str!["1997-09-09T09:09:09.09Z"],
            str![[r#"
6..29

"#]],
        ),
        (
            "1997-09-09T09:09:09.09+09:09",
            str!["1997-09-09T09:09:09.09+09:09"],
            str![[r#"
6..34

"#]],
        ),
        (
            "1997-09-09T09:09:09.09-09:09",
            str!["1997-09-09T09:09:09.09-09:09"],
            str![[r#"
6..34

"#]],
        ),
        (
            "1997-09-09T09:09:09.09",
            str!["1997-09-09T09:09:09.09"],
            str![[r#"
6..28

"#]],
        ),
        (
            "09:09:09.09",
            str!["09:09:09.09"],
            str![[r#"
6..17

"#]],
        ),
    ];
    for (value, expected, span) in good_datetimes {
        let input = format!("foo = {value}");
        good::<Datetime>(&input, expected, span);
    }
    // ending at something other than the absolute end
    good::<u32>(
        "foo = 42\nnoise = true",
        str!["42"],
        str![[r#"
6..8

"#]],
    );
}

#[test]
fn test_inner_spanned_table() {
    #[derive(Deserialize, Debug)]
    struct Foo {
        foo: Spanned<BTreeMap<Spanned<String>, Spanned<String>>>,
    }

    #[track_caller]
    fn good(input: &str, zero: bool) {
        dbg!(input);
        let foo: Foo = crate::from_str(input).unwrap();
        dbg!(&foo);

        if zero {
            assert_eq!(foo.foo.span().start, 0, "invalid `foo.foo.span().start`");
            assert_eq!(foo.foo.span().end, 5, "invalid `foo.foo.span().end`");
        } else {
            assert_eq!(
                foo.foo.span().start,
                input.find('{').unwrap(),
                "invalid `foo.foo.span().start`"
            );
            assert_eq!(
                foo.foo.span().end,
                input.find('}').unwrap() + 1,
                "invalid `foo.foo.span().end`"
            );
        }
        for (k, v) in foo.foo.as_ref().iter() {
            dbg!(&k);
            dbg!(&v);
            assert_eq!(
                &input[k.span().start..k.span().end],
                k.as_ref(),
                "invalid key"
            );
            assert_eq!(
                &input[(v.span().start + 1)..(v.span().end - 1)],
                v.as_ref(),
                "invalid value"
            );
        }
    }

    good(
        "\
        [foo]
        a = 'b'
        bar = 'baz'
        c = 'd'
        e = \"f\"
    ",
        true,
    );

    good(
        "
        foo = { a = 'b', bar = 'baz', c = 'd', e = \"f\" }",
        false,
    );
}

#[test]
fn test_outer_spanned_table() {
    #[derive(Debug, Deserialize)]
    struct Foo {
        foo: BTreeMap<Spanned<String>, Spanned<String>>,
    }

    fn good(s: &str, foo: &Foo) {
        for (k, v) in foo.foo.iter() {
            assert_eq!(&s[k.span().start..k.span().end], k.as_ref());
            assert_eq!(&s[(v.span().start + 1)..(v.span().end - 1)], v.as_ref());
        }
    }

    let input = "
        [foo]
        a = 'b'
        bar = 'baz'
        c = 'd'
        e = \"f\"
    ";
    let foo: Foo = crate::from_str(input).unwrap();
    assert_data_eq!(
        foo.to_debug(),
        str![[r#"
Foo {
    foo: {
        Spanned {
            span: 23..24,
            value: "a",
        }: Spanned {
            span: 27..30,
            value: "b",
        },
        Spanned {
            span: 39..42,
            value: "bar",
        }: Spanned {
            span: 45..50,
            value: "baz",
        },
        Spanned {
            span: 59..60,
            value: "c",
        }: Spanned {
            span: 63..66,
            value: "d",
        },
        Spanned {
            span: 75..76,
            value: "e",
        }: Spanned {
            span: 79..82,
            value: "f",
        },
    },
}

"#]]
    );
    good(input, &foo);

    let input = "
        foo = { a = 'b', bar = 'baz', c = 'd', e = \"f\" }
    ";
    let foo: Foo = crate::from_str(input).unwrap();
    assert_data_eq!(
        foo.to_debug(),
        str![[r#"
Foo {
    foo: {
        Spanned {
            span: 17..18,
            value: "a",
        }: Spanned {
            span: 21..24,
            value: "b",
        },
        Spanned {
            span: 26..29,
            value: "bar",
        }: Spanned {
            span: 32..37,
            value: "baz",
        },
        Spanned {
            span: 39..40,
            value: "c",
        }: Spanned {
            span: 43..46,
            value: "d",
        },
        Spanned {
            span: 48..49,
            value: "e",
        }: Spanned {
            span: 52..55,
            value: "f",
        },
    },
}

"#]]
    );
    good(input, &foo);
}

#[test]
fn test_spanned_nested() {
    #[derive(Debug, Deserialize)]
    struct Foo {
        foo: BTreeMap<Spanned<String>, BTreeMap<Spanned<String>, Spanned<String>>>,
    }

    fn good(s: &str, foo: &Foo) {
        for (k, v) in foo.foo.iter() {
            assert_eq!(&s[k.span().start..k.span().end], k.as_ref());
            for (n_k, n_v) in v.iter() {
                assert_eq!(&s[n_k.span().start..n_k.span().end], n_k.as_ref());
                assert_eq!(
                    &s[(n_v.span().start + 1)..(n_v.span().end - 1)],
                    n_v.as_ref()
                );
            }
        }
    }

    let input = "
        [foo.a]
        a = 'b'
        c = 'd'
        e = \"f\"
        [foo.bar]
        baz = 'true'
    ";
    let foo: Foo = crate::from_str(input).unwrap();
    assert_data_eq!(
        foo.to_debug(),
        str![[r#"
Foo {
    foo: {
        Spanned {
            span: 14..15,
            value: "a",
        }: {
            Spanned {
                span: 25..26,
                value: "a",
            }: Spanned {
                span: 29..32,
                value: "b",
            },
            Spanned {
                span: 41..42,
                value: "c",
            }: Spanned {
                span: 45..48,
                value: "d",
            },
            Spanned {
                span: 57..58,
                value: "e",
            }: Spanned {
                span: 61..64,
                value: "f",
            },
        },
        Spanned {
            span: 78..81,
            value: "bar",
        }: {
            Spanned {
                span: 91..94,
                value: "baz",
            }: Spanned {
                span: 97..103,
                value: "true",
            },
        },
    },
}

"#]]
    );
    good(input, &foo);

    let input = "
        [foo]
        foo = { a = 'b', bar = 'baz', c = 'd', e = \"f\" }
        bazz = {}
        g = { h = 'i' }
    ";
    let foo: Foo = crate::from_str(input).unwrap();
    assert_data_eq!(
        foo.to_debug(),
        str![[r#"
Foo {
    foo: {
        Spanned {
            span: 80..84,
            value: "bazz",
        }: {},
        Spanned {
            span: 23..26,
            value: "foo",
        }: {
            Spanned {
                span: 31..32,
                value: "a",
            }: Spanned {
                span: 35..38,
                value: "b",
            },
            Spanned {
                span: 40..43,
                value: "bar",
            }: Spanned {
                span: 46..51,
                value: "baz",
            },
            Spanned {
                span: 53..54,
                value: "c",
            }: Spanned {
                span: 57..60,
                value: "d",
            },
            Spanned {
                span: 62..63,
                value: "e",
            }: Spanned {
                span: 66..69,
                value: "f",
            },
        },
        Spanned {
            span: 98..99,
            value: "g",
        }: {
            Spanned {
                span: 104..105,
                value: "h",
            }: Spanned {
                span: 108..111,
                value: "i",
            },
        },
    },
}

"#]]
    );
    good(input, &foo);
}

#[test]
fn test_spanned_array() {
    #[derive(Debug, Deserialize)]
    struct Foo {
        foo: Vec<Spanned<BTreeMap<Spanned<String>, Spanned<String>>>>,
    }

    let toml = "\
        [[foo]]
        a = 'b'
        bar = 'baz'
        c = 'd'
        e = \"f\"
        [[foo]]
        a = 'c'
        bar = 'baz'
        c = 'g'
        e = \"h\"
    ";
    let foo_list: Foo = crate::from_str(toml).unwrap();
    assert_data_eq!(
        foo_list.to_debug(),
        str![[r#"
Foo {
    foo: [
        Spanned {
            span: 0..7,
            value: {
                Spanned {
                    span: 16..17,
                    value: "a",
                }: Spanned {
                    span: 20..23,
                    value: "b",
                },
                Spanned {
                    span: 32..35,
                    value: "bar",
                }: Spanned {
                    span: 38..43,
                    value: "baz",
                },
                Spanned {
                    span: 52..53,
                    value: "c",
                }: Spanned {
                    span: 56..59,
                    value: "d",
                },
                Spanned {
                    span: 68..69,
                    value: "e",
                }: Spanned {
                    span: 72..75,
                    value: "f",
                },
            },
        },
        Spanned {
            span: 84..91,
            value: {
                Spanned {
                    span: 100..101,
                    value: "a",
                }: Spanned {
                    span: 104..107,
                    value: "c",
                },
                Spanned {
                    span: 116..119,
                    value: "bar",
                }: Spanned {
                    span: 122..127,
                    value: "baz",
                },
                Spanned {
                    span: 136..137,
                    value: "c",
                }: Spanned {
                    span: 140..143,
                    value: "g",
                },
                Spanned {
                    span: 152..153,
                    value: "e",
                }: Spanned {
                    span: 156..159,
                    value: "h",
                },
            },
        },
    ],
}

"#]]
    );

    for (foo, expected) in foo_list.foo.iter().zip([0..7, 84..91]) {
        assert_eq!(foo.span(), expected);
        for (k, v) in foo.as_ref().iter() {
            assert_eq!(&toml[k.span().start..k.span().end], k.as_ref());
            assert_eq!(&toml[(v.span().start + 1)..(v.span().end - 1)], v.as_ref());
        }
    }
}

#[test]
fn implicit_tables() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum SpannedValue {
        String(String),
        Map(Vec<(String, Spanned<Self>)>),
    }

    impl<'de> Deserialize<'de> for SpannedValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let data = UntaggedEnumVisitor::new()
                .string(|str| Ok(Self::String(str.into())))
                .map(|mut map| {
                    let mut result = Vec::new();

                    while let Some((k, v)) = map.next_entry()? {
                        result.push((k, v));
                    }

                    Ok(Self::Map(result))
                })
                .deserialize(deserializer)?;

            Ok(data)
        }
    }

    const INPUT: &str = r#"
[foo.bar]
alice.bob = { one.two = "qux" }
"#;

    let result = crate::from_str::<SpannedValue>(INPUT);
    assert_data_eq!(
        result.unwrap().to_debug(),
        str![[r#"
Map(
    [
        (
            "foo",
            Spanned {
                span: 2..5,
                value: Map(
                    [
                        (
                            "bar",
                            Spanned {
                                span: 1..10,
                                value: Map(
                                    [
                                        (
                                            "alice",
                                            Spanned {
                                                span: 11..16,
                                                value: Map(
                                                    [
                                                        (
                                                            "bob",
                                                            Spanned {
                                                                span: 23..42,
                                                                value: Map(
                                                                    [
                                                                        (
                                                                            "one",
                                                                            Spanned {
                                                                                span: 25..28,
                                                                                value: Map(
                                                                                    [
                                                                                        (
                                                                                            "two",
                                                                                            Spanned {
                                                                                                span: 35..40,
                                                                                                value: String(
                                                                                                    "qux",
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ],
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                ),
                                            },
                                        ),
                                    ],
                                ),
                            },
                        ),
                    ],
                ),
            },
        ),
    ],
)

"#]]
    );
}

#[test]
fn deny_unknown_fields() {
    #[derive(Debug, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Example {
        #[allow(dead_code)]
        real: u32,
    }

    let error = crate::from_str::<Example>(
        r#"# my comment
# bla bla bla
fake = 1"#,
    )
    .unwrap_err();
    assert_data_eq!(
        error.to_string(),
        str![[r#"
TOML parse error at line 3, column 1
  |
3 | fake = 1
  | ^^^^
unknown field `fake`, expected `real`

"#]]
        .raw()
    );
}
