extern crate toml;

use std::str::FromStr;

macro_rules! bad {
    ($toml:expr, $msg:expr) => {
        match $toml.parse::<toml::Value>() {
            Ok(s) => panic!("parsed to: {:#?}", s),
            Err(e) => assert_eq!(e.to_string(), $msg),
        }
    };
}

#[test]
fn times() {
    fn dogood(s: &str, serialized: &str) {
        let to_parse = format!("foo = {}", s);
        let value = toml::Value::from_str(&to_parse).unwrap();
        assert_eq!(value["foo"].as_datetime().unwrap().to_string(), serialized);
    }
    fn good(s: &str) {
        dogood(s, s);
        dogood(&s.replace("T", " "), s);
        dogood(&s.replace("T", "t"), s);
        dogood(&s.replace("Z", "z"), s);
    }

    good("1997-09-09T09:09:09Z");
    good("1997-09-09T09:09:09+09:09");
    good("1997-09-09T09:09:09-09:09");
    good("1997-09-09T09:09:09");
    good("1997-09-09");
    dogood("1997-09-09 ", "1997-09-09");
    dogood("1997-09-09 # comment", "1997-09-09");
    good("09:09:09");
    good("1997-09-09T09:09:09.09Z");
    good("1997-09-09T09:09:09.09+09:09");
    good("1997-09-09T09:09:09.09-09:09");
    good("1997-09-09T09:09:09.09");
    good("09:09:09.09");
}

#[test]
fn bad_times() {
    bad!(
        "foo = 199-09-09",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 199709-09",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-9-09",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-9",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-0909:09:09",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.",
        "invalid date at line 1 column 7"
    );
    bad!(
        "foo = T",
        "invalid TOML value, did you mean to use a quoted string? at line 1 column 7"
    );
    bad!(
        "foo = T.",
        "invalid TOML value, did you mean to use a quoted string? at line 1 column 7"
    );
    bad!(
        "foo = TZ",
        "invalid TOML value, did you mean to use a quoted string? at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.09+",
        "invalid date at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.09+09",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.09+09:9",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.09+0909",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.09-",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.09-09",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.09-09:9",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T09:09:09.09-0909",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );

    bad!(
        "foo = 1997-00-09T09:09:09.09Z",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-00T09:09:09.09Z",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T30:09:09.09Z",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T12:69:09.09Z",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
    bad!(
        "foo = 1997-09-09T12:09:69.09Z",
        "failed to parse datetime for key `foo` at line 1 column 7"
    );
}
