use crate::decor::{Formatted, Repr};
use crate::document::Document;
use crate::table::{Item, Table};
use crate::value::{Array, DateTime, InlineTable, Value};
use std::fmt::{Display, Error, Formatter, Result, Write};

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

impl Table {
    fn visit_nested_tables_in_original_order<'t, F>(
        &'t self,
        path: &mut Vec<&'t str>,
        is_array_of_tables: bool,
        callback: &mut F,
    ) -> Result
    where
        F: FnMut(&Table, &Vec<&'t str>, bool) -> Result,
    {
        let mut current_position = 0usize;
        let mut last_position = 0usize;
        let mut max_position = 0usize;
        let mut should_print = true;
        loop {
            self.visit_nested_tables(path, is_array_of_tables, &mut |t, path, is_array| {
                match &t.position {
                    Some(ref pos) => {
                        if *pos > max_position {
                            max_position = *pos;
                        }
                        if *pos == current_position {
                            // if this table is from from the position we're
                            // looking for, print it.
                            should_print = true;
                            current_position += 1;
                        } else {
                            // if this table isn't from the position we're
                            // looking for, skip it.
                            should_print = false;
                        }
                    }
                    // If this table doesn't have a position then it was
                    // probably made programmatically, so put it after wherever
                    // it happens to be after in the tree. We only want to print
                    // it once though, so rely on should_print from the previous
                    // table to tell you whether you need to print this one.
                    None => (),
                }
                if should_print {
                    callback(t, path, is_array)
                } else {
                    Ok(())
                }
            })?;
            if current_position > max_position || max_position == 0 {
                break;
            } else if current_position == last_position {
                current_position += 1;
            }
            last_position = current_position;
            // we set should_print to true the first time we went around the
            // loop, so initially None-positioned Tables will have already been
            // printed.
            should_print = false;
        }
        Ok(())
    }

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
                    path.push(&kv.key.raw_value);
                    t.visit_nested_tables(path, false, callback)?;
                    path.pop();
                }
                Item::ArrayOfTables(ref a) => {
                    for t in a.iter() {
                        path.push(&kv.key.raw_value);
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

fn visit_table(f: &mut Write, table: &Table, path: &[&str], is_array_of_tables: bool) -> Result {
    if path.is_empty() {
        // don't print header for the root node
    } else if is_array_of_tables {
        write!(f, "{}[[", table.decor.prefix)?;
        write!(f, "{}", path.join("."))?;
        writeln!(f, "]]{}", table.decor.suffix)?;
    } else if !(table.implicit && table.values_len() == 0) {
        write!(f, "{}[", table.decor.prefix)?;
        write!(f, "{}", path.join("."))?;
        writeln!(f, "]{}", table.decor.suffix)?;
    }
    // print table body
    for kv in table.items.values() {
        if let Item::Value(ref value) = kv.value {
            writeln!(f, "{}={}", kv.key, value)?;
        }
    }
    Ok(())
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
    ///
    /// Known issues:
    /// * If you have created your Document by parsing two .toml files and
    ///   merging the results together, this method may silently skip some
    ///   tables. Please use Document.to_string() instead, which doesn't have
    ///   this problem.
    /// * The best case performance of this function is similar to
    ///   Document.to_string(). This will be true for  If you have lots of
    ///   tables that have been deleted or are in strange orders.
    pub fn to_string_in_original_order(&self) -> std::result::Result<String, Error> {
        let mut string = String::default();
        let mut path = Vec::new();

        self.as_table().visit_nested_tables_in_original_order(
            &mut path,
            false,
            &mut |t, path, is_array| visit_table(&mut string, t, path, is_array),
        )?;

        write!(string, "{}", self.trailing)?;
        Ok(string)
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
