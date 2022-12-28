#[test]
fn basic_spans() {
    let toml = r#"
key1 = "value"
"#;
    let doc = toml.parse::<toml_edit::Document>();
    assert!(doc.is_ok());
    let doc = doc.unwrap();

    let (key, value) = doc.as_table().get_key_value("key1").unwrap();
    assert_eq!(&toml[key.span().unwrap()], r#"key1"#);
    assert_eq!(&toml[value.span().unwrap()], r#""value""#);
}
