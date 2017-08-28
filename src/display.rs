use std::fmt::{Display, Formatter, Result};
use value::{Array, DateTime, InlineTable, KeyValue, Value};
use decor::{Formatted, Repr};
use document::Document;
use table::{Container, Table};
use std::cell::{Cell, RefCell};

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
        if self.trailing_comma {
            write!(f, ",")?;
        }
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

struct TableFormatState<'t> {
    path: RefCell<Vec<&'t str>>,
    table: Cell<&'t Table>,
}

impl<'a> Display for TableFormatState<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let table = self.table.get();

        join(f, table.key_value_pairs.values(), "\n")?;
        if !table.key_value_pairs.is_empty() {
            write!(f, "\n")?;
        }

        for &(ref name, ref c) in table.containers.values() {
            match *c {
                Container::Table(ref t) => {
                    self.path.borrow_mut().push(name);
                    self.table.set(t);
                    if !t.implicit {
                        write!(f, "{}[", t.decor.prefix)?;
                        write!(f, "{}", self.path.borrow().join("."))?;
                        write!(f, "]{}\n", t.decor.suffix)?;
                    }
                    write!(f, "{}", self)?;
                    self.table.set(table);
                    self.path.borrow_mut().pop();
                }
                Container::Array(ref a) => {
                    self.path.borrow_mut().push(name);
                    for t in &a.values {
                        self.table.set(t);
                        write!(f, "{}[[", t.decor.prefix)?;
                        write!(f, "{}", self.path.borrow().join("."))?;
                        write!(f, "]]{}\n", t.decor.suffix)?;
                        write!(f, "{}", self)?;
                    }
                    self.table.set(table);
                    self.path.borrow_mut().pop();
                }
            }
        }
        Ok(())
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let state = TableFormatState {
            path: RefCell::new(Vec::new()),
            table: Cell::new(&self.root),
        };
        write!(f, "{}", state)?;
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
