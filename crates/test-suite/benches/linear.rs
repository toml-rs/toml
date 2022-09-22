// Regressoion test for https://github.com/alexcrichton/toml-rs/issues/342

use bencher::{benchmark_group, benchmark_main, black_box, Bencher};
use toml::Value;

fn parse(bench: &mut Bencher, entries: usize, f: impl Fn(usize) -> String) {
    let mut s = String::new();
    for i in 0..entries {
        s += &f(i);
        s += "entry = 42\n"
    }
    let s = black_box(s);
    bench.iter(|| {
        black_box(s.parse::<Value>().unwrap());
    })
}

fn map_10(bench: &mut Bencher) {
    parse(bench, 10, |i| format!("[header_no_{}]\n", i))
}

fn map_100(bench: &mut Bencher) {
    parse(bench, 100, |i| format!("[header_no_{}]\n", i))
}

fn array_10(bench: &mut Bencher) {
    parse(bench, 10, |_i| "[[header]]\n".to_owned())
}

fn array_100(bench: &mut Bencher) {
    parse(bench, 100, |_i| "[[header]]\n".to_owned())
}

benchmark_group!(benches, map_10, map_100, array_10, array_100);
benchmark_main!(benches);
