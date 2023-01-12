use std::collections::HashMap;
use std::fmt::Debug;

use serde_derive::Deserialize;
use toml::value::Datetime;
use toml::Spanned;

/// A set of good datetimes.
pub fn good_datetimes() -> Vec<&'static str> {
    vec![
        "1997-09-09T09:09:09Z",
        "1997-09-09T09:09:09+09:09",
        "1997-09-09T09:09:09-09:09",
        "1997-09-09T09:09:09",
        "1997-09-09",
        "09:09:09",
        "1997-09-09T09:09:09.09Z",
        "1997-09-09T09:09:09.09+09:09",
        "1997-09-09T09:09:09.09-09:09",
        "1997-09-09T09:09:09.09",
        "09:09:09.09",
    ]
}

#[test]
fn test_spanned_field() {
    #[derive(Deserialize)]
    struct Foo<T> {
        foo: Spanned<T>,
    }

    #[derive(Deserialize)]
    struct BareFoo<T> {
        foo: T,
    }

    fn good<'de, T>(s: &'de str, expected: &str, end: Option<usize>)
    where
        T: serde::Deserialize<'de> + Debug + PartialEq,
    {
        let foo: Foo<T> = toml::from_str(s).unwrap();

        assert_eq!(6, foo.foo.span().start);
        if let Some(end) = end {
            assert_eq!(end, foo.foo.span().end);
        } else {
            assert_eq!(s.len(), foo.foo.span().end);
        }
        assert_eq!(expected, &s[foo.foo.span()]);

        // Test for Spanned<> at the top level
        let foo_outer: Spanned<BareFoo<T>> = toml::from_str(s).unwrap();

        assert_eq!(0, foo_outer.span().start);
        assert_eq!(s.len(), foo_outer.span().end);
        assert_eq!(foo.foo.into_inner(), foo_outer.into_inner().foo);
    }

    good::<String>("foo = \"foo\"", "\"foo\"", None);
    good::<u32>("foo = 42", "42", None);
    // leading plus
    good::<u32>("foo = +42", "+42", None);
    // table
    good::<HashMap<String, u32>>(
        "foo = {\"foo\" = 42, \"bar\" = 42}",
        "{\"foo\" = 42, \"bar\" = 42}",
        None,
    );
    // array
    good::<Vec<u32>>("foo = [0, 1, 2, 3, 4]", "[0, 1, 2, 3, 4]", None);
    // datetime
    good::<String>(
        "foo = \"1997-09-09T09:09:09Z\"",
        "\"1997-09-09T09:09:09Z\"",
        None,
    );

    for expected in good_datetimes() {
        let s = format!("foo = {}", expected);
        good::<Datetime>(&s, expected, None);
    }
    // ending at something other than the absolute end
    good::<u32>("foo = 42\nnoise = true", "42", Some(8));
}

#[test]
fn test_inner_spanned_table() {
    #[derive(Deserialize)]
    struct Foo {
        foo: Spanned<HashMap<Spanned<String>, Spanned<String>>>,
    }

    fn good(s: &str, zero: bool) {
        let foo: Foo = toml::from_str(s).unwrap();

        if zero {
            assert_eq!(foo.foo.span().start, 0);
            // We'd actually have to assert equality with s.len() here,
            // but the current implementation doesn't support that,
            // and it's not possible with toml's data format to support it
            // in the general case as spans aren't always well-defined.
            // So this check mainly serves as a reminder that this test should
            // be updated *if* one day there is support for emitting the actual span.
            assert_eq!(foo.foo.span().end, 0);
        } else {
            assert_eq!(foo.foo.span().start, s.find('{').unwrap());
            assert_eq!(foo.foo.span().end, s.find('}').unwrap() + 1);
        }
        for (k, v) in foo.foo.as_ref().iter() {
            assert_eq!(&s[k.span().start..k.span().end], k.as_ref());
            assert_eq!(&s[(v.span().start + 1)..(v.span().end - 1)], v.as_ref());
        }
    }

    good(
        "
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
    #[derive(Deserialize)]
    struct Foo {
        foo: HashMap<Spanned<String>, Spanned<String>>,
    }

    fn good(s: &str) {
        let foo: Foo = toml::from_str(s).unwrap();

        for (k, v) in foo.foo.iter() {
            assert_eq!(&s[k.span().start..k.span().end], k.as_ref());
            assert_eq!(&s[(v.span().start + 1)..(v.span().end - 1)], v.as_ref());
        }
    }

    good(
        "
        [foo]
        a = 'b'
        bar = 'baz'
        c = 'd'
        e = \"f\"
    ",
    );

    good(
        "
        foo = { a = 'b', bar = 'baz', c = 'd', e = \"f\" }
    ",
    );
}

#[test]
fn test_spanned_nested() {
    #[derive(Deserialize)]
    struct Foo {
        foo: HashMap<Spanned<String>, HashMap<Spanned<String>, Spanned<String>>>,
    }

    fn good(s: &str) {
        let foo: Foo = toml::from_str(s).unwrap();

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

    good(
        "
        [foo.a]
        a = 'b'
        c = 'd'
        e = \"f\"
        [foo.bar]
        baz = 'true'
    ",
    );

    good(
        "
        [foo]
        foo = { a = 'b', bar = 'baz', c = 'd', e = \"f\" }
        bazz = {}
        g = { h = 'i' }
    ",
    );
}

#[test]
fn test_spanned_array() {
    #[derive(Deserialize)]
    struct Foo {
        foo: Vec<Spanned<HashMap<Spanned<String>, Spanned<String>>>>,
    }

    fn good(s: &str) {
        let foo_list: Foo = toml::from_str(s).unwrap();

        for foo in foo_list.foo.iter() {
            assert_eq!(foo.span().start, 0);
            // We'd actually have to assert equality with s.len() here,
            // but the current implementation doesn't support that,
            // and it's not possible with toml's data format to support it
            // in the general case as spans aren't always well-defined.
            // So this check mainly serves as a reminder that this test should
            // be updated *if* one day there is support for emitting the actual span.
            assert_eq!(foo.span().end, 0);
            for (k, v) in foo.as_ref().iter() {
                assert_eq!(&s[k.span().start..k.span().end], k.as_ref());
                assert_eq!(&s[(v.span().start + 1)..(v.span().end - 1)], v.as_ref());
            }
        }
    }

    good(
        "
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
    ",
    );
}
