// Regressoion test for https://github.com/alexcrichton/toml-rs/issues/342

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use toml::Value;

fn parse(bench: &mut Criterion, name: &str, entries: usize, f: impl Fn(usize) -> String) {
    let mut s = String::new();
    for i in 0..entries {
        s += &f(i);
        s += "entry = 42\n"
    }
    let s = black_box(s);
    bench.bench_function(name, |b| {
        b.iter(|| {
            black_box(s.parse::<Value>().unwrap());
        })
    });
}

fn map_10(bench: &mut Criterion) {
    parse(bench, "map_10", 10, |i| format!("[header_no_{}]\n", i))
}

fn map_100(bench: &mut Criterion) {
    parse(bench, "map_100", 100, |i| format!("[header_no_{}]\n", i))
}

fn array_10(bench: &mut Criterion) {
    parse(bench, "array_10", 10, |_i| "[[header]]\n".to_owned())
}

fn array_100(bench: &mut Criterion) {
    parse(bench, "array_100", 100, |_i| "[[header]]\n".to_owned())
}

criterion_group!(benches, map_10, map_100, array_10, array_100);
criterion_main!(benches);
