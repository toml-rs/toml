#![allow(elided_lifetimes_in_paths)]

const NUM_ENTRIES: &[usize] = &[10, 100];

mod toml_parse {
    use crate::gen;
    use crate::NUM_ENTRIES;

    #[divan::bench(args = NUM_ENTRIES)]
    fn tokens(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| {
                let mut tokenizer = ::toml_parse::lexer::Tokenizer::new(&sample);
                while let Ok(Some(token)) = tokenizer.next() {
                    std::hint::black_box(token);
                }
            });
    }
}

mod toml_edit {
    use crate::gen;
    use crate::NUM_ENTRIES;

    #[divan::bench(args = NUM_ENTRIES)]
    fn document(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml_edit::DocumentMut>().unwrap());
    }
}

mod toml {
    use crate::gen;
    use crate::NUM_ENTRIES;

    #[divan::bench(args = NUM_ENTRIES)]
    fn document(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml::Table>().unwrap());
    }
}

mod toml_v05 {
    use crate::gen;
    use crate::NUM_ENTRIES;

    #[divan::bench(args = NUM_ENTRIES)]
    fn document(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml_old::Value>().unwrap());
    }
}

fn gen(num_entries: usize) -> String {
    let mut s = String::new();
    for _ in 0..num_entries {
        s += "[[header]]\n";
        s += "entry = 42\n";
    }
    s
}

fn main() {
    divan::main();
}
