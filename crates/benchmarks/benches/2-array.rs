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
                let source = ::toml_parse::Source::new(&sample);
                source.lex().last()
            });
    }

    #[divan::bench(args = NUM_ENTRIES)]
    fn events(bencher: divan::Bencher, num_entries: usize) {
        bencher
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| {
                let source = ::toml_parse::Source::new(&sample);
                let tokens = source.lex().into_vec();
                let mut errors = Vec::new();
                ::toml_parse::parser::parse_document(
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
            .with_inputs(|| gen(num_entries))
            .input_counter(divan::counter::BytesCount::of_str)
            .bench_values(|sample| {
                struct Void<'s> {
                    source: &'s ::toml_parse::Source<'s>,
                }

                impl ::toml_parse::parser::EventReceiver for Void<'_> {
                    fn comment(
                        &mut self,
                        span: ::toml_parse::Span,
                        error: &mut dyn ::toml_parse::ErrorSink,
                    ) {
                        let event = ::toml_parse::parser::Event::new_unchecked(
                            ::toml_parse::parser::EventKind::Comment,
                            None,
                            span,
                        );
                        #[cfg(feature = "unsafe")]
                        // SAFETY: `EventReceiver` should always receive valid
                        // spans
                        let raw = unsafe { self.source.get_unchecked(event) };
                        #[cfg(not(feature = "unsafe"))]
                        let raw = self.source.get(event).unwrap();
                        raw.decode_comment(error);
                    }
                    fn simple_key(
                        &mut self,
                        span: ::toml_parse::Span,
                        encoding: Option<::toml_parse::decode::Encoding>,
                        error: &mut dyn ::toml_parse::ErrorSink,
                    ) {
                        let event = ::toml_parse::parser::Event::new_unchecked(
                            ::toml_parse::parser::EventKind::SimpleKey,
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
                        span: ::toml_parse::Span,
                        encoding: Option<::toml_parse::decode::Encoding>,
                        error: &mut dyn ::toml_parse::ErrorSink,
                    ) {
                        let event = ::toml_parse::parser::Event::new_unchecked(
                            ::toml_parse::parser::EventKind::SimpleKey,
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

                let source = ::toml_parse::Source::new(&sample);
                let tokens = source.lex().into_vec();
                let mut errors = Vec::new();
                let mut events = Void { source: &source };
                ::toml_parse::parser::parse_document(&tokens, &mut events, &mut errors);
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
