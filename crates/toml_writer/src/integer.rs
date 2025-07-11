use core::fmt::{self, Display};

/// N must be an integer type.
#[derive(Copy, Clone, Debug)]
pub struct TomlInteger<N> {
    value: N,
    radix: Radix,
}

impl crate::WriteTomlValue for TomlInteger<u8> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i8> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<u16> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i16> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<u32> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i32> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<u64> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i64> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<u128> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<i128> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<usize> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

impl crate::WriteTomlValue for TomlInteger<isize> {
    fn write_toml_value<W: crate::TomlWrite + ?Sized>(&self, writer: &mut W) -> fmt::Result {
        write_toml_value(self.value, &self.radix, writer)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TomlIntegerFormat {
    radix: Radix,
    _separators: (), // Placeholder for future use
}

impl TomlIntegerFormat {
    pub fn new() -> Self {
        Self {
            radix: Radix::Decimal,
            _separators: (),
        }
    }

    pub fn as_decimal(self) -> Self {
        Self {
            radix: Radix::Decimal,
            ..self
        }
    }

    pub fn as_hex_upper(self) -> Self {
        Self {
            radix: Radix::Hexadecimal {
                case: HexCase::Upper,
            },
            ..self
        }
    }

    pub fn as_hex_lower(self) -> Self {
        Self {
            radix: Radix::Hexadecimal {
                case: HexCase::Lower,
            },
            ..self
        }
    }

    pub fn as_octal(self) -> Self {
        Self {
            radix: Radix::Octal,
            ..self
        }
    }

    pub fn as_binary(self) -> Self {
        Self {
            radix: Radix::Binary,
            ..self
        }
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
            radix: self.radix,
        })
    }
}

impl Default for TomlIntegerFormat {
    fn default() -> Self {
        Self::new()
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
    radix: &Radix,
    writer: &mut W,
) -> fmt::Result {
    match radix {
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
