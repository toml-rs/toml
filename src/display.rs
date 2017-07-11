use std::fmt::{Display, Formatter, Result};
use value::{Array, DateTime, InlineTable, KeyValue, KeyValuePairs, Value};
use decor::{Formatted, Repr};
use document::Document;
use table::{Header, HeaderKind, Table};

impl Display for Repr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}{}{}",
            self.decor.prefix,
            self.raw_value,
            self.decor.suffix
        )
    }
}

impl<T> Display for Formatted<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.repr)
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            DateTime::OffsetDateTime(d) => write!(f, "{}", d),
            DateTime::LocalDateTime(d) => write!(f, "{}", d),
            DateTime::LocalDate(d) => write!(f, "{}", d),
            DateTime::LocalTime(d) => write!(f, "{}", d),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Value::Integer(ref repr) => write!(f, "{}", repr),
            Value::String(ref repr) => write!(f, "{}", repr),
            Value::Float(ref repr) => write!(f, "{}", repr),
            Value::Boolean(ref repr) => write!(f, "{}", repr),
            Value::DateTime(ref repr) => write!(f, "{}", repr),
            Value::Array(ref array) => write!(f, "{}", array),
            Value::InlineTable(ref table) => write!(f, "{}", table),
        }
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}[", self.decor.prefix)?;
        join(f, self.values.iter(), ",")?;
        write!(f, "{}", self.trailing)?;
        write!(f, "]{}", self.decor.suffix)
    }
}

impl Display for KeyValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl Display for InlineTable {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{{", self.decor.prefix)?;
        write!(f, "{}", self.preamble)?;
        join(f, self.key_value_pairs.values(), ",")?;
        write!(f, "}}{}", self.decor.suffix)
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.kind == HeaderKind::Implicit {
            return Ok(());
        }
        let brackets = if self.kind == HeaderKind::Standard {
            ["[", "]"]
        } else {
            ["[[", "]]"]
        };
        write!(f, "{}{}", self.repr.decor.prefix, brackets[0])?;
        write!(f, "{}", self.repr.raw_value)?;
        write!(f, "{}{}", brackets[1], self.repr.decor.suffix)
    }
}

/// **Note**: It only displays Key/Value Pairs
impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.header.kind != HeaderKind::Implicit {
            write!(f, "{}", self.header)?;
            display_table_values(f, &self.key_value_pairs)?;
        }
        Ok(())
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, c) in self.inner.list.iter().enumerate() {
            if i > 0 {
                write!(f, "{}", c)?;
            } else {
                // root table
                display_table_values(f, &c.key_value_pairs)?;
            }
        }
        write!(f, "{}", self.trailing)
    }
}

fn join<D, I>(f: &mut Formatter, iter: I, sep: &str) -> Result
where
    D: Display,
    I: Iterator<Item = D>,
{
    for (i, v) in iter.enumerate() {
        if i > 0 {
            write!(f, "{}", sep)?;
        }
        write!(f, "{}", v)?;
    }
    Ok(())
}

fn display_table_values(f: &mut Formatter, key_value_pairs: &KeyValuePairs) -> Result {
    join(f, key_value_pairs.values(), "")
}
