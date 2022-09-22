use super::Error;

#[doc(hidden)]
pub struct SerializeItemArray {
    values: Vec<crate::Item>,
}

impl SerializeItemArray {
    pub(crate) fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub(crate) fn with_capacity(len: usize) -> Self {
        Self {
            values: Vec::with_capacity(len),
        }
    }
}

impl serde::ser::SerializeSeq for SerializeItemArray {
    type Ok = crate::Item;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize,
    {
        let value = value.serialize(super::ItemSerializer {})?;
        self.values.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(crate::Item::Value(crate::Value::Array(
            crate::Array::with_vec(self.values),
        )))
    }
}

impl serde::ser::SerializeTuple for SerializeItemArray {
    type Ok = crate::Item;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleVariant for SerializeItemArray {
    type Ok = crate::Item;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for SerializeItemArray {
    type Ok = crate::Item;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde::ser::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}
