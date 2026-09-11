#![allow(elided_lifetimes_in_paths)]

#[cfg(feature = "alloc-profiler")]
#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

const NUM_ENTRIES: &[usize] = &[10, 100];

mod toml_parser {
    use crate::NUM_ENTRIES;
    use crate::generate;

    #[divan::bench(args = NUM_ENTRIES)]
    fn tokens(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| generate(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| {
                let source = ::toml_parser::Source::new(&sample);
                source.lex().last()
            });
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn events(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| generate(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| {
                let source = ::toml_parser::Source::new(&sample);
                let tokens = source.lex().into_vec();
                let mut errors = Vec::new();
                ::toml_parser::parser::parse_document(
                    &tokens,
                    &mut |event| {
                        std::hint::black_box(event);
                    },
                    &mut errors,
                );
            });
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn decoded(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| generate(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| {
                struct Void<'s> {
                    source: &'s ::toml_parser::Source<'s>,
                }

                impl ::toml_parser::parser::EventReceiver for Void<'_> {
                    fn simple_key(
                        &mut self,
                        span: ::toml_parser::Span,
                        encoding: Option<::toml_parser::decoder::Encoding>,
                        error: &mut dyn ::toml_parser::ErrorSink,
                    ) {
                        let event = ::toml_parser::parser::Event::new_unchecked(
                            ::toml_parser::parser::EventKind::SimpleKey,
                            encoding,
                            span,
                        );
                        #[cfg(feature = "unsafe")]
                        // SAFETY: `EventReceiver` should always receive valid
                        // spans
                        let raw = unsafe { self.source.get_unchecked(event) };
                        #[cfg(not(feature = "unsafe"))]
                        let raw = self.source.get(event).unwrap();
                        let mut decoded = std::borrow::Cow::Borrowed("");
                        raw.decode_key(&mut decoded, error);
                    }
                    fn scalar(
                        &mut self,
                        span: ::toml_parser::Span,
                        encoding: Option<::toml_parser::decoder::Encoding>,
                        error: &mut dyn ::toml_parser::ErrorSink,
                    ) {
                        let event = ::toml_parser::parser::Event::new_unchecked(
                            ::toml_parser::parser::EventKind::SimpleKey,
                            encoding,
                            span,
                        );
                        #[cfg(feature = "unsafe")]
                        // SAFETY: `EventReceiver` should always receive valid
                        // spans
                        let raw = unsafe { self.source.get_unchecked(event) };
                        #[cfg(not(feature = "unsafe"))]
                        let raw = self.source.get(event).unwrap();
                        let mut decoded = std::borrow::Cow::Borrowed("");
                        let kind = raw.decode_scalar(&mut decoded, error);
                        std::hint::black_box(kind);
                    }
                }

                let source = ::toml_parser::Source::new(&sample);
                let tokens = source.lex().into_vec();
                let mut errors = Vec::new();
                let mut events = Void { source: &source };
                let mut receiver =
                    toml_parser::parser::ValidateWhitespace::new(&mut events, source);
                ::toml_parser::parser::parse_document(&tokens, &mut receiver, &mut errors);
            });
    }
}

mod toml_edit {
    use crate::NUM_ENTRIES;
    use crate::generate;
    use crate::generate_arrays_of_tables;
    use crate::generate_value_arrays;

    #[divan::bench(args = NUM_ENTRIES)]
    fn document(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| generate(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml_edit::DocumentMut>().unwrap());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn dump(bencher: divan::Bencher, num_entries: usize) {
        let document = generate(num_entries)
            .parse::<toml_edit::DocumentMut>()
            .unwrap();
        bencher.bench(|| std::hint::black_box(&document).to_string());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn value_arrays_dump(bencher: divan::Bencher, num_entries: usize) {
        let document = generate_value_arrays(num_entries)
            .parse::<toml_edit::DocumentMut>()
            .unwrap();
        bencher.bench(|| std::hint::black_box(&document).to_string());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn arrays_of_tables_dump(bencher: divan::Bencher, num_entries: usize) {
        let document = generate_arrays_of_tables(num_entries)
            .parse::<toml_edit::DocumentMut>()
            .unwrap();
        bencher.bench(|| std::hint::black_box(&document).to_string());
    }
}

mod toml {
    use crate::NUM_ENTRIES;
    use crate::generate;

    #[divan::bench(args = NUM_ENTRIES)]
    fn detable_owned(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| generate(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| {
                let mut table = toml::de::DeTable::parse(&sample).unwrap();
                table.get_mut().make_owned();
                // SAFETY: `make`_owned` removes references to `sample` and lifetimes don't affect
                // layout
                let table = unsafe {
                    std::mem::transmute::<
                        serde_spanned::Spanned<toml::de::DeTable<'_>>,
                        serde_spanned::Spanned<toml::de::DeTable<'static>>,
                    >(table)
                };
                table
            });
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn document(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| generate(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml::Table>().unwrap());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn dump(bencher: divan::Bencher, num_entries: usize) {
        let document = generate(num_entries).parse::<toml::Table>().unwrap();
        bencher.bench(|| toml::to_string(std::hint::black_box(&document)).unwrap());
    }
}

mod toml_v05 {
    use crate::NUM_ENTRIES;
    use crate::generate;

    #[divan::bench(args = NUM_ENTRIES)]
    fn document(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| generate(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| sample.parse::<toml_old::Value>().unwrap());
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn dump(bencher: divan::Bencher, num_entries: usize) {
        let document = generate(num_entries).parse::<toml_old::Value>().unwrap();
        bencher.bench(|| toml_old::to_string(std::hint::black_box(&document)).unwrap());
    }
}

fn generate(num_entries: usize) -> String {
    let mut s = String::new();
    for _ in 0..num_entries {
        s += "[[header]]\n";
        s += "entry = 42\n";
    }
    s
}

fn generate_value_arrays(num_entries: usize) -> String {
    let mut s = String::new();
    for i in 0..num_entries {
        s += &format!("array_{i} = [1, 2, 3]\n");
    }
    s
}

fn generate_arrays_of_tables(num_entries: usize) -> String {
    let mut s = String::new();
    for i in 0..num_entries {
        s += &format!("[[header_{i}]]\n");
        s += "entry = 42\n";
    }
    s
}

fn main() {
    divan::main();
}
