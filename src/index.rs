use std::ops;

use crate::document::Document;
use crate::key::Key;
use crate::table::TableKeyValue;
use crate::{value, InlineTable, Item, Table, Value};

// copied from
// https://github.com/serde-rs/json/blob/master/src/value/index.rs

pub trait Index: private::Sealed {
    /// Return `Option::None` if the key is not already in the array or table.
    #[doc(hidden)]
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item>;

    /// Panic if array index out of bounds. If key is not already in the table,
    /// insert it with a value of `Item::None`. Panic if `v` has a type that cannot be
    /// indexed into, except if `v` is `Item::None` then it can be treated as an empty
    /// inline table.
    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Item) -> &'v mut Item;
}

impl Index for usize {
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        match *v {
            Item::ArrayOfTables(ref aot) => aot.values.get(*self),
            Item::Value(ref a) if a.is_array() => a.as_array().unwrap().values.get(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Item) -> &'v mut Item {
        match *v {
            Item::ArrayOfTables(ref mut vec) => {
                vec.values.get_mut(*self).expect("index out of bounds")
            }
            Item::Value(ref mut a) if a.is_array() => a
                .as_array_mut()
                .unwrap()
                .values
                .get_mut(*self)
                .expect("index out of bounds"),
            _ => panic!("cannot access index {}", self),
        }
    }
}

impl Index for str {
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        match *v {
            Item::Table(ref t) => t.get(self),
            Item::Value(ref v) if v.is_inline_table() => v
                .as_inline_table()
                .and_then(|t| t.items.get(self).map(|kv| &kv.value)),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut Item) -> &'v mut Item {
        let key = Key::with_key(self);
        if let Item::None = *v {
            let mut t = InlineTable::default();
            t.items.insert(
                key.get().to_owned(),
                TableKeyValue::new(key.clone(), Item::None),
            );
            *v = value(Value::InlineTable(t));
        }
        match *v {
            Item::Table(ref mut t) => t.entry(key.get()).or_insert(Item::None),
            Item::Value(ref mut v) if v.is_inline_table() => {
                &mut v
                    .as_inline_table_mut()
                    .unwrap()
                    .items
                    .entry(key.get().to_owned())
                    .or_insert(TableKeyValue::new(key, Item::None))
                    .value
            }
            _ => panic!("cannot access key {}", self),
        }
    }
}

impl Index for String {
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        self[..].index(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Item) -> &'v mut Item {
        self[..].index_or_insert(v)
    }
}

impl<'a, T: ?Sized> Index for &'a T
where
    T: Index,
{
    fn index<'v>(&self, v: &'v Item) -> Option<&'v Item> {
        (**self).index(v)
    }
    fn index_or_insert<'v>(&self, v: &'v mut Item) -> &'v mut Item {
        (**self).index_or_insert(v)
    }
}

impl<I> ops::Index<I> for Item
where
    I: Index,
{
    type Output = Item;

    fn index(&self, index: I) -> &Item {
        static NONE: Item = Item::None;
        index.index(self).unwrap_or(&NONE)
    }
}

impl<I> ops::IndexMut<I> for Item
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Item {
        index.index_or_insert(self)
    }
}

impl<'s> ops::Index<&'s str> for Table {
    type Output = Item;

    fn index(&self, key: &'s str) -> &Item {
        static NONE: Item = Item::None;
        self.get(key).unwrap_or(&NONE)
    }
}

impl<'s> ops::IndexMut<&'s str> for Table {
    fn index_mut(&mut self, key: &'s str) -> &mut Item {
        self.entry(key).or_insert(Item::None)
    }
}

impl<'s> ops::Index<&'s str> for Document {
    type Output = Item;

    fn index(&self, key: &'s str) -> &Item {
        self.root.index(key)
    }
}

impl<'s> ops::IndexMut<&'s str> for Document {
    fn index_mut(&mut self, key: &'s str) -> &mut Item {
        self.root.index_mut(key)
    }
}

// Prevent users from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T: ?Sized> Sealed for &'a T where T: Sealed {}
}
