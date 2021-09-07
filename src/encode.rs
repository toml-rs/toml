use std::fmt::{Display, Formatter, Result, Write};

use itertools::Itertools;

use crate::document::Document;
use crate::inline_table::DEFAULT_INLINE_KEY_DECOR;
use crate::key::Key;
use crate::repr::{DecorDisplay, Formatted, Repr};
use crate::table::{DEFAULT_KEY_DECOR, DEFAULT_KEY_PATH_DECOR, DEFAULT_TABLE_DECOR};
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
        write!(
            f,
            "{}",
            self.decor().display(self.repr(), DEFAULT_VALUE_DECOR)
        )
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // HACK: For now, leaving off decor since we don't know the defaults to use in this context
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
            self.decor().prefix().unwrap_or(DEFAULT_VALUE_DECOR.0)
        )?;
        write!(f, "{}", self.iter().join(","))?;
        if self.trailing_comma() && !self.is_empty() {
            write!(f, ",")?;
        }
        write!(f, "{}", self.trailing())?;
        write!(
            f,
            "]{}",
            self.decor().suffix().unwrap_or(DEFAULT_VALUE_DECOR.1)
        )
    }
}

impl Display for InlineTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}{{",
            self.decor().prefix().unwrap_or(DEFAULT_VALUE_DECOR.0)
        )?;
        write!(f, "{}", self.preamble)?;

        let children = self.get_values();
        for (i, (key_path, value)) in children.into_iter().enumerate() {
            let key = key_path_display(&key_path, DEFAULT_INLINE_KEY_DECOR);
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}={}", key, value)?;
        }

        write!(
            f,
            "}}{}",
            self.decor().suffix().unwrap_or(DEFAULT_VALUE_DECOR.1)
        )
    }
}

impl Table {
    fn visit_nested_tables<'t, F>(
        &'t self,
        path: &mut Vec<&'t Key>,
        is_array_of_tables: bool,
        callback: &mut F,
    ) -> Result
    where
        F: FnMut(&'t Table, &Vec<&'t Key>, bool) -> Result,
    {
        callback(self, path, is_array_of_tables)?;

        for kv in self.items.values() {
            match kv.value {
                Item::Table(ref t) if !t.is_dotted() => {
                    path.push(&kv.key);
                    t.visit_nested_tables(path, false, callback)?;
                    path.pop();
                }
                Item::ArrayOfTables(ref a) => {
                    for t in a.iter() {
                        path.push(&kv.key);
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
    path: &[&Key],
    is_array_of_tables: bool,
) -> Result {
    let children = table.get_values();

    if path.is_empty() {
        // don't print header for the root node
    } else if is_array_of_tables {
        write!(
            f,
            "{}[[",
            table.decor.prefix().unwrap_or(DEFAULT_TABLE_DECOR.0)
        )?;
        write!(
            f,
            "{}",
            path.iter()
                .map(|k| k.decor.display(k, DEFAULT_KEY_PATH_DECOR))
                .join(".")
        )?;
        writeln!(
            f,
            "]]{}",
            table.decor.suffix().unwrap_or(DEFAULT_TABLE_DECOR.1)
        )?;
    } else if !(table.implicit && children.is_empty()) {
        write!(
            f,
            "{}[",
            table.decor.prefix().unwrap_or(DEFAULT_TABLE_DECOR.0)
        )?;
        write!(
            f,
            "{}",
            path.iter()
                .map(|k| k.decor.display(k, DEFAULT_KEY_PATH_DECOR))
                .join(".")
        )?;
        writeln!(
            f,
            "]{}",
            table.decor.suffix().unwrap_or(DEFAULT_TABLE_DECOR.1)
        )?;
    }
    // print table body
    for (key_path, value) in children {
        let key = key_path_display(&key_path, DEFAULT_KEY_DECOR);
        writeln!(f, "{}={}", key, value)?;
    }
    Ok(())
}

fn key_path_display(key_path: &[&Key], default: (&'static str, &'static str)) -> impl Display {
    key_path
        .iter()
        .enumerate()
        .map(|(ki, k)| {
            let prefix = if ki == 0 {
                default.0
            } else {
                DEFAULT_KEY_PATH_DECOR.0
            };
            let suffix = if ki + 1 == key_path.len() {
                default.1
            } else {
                DEFAULT_KEY_PATH_DECOR.1
            };
            k.decor.display(k, (prefix, suffix))
        })
        .join(".")
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

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut path = Vec::new();
        let mut last_position = 0;
        let mut tables = Vec::new();
        self.as_table()
            .visit_nested_tables(&mut path, false, &mut |t, p, is_array| {
                if let Some(pos) = t.position {
                    last_position = pos;
                }
                tables.push((last_position, t, p.clone(), is_array));
                Ok(())
            })
            .unwrap();

        tables.sort_by_key(|&(id, _, _, _)| id);
        for (_, table, path, is_array) in tables {
            visit_table(f, table, &path, is_array)?;
        }
        self.trailing.fmt(f)
    }
}
