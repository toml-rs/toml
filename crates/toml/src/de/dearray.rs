use serde_spanned::Spanned;

use crate::de::DeValue;

/// Type representing a TOML array, payload of the `DeValue::Array` variant
#[derive(Clone)]
pub struct DeArray<'i> {
    items: Vec<Spanned<DeValue<'i>>>,
}

impl<'i> std::ops::Deref for DeArray<'i> {
    type Target = [Spanned<DeValue<'i>>];

    #[inline]
    fn deref(&self) -> &[Spanned<DeValue<'i>>] {
        self.items.as_slice()
    }
}

impl<'i> std::ops::DerefMut for DeArray<'i> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Spanned<DeValue<'i>>] {
        self.items.as_mut_slice()
    }
}

impl<'i> AsRef<[Spanned<DeValue<'i>>]> for DeArray<'i> {
    fn as_ref(&self) -> &[Spanned<DeValue<'i>>] {
        &self.items
    }
}

impl<'i> AsMut<[Spanned<DeValue<'i>>]> for DeArray<'i> {
    fn as_mut(&mut self) -> &mut [Spanned<DeValue<'i>>] {
        &mut self.items
    }
}

impl<'i> std::borrow::Borrow<[Spanned<DeValue<'i>>]> for DeArray<'i> {
    fn borrow(&self) -> &[Spanned<DeValue<'i>>] {
        &self.items[..]
    }
}

impl<'i> std::borrow::BorrowMut<[Spanned<DeValue<'i>>]> for DeArray<'i> {
    fn borrow_mut(&mut self) -> &mut [Spanned<DeValue<'i>>] {
        &mut self.items[..]
    }
}

impl<'i, I: std::slice::SliceIndex<[Spanned<DeValue<'i>>]>> std::ops::Index<I> for DeArray<'i> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.items.index(index)
    }
}

impl<'a, 'i> IntoIterator for &'a DeArray<'i> {
    type Item = &'a Spanned<DeValue<'i>>;

    type IntoIter = std::slice::Iter<'a, Spanned<DeValue<'i>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i> IntoIterator for DeArray<'i> {
    type Item = Spanned<DeValue<'i>>;

    type IntoIter = std::vec::IntoIter<Spanned<DeValue<'i>>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'i> FromIterator<Spanned<DeValue<'i>>> for DeArray<'i> {
    #[inline]
    #[track_caller]
    fn from_iter<I: IntoIterator<Item = Spanned<DeValue<'i>>>>(iter: I) -> DeArray<'i> {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

impl Default for DeArray<'static> {
    #[inline]
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}

impl PartialEq for DeArray<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.items.eq(&other.items)
    }
}

impl Eq for DeArray<'_> {}

impl std::fmt::Debug for DeArray<'_> {
    #[inline]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.items.fmt(formatter)
    }
}
