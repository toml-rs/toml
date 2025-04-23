pub trait TomlWrite: core::fmt::Write {
    fn open_table_header(&mut self) -> core::fmt::Result {
        write!(self, "[")
    }
    fn close_table_header(&mut self) -> core::fmt::Result {
        write!(self, "]")
    }

    fn open_array_of_tables_header(&mut self) -> core::fmt::Result {
        write!(self, "[[")
    }
    fn close_array_of_tables_header(&mut self) -> core::fmt::Result {
        write!(self, "]]")
    }

    fn open_inline_table(&mut self) -> core::fmt::Result {
        write!(self, "{{")
    }
    fn close_inline_table(&mut self) -> core::fmt::Result {
        write!(self, "}}")
    }

    fn open_array(&mut self) -> core::fmt::Result {
        write!(self, "[")
    }
    fn close_array(&mut self) -> core::fmt::Result {
        write!(self, "]")
    }

    fn key_sep(&mut self) -> core::fmt::Result {
        write!(self, ".")
    }

    fn keyval_sep(&mut self) -> core::fmt::Result {
        write!(self, "=")
    }

    fn key(&mut self, value: impl crate::WriteTomlKey) -> core::fmt::Result {
        value.write_toml_key(self)
    }

    fn value(&mut self, value: impl crate::WriteTomlValue) -> core::fmt::Result {
        value.write_toml_value(self)
    }

    fn val_sep(&mut self) -> core::fmt::Result {
        write!(self, ",")
    }

    fn space(&mut self) -> core::fmt::Result {
        write!(self, " ")
    }

    fn open_comment(&mut self) -> core::fmt::Result {
        write!(self, "#")
    }

    fn newline(&mut self) -> core::fmt::Result {
        writeln!(self)
    }
}

impl<W> TomlWrite for W where W: core::fmt::Write {}
