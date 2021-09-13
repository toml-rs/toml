use std::fmt::{Display, Formatter, Result, Write};

use itertools::Itertools;

use crate::datetime::*;
use crate::document::Document;
use crate::inline_table::DEFAULT_INLINE_KEY_DECOR;
use crate::key::Key;
use crate::repr::{DecorDisplay, Formatted, Repr, ValueRepr};
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

impl<T> Display for Formatted<T>
where
    T: ValueRepr,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let repr = self.to_repr();
        write!(
            f,
            "{}",
            self.decor().display(repr.as_ref(), DEFAULT_VALUE_DECOR)
        )
    }
}

impl ValueRepr for String {
    fn to_repr(&self) -> Repr {
        to_string_repr(self, None, None)
    }
}

pub(crate) fn to_string_repr(
    value: &str,
    style: Option<StringStyle>,
    literal: Option<bool>,
) -> Repr {
    let (style, literal) = match (style, literal) {
        (Some(style), Some(literal)) => (style, literal),
        (_, Some(literal)) => (infer_style(value).0, literal),
        (Some(style), _) => (style, infer_style(value).1),
        (_, _) => infer_style(value),
    };

    let mut output = String::with_capacity(value.len() * 2);
    if literal {
        output.push_str(style.literal_start());
        output.push_str(value);
        output.push_str(style.literal_end());
    } else {
        output.push_str(style.standard_start());
        for ch in value.chars() {
            match ch {
                '\u{8}' => output.push_str("\\b"),
                '\u{9}' => output.push_str("\\t"),
                '\u{a}' => match style {
                    StringStyle::NewlineTripple => output.push('\n'),
                    StringStyle::OnelineSingle => output.push_str("\\n"),
                    _ => unreachable!(),
                },
                '\u{c}' => output.push_str("\\f"),
                '\u{d}' => output.push_str("\\r"),
                '\u{22}' => output.push_str("\\\""),
                '\u{5c}' => output.push_str("\\\\"),
                c if c <= '\u{1f}' || c == '\u{7f}' => {
                    write!(output, "\\u{:04X}", ch as u32).unwrap();
                }
                ch => output.push(ch),
            }
        }
        output.push_str(style.standard_end());
    }

    Repr::new_unchecked(output)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum StringStyle {
    NewlineTripple,
    OnelineTripple,
    OnelineSingle,
}

impl StringStyle {
    fn literal_start(self) -> &'static str {
        match self {
            Self::NewlineTripple => "'''\n",
            Self::OnelineTripple => "'''",
            Self::OnelineSingle => "'",
        }
    }
    fn literal_end(self) -> &'static str {
        match self {
            Self::NewlineTripple => "'''",
            Self::OnelineTripple => "'''",
            Self::OnelineSingle => "'",
        }
    }

    fn standard_start(self) -> &'static str {
        match self {
            Self::NewlineTripple => "\"\"\"\n",
            // note: OnelineTripple can happen if do_pretty wants to do
            // '''it's one line'''
            // but literal == false
            Self::OnelineTripple | Self::OnelineSingle => "\"",
        }
    }

    fn standard_end(self) -> &'static str {
        match self {
            Self::NewlineTripple => "\"\"\"",
            // note: OnelineTripple can happen if do_pretty wants to do
            // '''it's one line'''
            // but literal == false
            Self::OnelineTripple | Self::OnelineSingle => "\"",
        }
    }
}

fn infer_style(value: &str) -> (StringStyle, bool) {
    // For doing pretty prints we store in a new String
    // because there are too many cases where pretty cannot
    // work. We need to determine:
    // - if we are a "multi-line" pretty (if there are \n)
    // - if ['''] appears if multi or ['] if single
    // - if there are any invalid control characters
    //
    // Doing it any other way would require multiple passes
    // to determine if a pretty string works or not.
    let mut out = String::with_capacity(value.len() * 2);
    let mut ty = StringStyle::OnelineSingle;
    // found consecutive single quotes
    let mut max_found_singles = 0;
    let mut found_singles = 0;
    let mut prefer_literal = false;
    let mut can_be_pretty = true;

    for ch in value.chars() {
        if can_be_pretty {
            if ch == '\'' {
                found_singles += 1;
                if found_singles >= 3 {
                    can_be_pretty = false;
                }
            } else {
                if found_singles > max_found_singles {
                    max_found_singles = found_singles;
                }
                found_singles = 0
            }
            match ch {
                '\t' => {}
                '\\' => {
                    prefer_literal = true;
                }
                '\n' => ty = StringStyle::NewlineTripple,
                // Escape codes are needed if any ascii control
                // characters are present, including \b \f \r.
                c if c <= '\u{1f}' || c == '\u{7f}' => can_be_pretty = false,
                _ => {}
            }
            out.push(ch);
        } else {
            // the string cannot be represented as pretty,
            // still check if it should be multiline
            if ch == '\n' {
                ty = StringStyle::NewlineTripple;
            }
        }
    }
    if found_singles > 0 && value.ends_with('\'') {
        // We cannot escape the ending quote so we must use """
        can_be_pretty = false;
    }
    if !prefer_literal {
        can_be_pretty = false;
    }
    if !can_be_pretty {
        debug_assert!(ty != StringStyle::OnelineTripple);
        return (ty, false);
    }
    if found_singles > max_found_singles {
        max_found_singles = found_singles;
    }
    debug_assert!(max_found_singles < 3);
    if ty == StringStyle::OnelineSingle && max_found_singles >= 1 {
        // no newlines, but must use ''' because it has ' in it
        ty = StringStyle::OnelineTripple;
    }
    (ty, true)
}

impl ValueRepr for i64 {
    fn to_repr(&self) -> Repr {
        Repr::new_unchecked(self.to_string())
    }
}

impl ValueRepr for f64 {
    fn to_repr(&self) -> Repr {
        to_f64_repr(*self)
    }
}

fn to_f64_repr(f: f64) -> Repr {
    let repr = match (f.is_sign_negative(), f.is_nan(), f == 0.0) {
        (true, true, _) => "-nan".to_owned(),
        (false, true, _) => "nan".to_owned(),
        (true, false, true) => "-0.0".to_owned(),
        (false, false, true) => "0.0".to_owned(),
        (_, false, false) => {
            if f % 1.0 == 0.0 {
                format!("{}.0", f)
            } else {
                format!("{}", f)
            }
        }
    };
    Repr::new_unchecked(repr)
}

impl ValueRepr for bool {
    fn to_repr(&self) -> Repr {
        Repr::new_unchecked(self.to_string())
    }
}

impl ValueRepr for Datetime {
    fn to_repr(&self) -> Repr {
        Repr::new_unchecked(self.to_string())
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // HACK: For now, leaving off decor since we don't know the defaults to use in this context
        self.to_repr().as_ref().fmt(f)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Value::String(ref repr) => write!(f, "{}", repr),
            Value::Integer(ref repr) => write!(f, "{}", repr),
            Value::Float(ref repr) => write!(f, "{}", repr),
            Value::Boolean(ref repr) => write!(f, "{}", repr),
            Value::Datetime(ref repr) => write!(f, "{}", repr),
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
