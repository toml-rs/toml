#![allow(elided_lifetimes_in_paths)]

#[cfg(feature = "alloc-profiler")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

const NUM_ENTRIES: &[usize] = &[10, 100];

mod toml_edit {
    use std::fmt::Write as _;

    use crate::NUM_ENTRIES;

    #[divan::bench(args = NUM_ENTRIES)]
    fn dump(bencher: divan::Bencher, entries: usize) {
        let mut document = ::toml_edit::DocumentMut::new();
        for i in 0..entries {
            let key = if i % 2 == 0 {
                format!("key_{i}")
            } else {
                format!("key {i}")
            };
            let value = match i % 4 {
                0 => ::toml_edit::Value::from("a generated string"),
                1 => ::toml_edit::Value::from(i64::try_from(i).unwrap()),
                2 => ::toml_edit::Value::from(true),
                _ => ::toml_edit::Value::from(1.25),
            };
            document.insert(&key, ::toml_edit::Item::Value(value));
        }
        bencher.bench(|| std::hint::black_box(&document).to_string());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn dotted_dump(bencher: divan::Bencher, entries: usize) {
        let mut input = String::new();
        for i in 0..entries {
            writeln!(&mut input, "parent.child.key_{i} = {i}").unwrap();
        }
        let document = input.parse::<::toml_edit::DocumentMut>().unwrap();
        bencher.bench(|| std::hint::black_box(&document).to_string());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn inline_dump(bencher: divan::Bencher, entries: usize) {
        let mut input = String::from("value = { ");
        for i in 0..entries {
            if i != 0 {
                input.push_str(", ");
            }
            write!(&mut input, "parent.child.key_{i} = {i}").unwrap();
        }
        input.push_str(" }\n");
        let document = input.parse::<::toml_edit::DocumentMut>().unwrap();
        bencher.bench(|| std::hint::black_box(&document).to_string());
    }
}

fn main() {
    divan::main();
}
