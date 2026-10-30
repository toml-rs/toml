use alloc::collections::BTreeMap;

use toml_writer::TomlWrite as _;

use crate::alloc_prelude::*;

/// TOML Document serialization buffer
#[derive(Debug, Default)]
pub struct Buffer {
    output: String,
    // Original byte offset and table creation order. Implicit parents can share an offset.
    insertions: BTreeMap<(InsertionOffset, Pos), String>,
    next_table_pos: Pos,
}

impl Buffer {
    /// Initialize a new serialization buffer
    pub fn new() -> Self {
        Default::default()
    }

    /// Reset the buffer for serializing another document
    pub fn clear(&mut self) {
        self.output.clear();
        self.insertions.clear();
        self.next_table_pos = Pos::default();
    }

    pub(crate) fn root_table(&mut self) -> Table {
        self.new_table(Vec::new())
    }

    pub(crate) fn child_table(&mut self, parent: &mut Table, encoded_key: String) -> Table {
        self.begin_children(parent);
        let mut path = Vec::with_capacity(parent.encoded_key.len() + 1);
        path.extend(parent.encoded_key.iter().cloned());
        path.push(encoded_key);
        self.new_table(path)
    }

    pub(crate) fn element_table(&mut self, parent: &mut Table, encoded_key: String) -> Table {
        let mut table = self.child_table(parent, encoded_key);
        table.array = true;
        table
    }

    fn new_table(&mut self, encoded_key: Vec<String>) -> Table {
        let pos = self.next_table_pos();
        Table {
            encoded_key,
            pos,
            insert_at: None,
            started: false,
            array: false,
        }
    }

    fn next_table_pos(&mut self) -> Pos {
        let pos = self.next_table_pos;
        self.next_table_pos.0 += 1;
        pos
    }

    pub(crate) fn begin_children(&mut self, table: &mut Table) {
        if table.insert_at.is_none() {
            // An array element needs its own header even if it only contains child tables.
            if table.array && !table.started {
                self.ensure_body(table);
            }
            table.insert_at = Some(self.insertion_offset());
        }
    }

    pub(crate) fn ensure_body(&mut self, table: &mut Table) -> &mut String {
        let (dst, offset) = if let Some(offset) = table.insert_at {
            (
                self.insertions.entry((offset, table.pos)).or_default(),
                offset,
            )
        } else {
            let offset = self.insertion_offset();
            (&mut self.output, offset)
        };
        if !table.started {
            if offset != InsertionOffset::start() {
                dst.push('\n');
            }
            table
                .write_header(dst)
                .expect("writing to a String cannot fail");
            table.started = true;
        }
        dst
    }

    fn insertion_offset(&self) -> InsertionOffset {
        InsertionOffset(self.output.len())
    }

    pub(crate) fn push(&mut self, mut table: Table) {
        if required_table(&table) {
            self.ensure_body(&mut table);
        }
    }

    pub(crate) fn into_string(mut self) -> String {
        for ((offset, _), mut text) in self.insertions.into_iter().rev() {
            if offset == InsertionOffset::start() && !self.output.is_empty() {
                text.push('\n');
            }
            self.output.insert_str(offset.0, &text);
        }
        self.output
    }
}

impl core::fmt::Display for Buffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut start = InsertionOffset::start();
        let mut leading_section = false;
        for (&(offset, _), text) in &self.insertions {
            if leading_section {
                f.write_str("\n")?;
            }
            f.write_str(&self.output[start.0..offset.0])?;
            f.write_str(text)?;
            start = offset;
            leading_section = offset == InsertionOffset::start();
        }
        if leading_section && !self.output.is_empty() {
            f.write_str("\n")?;
        }
        f.write_str(&self.output[start.0..])
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct InsertionOffset(usize);

impl InsertionOffset {
    const fn start() -> Self {
        Self(0)
    }
}

fn required_table(table: &Table) -> bool {
    // Empty leaf tables are explicit. Empty parents are implied by their children.
    !table.started && table.insert_at.is_none() && !table.encoded_key.is_empty()
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
    encoded_key: Vec<String>,
    pos: Pos,
    // Frozen when the first child starts. Later body entries belong at this boundary.
    insert_at: Option<InsertionOffset>,
    started: bool,
    array: bool,
}

impl Table {
    fn write_header(&self, dst: &mut String) -> core::fmt::Result {
        if let Some((first, rest)) = self.encoded_key.split_first() {
            if self.array {
                dst.open_array_of_tables_header()?;
            } else {
                dst.open_table_header()?;
            }
            dst.push_str(first);
            for key in rest {
                dst.key_sep()?;
                dst.push_str(key);
            }
            if self.array {
                dst.close_array_of_tables_header()?;
            } else {
                dst.close_table_header()?;
            }
            dst.newline()?;
        }
        Ok(())
    }
}
