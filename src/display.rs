use decor::{Formatted, Repr};
use document::Document;
use std::fmt::{Display, Formatter, Result};
use table::{Item, Table};
use value::{Array, DateTime, InlineTable, Value};

impl Display for Repr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}{}{}",
            self.decor.prefix, self.raw_value, self.decor.suffix
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
        join(f, self.iter(), ",")?;
        if self.trailing_comma {
            write!(f, ",")?;
        }
        write!(f, "{}", self.trailing)?;
        write!(f, "]{}", self.decor.suffix)
    }
}

impl Display for InlineTable {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{{", self.decor.prefix)?;
        write!(f, "{}", self.preamble)?;
        for (i, (key, value)) in self
            .items
            .iter()
            .filter(|&(_, kv)| kv.value.is_value())
            .map(|(_, kv)| (&kv.key, kv.value.as_value().unwrap()))
            .enumerate()
        {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}={}", key, value)?;
        }
        write!(f, "}}{}", self.decor.suffix)
    }
}

fn visit_nested_tables<'t, F>(
    table: &'t Table,
    path: &mut Vec<&'t str>,
    is_array_of_tables: bool,
    callback: &mut F,
) -> Result
where
    F: FnMut(&Table, &Vec<&'t str>, bool) -> Result,
{
    callback(table, path, is_array_of_tables)?;

    for kv in table.items.values() {
        match kv.value {
            Item::Table(ref t) => {
                path.push(&kv.key.raw_value);
                visit_nested_tables(t, path, false, callback)?;
                path.pop();
            }
            Item::ArrayOfTables(ref a) => {
                for t in a.iter() {
                    path.push(&kv.key.raw_value);
                    visit_nested_tables(t, path, true, callback)?;
                    path.pop();
                }
            }
            _ => {}
        }
    }
    Ok(())
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut path = Vec::new();

        visit_nested_tables(
            self,
            &mut path,
            false,
            &mut |t: &Table, path, is_array_of_tables: bool| {
                if path.len() == 0 {
                    // don't print header for the root node
                } else if is_array_of_tables {
                    write!(f, "{}[[", t.decor.prefix)?;
                    write!(f, "{}", path.join("."))?;
                    writeln!(f, "]]{}", t.decor.suffix)?;
                } else if !(t.implicit && t.values_len() == 0) {
                    write!(f, "{}[", t.decor.prefix)?;
                    write!(f, "{}", path.join("."))?;
                    writeln!(f, "]{}", t.decor.suffix)?;
                }
                // print table body
                for kv in t.items.values() {
                    if let Item::Value(ref value) = kv.value {
                        writeln!(f, "{}={}", kv.key, value)?;
                    }
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_table())?;
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
