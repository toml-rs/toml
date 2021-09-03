pub struct Decoder;

impl toml_test_harness::Decoder for Decoder {
    fn name(&self) -> &str {
        "toml_edit"
    }

    fn decode(&self, data: &[u8]) -> Result<toml_test_harness::Decoded, toml_test_harness::Error> {
        let data = std::str::from_utf8(data).map_err(toml_test_harness::Error::new)?;
        let document = data
            .parse::<toml_edit::Document>()
            .map_err(toml_test_harness::Error::new)?;
        document_to_decoded(&document)
    }
}

fn document_to_decoded(
    value: &toml_edit::Document,
) -> Result<toml_test_harness::Decoded, toml_test_harness::Error> {
    item_to_decoded(&value.root)
}

fn item_to_decoded(
    value: &toml_edit::Item,
) -> Result<toml_test_harness::Decoded, toml_test_harness::Error> {
    match value {
        toml_edit::Item::None => unreachable!("No nones"),
        toml_edit::Item::Value(v) => value_to_decoded(v),
        toml_edit::Item::Table(v) => table_to_decoded(v),
        toml_edit::Item::ArrayOfTables(v) => {
            let v: Result<_, toml_test_harness::Error> = v.iter().map(table_to_decoded).collect();
            Ok(toml_test_harness::Decoded::Array(v?))
        }
    }
}

fn value_to_decoded(
    value: &toml_edit::Value,
) -> Result<toml_test_harness::Decoded, toml_test_harness::Error> {
    match value {
        toml_edit::Value::Integer(v) => Ok(toml_test_harness::Decoded::Value(
            toml_test_harness::DecodedValue::from(*v.value()),
        )),
        toml_edit::Value::String(v) => Ok(toml_test_harness::Decoded::Value(
            toml_test_harness::DecodedValue::from(v.value()),
        )),
        toml_edit::Value::Float(v) => Ok(toml_test_harness::Decoded::Value(
            toml_test_harness::DecodedValue::from(*v.value()),
        )),
        toml_edit::Value::OffsetDateTime(v) => Ok(toml_test_harness::Decoded::Value(
            toml_test_harness::DecodedValue::Datetime(v.value().to_string()),
        )),
        toml_edit::Value::LocalDateTime(v) => Ok(toml_test_harness::Decoded::Value(
            toml_test_harness::DecodedValue::DatetimeLocal(v.value().to_string()),
        )),
        toml_edit::Value::LocalDate(v) => Ok(toml_test_harness::Decoded::Value(
            toml_test_harness::DecodedValue::DateLocal(v.value().to_string()),
        )),
        toml_edit::Value::LocalTime(v) => Ok(toml_test_harness::Decoded::Value(
            toml_test_harness::DecodedValue::TimeLocal(v.value().to_string()),
        )),
        toml_edit::Value::Boolean(v) => Ok(toml_test_harness::Decoded::Value(
            toml_test_harness::DecodedValue::from(*v.value()),
        )),
        toml_edit::Value::Array(v) => {
            let v: Result<_, toml_test_harness::Error> = v.iter().map(value_to_decoded).collect();
            Ok(toml_test_harness::Decoded::Array(v?))
        }
        toml_edit::Value::InlineTable(v) => inline_table_to_decoded(v),
    }
}

fn table_to_decoded(
    value: &toml_edit::Table,
) -> Result<toml_test_harness::Decoded, toml_test_harness::Error> {
    let table: Result<_, toml_test_harness::Error> = value
        .iter()
        .map(|(k, v)| {
            let k = k.to_owned();
            let v = item_to_decoded(v)?;
            Ok((k, v))
        })
        .collect();
    Ok(toml_test_harness::Decoded::Table(table?))
}

fn inline_table_to_decoded(
    value: &toml_edit::InlineTable,
) -> Result<toml_test_harness::Decoded, toml_test_harness::Error> {
    let table: Result<_, toml_test_harness::Error> = value
        .iter()
        .map(|(k, v)| {
            let k = k.to_owned();
            let v = value_to_decoded(v)?;
            Ok((k, v))
        })
        .collect();
    Ok(toml_test_harness::Decoded::Table(table?))
}
