use crate::key::Key;
use crate::parser::strings::{basic_string, literal_string};
use crate::parser::trivia::ws;
use crate::repr::{Decor, Repr};
use crate::InternalString;
use combine::parser::char::char;
use combine::parser::range::{recognize_with_value, take_while1};
use combine::stream::RangeStream;
use combine::*;

/// Bypass allocations for single-element paths
#[derive(Default)]
struct KeyPath {
    parent: Vec<Key>,
    key: Option<Key>,
}

impl Extend<Key> for KeyPath {
    fn extend<T: IntoIterator<Item = Key>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();
        if let Some(last) = self.key.take() {
            self.parent.push(last);
            self.parent.extend(iter);
            let key = self.parent.pop().unwrap();
            self.key = Some(key);
        } else {
            let first = iter.next().expect("always at least one");
            if let Some(second) = iter.next() {
                self.parent.push(first);
                self.parent.push(second);
                self.parent.extend(iter);
                let key = self.parent.pop().unwrap();
                self.key = Some(key);
            } else {
                // Might not look like it but this is the common case and why we are doing all of
                // this.
                self.key = Some(first);
            }
        }
    }
}

// key = simple-key / dotted-key
// dotted-key = simple-key 1*( dot-sep simple-key )
parse!(key() -> Vec<Key>, {
    sep_by1(
        simple_key_ws(),
        char(DOT_SEP)
    )
});
parse!(key_path() -> (Vec<Key>, Key), {
    sep_by1::<KeyPath, _, _, _>(
        simple_key_ws(),
        char(DOT_SEP)
    ).map(|kp| (kp.parent, kp.key.unwrap()))
});
parse!(simple_key_ws() -> Key, {
    attempt((
        ws(),
        simple_key(),
        ws(),
    )).map(|(pre, (raw, key), suffix)| {
        Key::new(key).with_repr_unchecked(Repr::new_unchecked(raw)).with_decor(Decor::new(pre, suffix))
    })
});

// simple-key = quoted-key / unquoted-key
parse!(simple_key() -> (&'a str, InternalString), {
    recognize_with_value(choice((
        quoted_key(),
        unquoted_key().map(|s: &'a str| s.into()),
    )))
});

// unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _
parse!(unquoted_key() -> &'a str, {
    take_while1(is_unquoted_char)
});

#[inline]
pub(crate) fn is_unquoted_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_')
}

// quoted-key = basic-string / literal-string
parse!(quoted_key() -> InternalString, {
    choice((
        basic_string().map(|s: String| s.into()),
        literal_string().map(|s: &'a str| s.into()),
    ))
});

// dot-sep   = ws %x2E ws  ; . Period
const DOT_SEP: char = '.';
