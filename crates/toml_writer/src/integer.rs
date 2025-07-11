use core::fmt::{self, Display};

#[derive(Copy, Clone, Debug)]
pub struct TomlIntegerFormat {
    radix: Radix,
}

impl TomlIntegerFormat {
    pub fn new() -> Self {
        Self {
            radix: Radix::Decimal,
        }
    }

    pub fn as_decimal(mut self) -> Self {
        self.radix = Radix::Decimal;
        self
    }

    pub fn as_hex_upper(mut self) -> Self {
        self.radix = Radix::Hexadecimal {
            case: HexCase::Upper,
        };
        self
    }

    pub fn as_hex_lower(mut self) -> Self {
        self.radix = Radix::Hexadecimal {
            case: HexCase::Lower,
        };
        self
    }

    pub fn as_octal(mut self) -> Self {
        self.radix = Radix::Octal;
        self
    }

    pub fn as_binary(mut self) -> Self {
        self.radix = Radix::Binary;
        self
    }

    /// Returns `None` if the value is negative and the radix is not decimal.
    pub fn format<N: PartialOrd<i32>>(self, value: N) -> Option<TomlInteger<N>>
    where
        TomlInteger<N>: crate::WriteTomlValue,
    {
        match self.radix {
            Radix::Decimal => (),
            Radix::Hexadecimal { .. } | Radix::Octal | Radix::Binary => {
                if value < 0 {
                    return None;
                }
            }
        }

        Some(TomlInteger {
            value,
            format: self,
        })
    }
}

impl Default for TomlIntegerFormat {
    fn default() -> Self {
        Self::new()
    }
}

/// N must be an integer type.
#[derive(Copy, Clone, Debug)]
pub struct TomlInteger<N> {
    value: N,
    format: TomlIntegerFormat,
}

impl crate::WriteTomlValue for TomlInteger<u8> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i8> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<u16> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i16> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<u32> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i32> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<u64> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i64> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<u128> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i128> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<usize> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<isize> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.format, writer)
    }
}

#[derive(Copy, Clone, Debug)]
enum Radix {
    Decimal,
    Hexadecimal { case: HexCase },
    Octal,
    Binary,
}

#[derive(Copy, Clone, Debug)]
enum HexCase {
    Upper,
    Lower,
}

fn write_toml_value<
    N: Display + fmt::UpperHex + fmt::LowerHex + fmt::Octal + fmt::Binary,
    W: crate::TomlWrite + ?Sized,
>(
    value: N,
    format: &TomlIntegerFormat,
    writer: &mut W,
) -> fmt::Result {
    match format.radix {
        Radix::Decimal => write!(writer, "{value}")?,
        Radix::Hexadecimal { case } => match case {
            HexCase::Upper => write!(writer, "0x{value:X}")?,
            HexCase::Lower => write!(writer, "0x{value:x}")?,
        },
        Radix::Octal => write!(writer, "0o{value:o}")?,
        Radix::Binary => write!(writer, "0b{value:b}")?,
    }
    Ok(())
}
