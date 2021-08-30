use std::fmt::{Display, Formatter, Result, Write};

use crate::document::Document;
use crate::inline_table::DEFAULT_INLINE_KEY_DECOR;
use crate::key::Key;
use crate::repr::{DecorDisplay, Formatted, Repr};
use crate::table::{DEFAULT_KEY_DECOR, DEFAULT_TABLE_DECOR};
use crate::value::DEFAULT_VALUE_DECOR;
use crate::{Array, InlineTable, Item, Table, Value};

impl<'d, D: Display> Display for DecorDisplay<'d, D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}{}{}",
            self.decor.prefix().unwrap_or(self.default.0),
            self.inner,
            self.decor.suffix().unwrap_or(self.default.1)
        )
    }
}

impl Display for Repr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.as_raw().fmt(f)
    }
}

impl<T> Display for Formatted<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.decor.display(&self.repr, DEFAULT_VALUE_DECOR))
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.repr().fmt(f)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Value::String(ref repr) => write!(f, "{}", repr),
            Value::Integer(ref repr) => write!(f, "{}", repr),
            Value::Float(ref repr) => write!(f, "{}", repr),
            Value::Boolean(ref repr) => write!(f, "{}", repr),
            Value::OffsetDateTime(ref repr) => write!(f, "{}", repr),
            Value::LocalDateTime(ref repr) => write!(f, "{}", repr),
            Value::LocalDate(ref repr) => write!(f, "{}", repr),
            Value::LocalTime(ref repr) => write!(f, "{}", repr),
            Value::Array(ref array) => write!(f, "{}", array),
            Value::InlineTable(ref table) => write!(f, "{}", table),
        }
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}[",
            self.decor.prefix().unwrap_or(DEFAULT_VALUE_DECOR.0)
        )?;
        join(f, self.iter(), ",")?;
        if self.trailing_comma {
            write!(f, ",")?;
        }
        write!(f, "{}", self.trailing)?;
        write!(
            f,
            "]{}",
            self.decor.suffix().unwrap_or(DEFAULT_VALUE_DECOR.1)
        )
    }
}

impl Display for InlineTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}{{",
            self.decor.prefix().unwrap_or(DEFAULT_VALUE_DECOR.0)
        )?;
        write!(f, "{}", self.preamble)?;
        for (i, (key, value)) in self
            .items
            .iter()
            .filter(|&(_, kv)| kv.value.is_value())
            .map(|(_, kv)| {
                (
                    kv.key_decor.display(&kv.key_repr, DEFAULT_INLINE_KEY_DECOR),
                    kv.value.as_value().unwrap(),
                )
            })
            .enumerate()
        {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}={}", key, value)?;
        }
        write!(
            f,
            "}}{}",
            self.decor.suffix().unwrap_or(DEFAULT_VALUE_DECOR.1)
        )
    }
}

impl Table {
    fn visit_nested_tables<'t, F>(
        &'t self,
        path: &mut Vec<&'t str>,
        is_array_of_tables: bool,
        callback: &mut F,
    ) -> Result
    where
        F: FnMut(&Table, &Vec<&'t str>, bool) -> Result,
    {
        callback(self, path, is_array_of_tables)?;

        for kv in self.items.values() {
            match kv.value {
                Item::Table(ref t) => {
                    path.push(kv.key_repr.as_raw());
                    t.visit_nested_tables(path, false, callback)?;
                    path.pop();
                }
                Item::ArrayOfTables(ref a) => {
                    for t in a.iter() {
                        path.push(kv.key_repr.as_raw());
                        t.visit_nested_tables(path, true, callback)?;
                        path.pop();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn visit_table(
    f: &mut dyn Write,
    table: &Table,
    path: &[&str],
    is_array_of_tables: bool,
) -> Result {
    if path.is_empty() {
        // don't print header for the root node
    } else if is_array_of_tables {
        write!(
            f,
            "{}[[",
            table.decor.prefix().unwrap_or(DEFAULT_TABLE_DECOR.0)
        )?;
        write!(f, "{}", path.join("."))?;
        writeln!(
            f,
            "]]{}",
            table.decor.suffix().unwrap_or(DEFAULT_TABLE_DECOR.1)
        )?;
    } else if !(table.implicit && table.values_len() == 0) {
        write!(
            f,
            "{}[",
            table.decor.prefix().unwrap_or(DEFAULT_TABLE_DECOR.0)
        )?;
        write!(f, "{}", path.join("."))?;
        writeln!(
            f,
            "]{}",
            table.decor.suffix().unwrap_or(DEFAULT_TABLE_DECOR.1)
        )?;
    }
    // print table body
    for kv in table.items.values() {
        if let Item::Value(ref value) = kv.value {
            writeln!(
                f,
                "{}={}",
                kv.key_decor.display(&kv.key_repr, DEFAULT_KEY_DECOR),
                value
            )?;
        }
    }
    Ok(())
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut path = Vec::new();

        self.visit_nested_tables(&mut path, false, &mut |t, path, is_array| {
            visit_table(f, t, path, is_array)
        })?;
        Ok(())
    }
}

impl Document {
    /// Returns a string representation of the TOML document, attempting to keep
    /// the table headers in their original order.
    pub fn to_string_in_original_order(&self) -> String {
        let mut string = String::new();
        let mut path = Vec::new();
        let mut last_position = 0;
        let mut tables = Vec::new();
        self.as_table()
            .visit_nested_tables(&mut path, false, &mut |t, p, is_array| {
                if let Some(pos) = t.position {
                    last_position = pos;
                }
                let mut s = String::new();
                visit_table(&mut s, t, p, is_array)?;
                tables.push((last_position, s));
                Ok(())
            })
            .unwrap();

        tables.sort_by_key(|&(id, _)| id);
        for (_, table) in tables {
            string.push_str(&table);
        }
        string.push_str(&self.trailing);
        string
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_table())?;
        write!(f, "{}", self.trailing)
    }
}

fn join<D, I>(f: &mut Formatter<'_>, iter: I, sep: &str) -> Result
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
