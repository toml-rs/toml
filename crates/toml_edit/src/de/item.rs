use crate::de::Error;

pub(crate) struct ItemDeserializer {
    input: crate::Item,
    validate_struct_keys: bool,
}

impl ItemDeserializer {
    pub(crate) fn new(input: crate::Item) -> Self {
        Self {
            input,
            validate_struct_keys: false,
        }
    }

    pub(crate) fn with_struct_key_validation(mut self) -> Self {
        self.validate_struct_keys = true;
        self
    }
}

impl<'de> serde::Deserializer<'de> for ItemDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.input.deserialize_any(visitor)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.input.deserialize_option(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.input.span();
        visitor
            .visit_newtype_struct(self)
            .map_err(|mut e: Self::Error| {
                if e.span().is_none() {
                    e.set_span(span);
                }
                e
            })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if super::is_spanned(name, fields) {
            if let Some(span) = self.input.span() {
                return visitor.visit_map(super::SpannedDeserializer::new(self.input, span));
            }
        }

        if self.validate_struct_keys {
            let span = self.input.span();
            match &self.input {
                crate::Item::Table(values) => super::validate_struct_keys(&values.items, fields)
                    .map_err(|mut e: Self::Error| {
                        if e.span().is_none() {
                            e.set_span(span);
                        }
                        e
                    })?,
                crate::Item::Value(crate::Value::InlineTable(values)) => {
                    super::validate_struct_keys(&values.items, fields).map_err(
                        |mut e: Self::Error| {
                            if e.span().is_none() {
                                e.set_span(span);
                            }
                            e
                        },
                    )?
                }
                _ => {}
            }
        }

        self.input.deserialize_struct(name, fields, visitor)
    }

    // Called when the type to deserialize is an enum, as opposed to a field in the type.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.input.deserialize_enum(name, variants, visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for ItemDeserializer {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

impl<'de> serde::Deserializer<'de> for crate::Item {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.span();
        match self {
            crate::Item::None => visitor.visit_none().map_err(|mut e: Self::Error| {
                if e.span().is_none() {
                    e.set_span(span);
                }
                e
            }),
            crate::Item::Value(v) => v.deserialize_any(visitor).map_err(|mut e: Self::Error| {
                if e.span().is_none() {
                    e.set_span(span);
                }
                e
            }),
            crate::Item::Table(v) => visitor
                .visit_map(crate::de::TableMapAccess::new(v))
                .map_err(|mut e: Self::Error| {
                    if e.span().is_none() {
                        e.set_span(span);
                    }
                    e
                }),
            crate::Item::ArrayOfTables(v) => visitor
                .visit_seq(crate::de::ArraySeqAccess::with_array_of_tables(v))
                .map_err(|mut e: Self::Error| {
                    if e.span().is_none() {
                        e.set_span(span);
                    }
                    e
                }),
        }
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.span();
        visitor.visit_some(self).map_err(|mut e: Self::Error| {
            if e.span().is_none() {
                e.set_span(span);
            }
            e
        })
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.span();
        visitor
            .visit_newtype_struct(self)
            .map_err(|mut e: Self::Error| {
                if e.span().is_none() {
                    e.set_span(span);
                }
                e
            })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if super::is_spanned(name, fields) {
            if let Some(span) = self.span() {
                return visitor.visit_map(super::SpannedDeserializer::new(self, span));
            }
        }

        self.deserialize_any(visitor)
    }

    // Called when the type to deserialize is an enum, as opposed to a field in the type.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let span = self.span();
        match self {
            crate::Item::Value(v) => {
                v.deserialize_enum(name, variants, visitor)
                    .map_err(|mut e: Self::Error| {
                        if e.span().is_none() {
                            e.set_span(span);
                        }
                        e
                    })
            }
            crate::Item::Table(v) => {
                if v.is_empty() {
                    Err(crate::de::Error::custom(
                        "wanted exactly 1 element, found 0 elements",
                        v.span(),
                    ))
                } else if v.len() != 1 {
                    Err(crate::de::Error::custom(
                        "wanted exactly 1 element, more than 1 element",
                        v.span(),
                    ))
                } else {
                    visitor
                        .visit_enum(crate::de::TableMapAccess::new(v))
                        .map_err(|mut e: Self::Error| {
                            if e.span().is_none() {
                                e.set_span(span);
                            }
                            e
                        })
                }
            }
            e => Err(crate::de::Error::custom("wanted string or table", e.span())),
        }
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

impl<'de> serde::de::IntoDeserializer<'de, crate::de::Error> for crate::Item {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}
