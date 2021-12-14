#![cfg(feature = "easy")]

pub struct Encoder;

impl toml_test_harness::Encoder for Encoder {
    fn name(&self) -> &str {
        "toml_edit::easy"
    }

    fn encode(&self, data: toml_test_harness::Decoded) -> Result<String, toml_test_harness::Error> {
        let value = from_decoded(&data)?;
        let s = toml_edit::easy::to_string(&value).map_err(toml_test_harness::Error::new)?;
        Ok(s)
    }
}

fn from_decoded(
    decoded: &toml_test_harness::Decoded,
) -> Result<toml_edit::easy::Value, toml_test_harness::Error> {
    let value = match decoded {
        toml_test_harness::Decoded::Value(value) => from_decoded_value(value)?,
        toml_test_harness::Decoded::Table(value) => {
            toml_edit::easy::Value::Table(from_table(value)?)
        }
        toml_test_harness::Decoded::Array(value) => {
            toml_edit::easy::Value::Array(from_array(value)?)
        }
    };
    Ok(value)
}

fn from_decoded_value(
    decoded: &toml_test_harness::DecodedValue,
) -> Result<toml_edit::easy::Value, toml_test_harness::Error> {
    match decoded {
        toml_test_harness::DecodedValue::String(value) => {
            Ok(toml_edit::easy::Value::String(value.clone()))
        }
        toml_test_harness::DecodedValue::Integer(value) => value
            .parse::<i64>()
            .map_err(toml_test_harness::Error::new)
            .map(toml_edit::easy::Value::Integer),
        toml_test_harness::DecodedValue::Float(value) => value
            .parse::<f64>()
            .map_err(toml_test_harness::Error::new)
            .map(toml_edit::easy::Value::Float),
        toml_test_harness::DecodedValue::Bool(value) => value
            .parse::<bool>()
            .map_err(toml_test_harness::Error::new)
            .map(toml_edit::easy::Value::Boolean),
        toml_test_harness::DecodedValue::Datetime(value) => value
            .parse::<toml_edit::Datetime>()
            .map_err(toml_test_harness::Error::new)
            .map(toml_edit::easy::Value::Datetime),
        toml_test_harness::DecodedValue::DatetimeLocal(value) => value
            .parse::<toml_edit::Datetime>()
            .map_err(toml_test_harness::Error::new)
            .map(toml_edit::easy::Value::Datetime),
        toml_test_harness::DecodedValue::DateLocal(value) => value
            .parse::<toml_edit::Datetime>()
            .map_err(toml_test_harness::Error::new)
            .map(toml_edit::easy::Value::Datetime),
        toml_test_harness::DecodedValue::TimeLocal(value) => value
            .parse::<toml_edit::Datetime>()
            .map_err(toml_test_harness::Error::new)
            .map(toml_edit::easy::Value::Datetime),
    }
}

fn from_table(
    decoded: &std::collections::HashMap<String, toml_test_harness::Decoded>,
) -> Result<toml_edit::easy::value::Table, toml_test_harness::Error> {
    decoded
        .iter()
        .map(|(k, v)| {
            let v = from_decoded(v)?;
            Ok((k.to_owned(), v))
        })
        .collect()
}

fn from_array(
    decoded: &[toml_test_harness::Decoded],
) -> Result<toml_edit::easy::value::Array, toml_test_harness::Error> {
    decoded.iter().map(from_decoded).collect()
}
