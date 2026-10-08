#![cfg(feature = "parse")]

#[test]
fn explicit_table_preserves_children_and_recovery_metadata() {
    let input = "[a.child]\nx = 1\n[z]\nx = 2\n['a']\nvalue = 1\n['a']\nvalue = 2\n";
    let (table, errors) = toml::de::DeTable::parse_recoverable(input);
    let headers: Vec<_> = input
        .match_indices("'a'")
        .map(|(pos, _)| pos..pos + 3)
        .collect();
    let values: Vec<_> = input
        .match_indices("value")
        .map(|(pos, _)| pos..pos + 5)
        .collect();
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].message(), "duplicate key");
    assert_eq!(errors[0].span(), Some(headers[1].clone()));
    assert_eq!(errors[1].message(), "duplicate key");
    assert_eq!(errors[1].span(), Some(values[1].clone()));

    let table = table.into_inner();
    let (key, value) = table
        .iter()
        .find(|(key, _)| key.get_ref().as_ref() == "a")
        .unwrap();
    assert_eq!(key.span(), headers[1]);
    let table = value.get_ref().as_table().unwrap();
    assert!(
        table
            .iter()
            .any(|(key, _)| key.get_ref().as_ref() == "child")
    );
    assert!(
        table
            .iter()
            .any(|(key, _)| key.get_ref().as_ref() == "value")
    );
}

#[cfg(feature = "preserve_order")]
#[test]
fn explicit_implicit_table_moves_to_end() {
    let input = "[a.child]\nx = 1\n[z]\nx = 2\n['a']\nvalue = 1\n";
    let table = toml::de::DeTable::parse(input).unwrap().into_inner();
    let keys: Vec<_> = table.keys().map(|key| key.get_ref().as_ref()).collect();
    assert_eq!(keys, ["z", "a"]);
}
