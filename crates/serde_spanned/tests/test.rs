use core::cmp::{Ord, Ordering, PartialOrd};

use serde_spanned::Spanned;

#[test]
fn operators() {
    struct Foo {
        bar: Spanned<bool>,
        baz: Spanned<String>,
    }
    let f = Foo {
        bar: Spanned::new(0..4, true),
        baz: Spanned::new(5..7, "yes".to_owned()),
    };
    let g = Foo {
        bar: Spanned::new(5..7, true),
        baz: Spanned::new(0..4, "yes".to_owned()),
    };
    assert!(f.bar.span() != g.bar.span());
    assert!(f.baz.span() != g.baz.span());

    // test that eq still holds
    assert_eq!(f.bar, g.bar);
    assert_eq!(f.baz, g.baz);

    // test that Ord returns equal order
    assert_eq!(f.bar.cmp(&g.bar), Ordering::Equal);
    assert_eq!(f.baz.cmp(&g.baz), Ordering::Equal);

    // test that PartialOrd returns equal order
    assert_eq!(f.bar.partial_cmp(&g.bar), Some(Ordering::Equal));
    assert_eq!(f.baz.partial_cmp(&g.baz), Some(Ordering::Equal));
}
