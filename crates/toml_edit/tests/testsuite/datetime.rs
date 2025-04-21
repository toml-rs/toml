#[test]
fn times() {
    fn dogood(s: &str, serialized: &str) {
        let to_parse = format!("foo = {s}");
        let document = to_parse.parse::<toml_edit::DocumentMut>().unwrap();
        assert_eq!(
            document["foo"].as_datetime().unwrap().to_string(),
            serialized
        );
    }
    fn good(s: &str) {
        dogood(s, s);
        dogood(&s.replace('T', " "), s);
        dogood(&s.replace('T', "t"), s);
        dogood(&s.replace('Z', "z"), s);
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
