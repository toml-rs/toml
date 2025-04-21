#[derive(Copy, Clone)]
pub(crate) struct Encoder;

impl toml_test_harness::Encoder for Encoder {
    fn name(&self) -> &str {
        "toml_edit"
    }

    fn encode(
        &self,
        data: toml_test_harness::DecodedValue,
    ) -> Result<String, toml_test_harness::Error> {
        let doc = decoded_to_document(&data)?;
        Ok(doc.to_string())
    }
}

fn decoded_to_document(
    decoded: &toml_test_harness::DecodedValue,
) -> Result<toml_edit::DocumentMut, toml_test_harness::Error> {
    let item = root_from_decoded(decoded)?;
    let mut doc = toml_edit::DocumentMut::new();
    *doc = item;
    Ok(doc)
}

fn root_from_decoded(
    decoded: &toml_test_harness::DecodedValue,
) -> Result<toml_edit::Table, toml_test_harness::Error> {
    match decoded {
        toml_test_harness::DecodedValue::Scalar(_) => {
            Err(toml_test_harness::Error::new("Root cannot be a value"))
        }
        toml_test_harness::DecodedValue::Table(value) => value
            .iter()
            .map(|(k, v)| {
                let k = k.as_str();
                let v = from_decoded(v)?;
                Ok((k, v))
            })
            .collect(),
        toml_test_harness::DecodedValue::Array(_) => {
            Err(toml_test_harness::Error::new("Root cannot be an array"))
        }
    }
}

fn from_decoded(
    decoded: &toml_test_harness::DecodedValue,
) -> Result<toml_edit::Value, toml_test_harness::Error> {
    let value = match decoded {
        toml_test_harness::DecodedValue::Scalar(value) => from_decoded_scalar(value)?,
        toml_test_harness::DecodedValue::Table(value) => {
            toml_edit::Value::InlineTable(from_table(value)?)
        }
        toml_test_harness::DecodedValue::Array(value) => {
            toml_edit::Value::Array(from_array(value)?)
        }
    };
    Ok(value)
}

fn from_decoded_scalar(
    decoded: &toml_test_harness::DecodedScalar,
) -> Result<toml_edit::Value, toml_test_harness::Error> {
    let value: toml_edit::Value = match decoded {
        toml_test_harness::DecodedScalar::String(value) => value.into(),
        toml_test_harness::DecodedScalar::Integer(value) => value
            .parse::<i64>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedScalar::Float(value) => value
            .parse::<f64>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedScalar::Bool(value) => value
            .parse::<bool>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedScalar::Datetime(value) => value
            .parse::<toml_edit::Datetime>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedScalar::DatetimeLocal(value) => value
            .parse::<toml_edit::Datetime>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedScalar::DateLocal(value) => value
            .parse::<toml_edit::Datetime>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
        toml_test_harness::DecodedScalar::TimeLocal(value) => value
            .parse::<toml_edit::Datetime>()
            .map_err(toml_test_harness::Error::new)?
            .into(),
    };
    Ok(value)
}

fn from_table(
    decoded: &std::collections::HashMap<String, toml_test_harness::DecodedValue>,
) -> Result<toml_edit::InlineTable, toml_test_harness::Error> {
    decoded
        .iter()
        .map(|(k, v)| {
            let v = from_decoded(v)?;
            Ok((k, v))
        })
        .collect()
}

fn from_array(
    decoded: &[toml_test_harness::DecodedValue],
) -> Result<toml_edit::Array, toml_test_harness::Error> {
    decoded.iter().map(from_decoded).collect()
}
