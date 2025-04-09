#![allow(elided_lifetimes_in_paths)]

mod toml_parse {
    use toml_benchmarks::{Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn tokens(sample: &Data<'static>) -> Option<::toml_parse::lexer::Token> {
        let source = ::toml_parse::Source::new(sample.content());
        source.lex().last()
    }

    #[divan::bench(args=MANIFESTS)]
    fn events(sample: &Data<'static>) {
        let source = ::toml_parse::Source::new(sample.content());
        let tokens = source.lex().into_vec();
        let mut errors = Vec::new();
        ::toml_parse::parser::parse_document(
            &tokens,
            &mut |event| {
                std::hint::black_box(event);
            },
            &mut errors,
        );
    }

    #[divan::bench(args=MANIFESTS)]
    fn decoded(sample: &Data<'static>) {
        struct Void<'s> {
            source: &'s ::toml_parse::Source<'s>,
        }

        impl ::toml_parse::parser::EventReceiver for Void<'_> {
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
                #[cfg(feature = "unsafe")] // SAFETY: `EventReceiver` should always receive valid
                // spans
                let raw = unsafe { self.source.get_unchecked(event) };
                #[cfg(not(feature = "unsafe"))]
                let raw = self.source.get(event).unwrap();
                let mut decoded = std::borrow::Cow::Borrowed("");
                raw.decode_key(&mut decoded, error);
            }
        }

        let source = ::toml_parse::Source::new(sample.content());
        let tokens = source.lex().into_vec();
        let mut errors = Vec::new();
        let mut events = Void { source: &source };
        ::toml_parse::parser::parse_document(&tokens, &mut events, &mut errors);
    }
}

mod toml_edit {
    use toml_benchmarks::{manifest, Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn document(sample: &Data<'static>) -> ::toml_edit::DocumentMut {
        sample.content().parse().unwrap()
    }

    #[divan::bench(args=MANIFESTS)]
    fn manifest(sample: &Data<'static>) -> manifest::Manifest {
        ::toml_edit::de::from_str(sample.content()).unwrap()
    }
}

mod toml {
    use toml_benchmarks::{manifest, Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn document(sample: &Data<'static>) -> ::toml::Value {
        sample.content().parse().unwrap()
    }

    #[divan::bench(args=MANIFESTS)]
    fn manifest(sample: &Data<'static>) -> manifest::Manifest {
        ::toml::de::from_str(sample.content()).unwrap()
    }
}

mod toml_v05 {
    use toml_benchmarks::{manifest, Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn document(sample: &Data<'static>) -> ::toml_old::Value {
        sample.content().parse().unwrap()
    }

    #[divan::bench(args=MANIFESTS)]
    fn manifest(sample: &Data<'static>) -> manifest::Manifest {
        ::toml_old::de::from_str(sample.content()).unwrap()
    }
}

mod serde_json {
    use toml_benchmarks::{manifest, Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn document(bencher: divan::Bencher, sample: &Data) {
        let value = toml_edit::de::from_str::<serde_json::Value>(sample.content()).unwrap();
        let json = serde_json::to_string_pretty(&value).unwrap();
        bencher
            .with_inputs(|| {
                let sample = Data(sample.name(), &json);
                sample
            })
            .bench_values(|sample| {
                serde_json::from_str::<::toml::Value>(sample.content()).unwrap()
            });
    }

    #[divan::bench(args=MANIFESTS)]
    fn manifest(bencher: divan::Bencher, sample: &Data) {
        let value = toml_edit::de::from_str::<serde_json::Value>(sample.content()).unwrap();
        let json = serde_json::to_string_pretty(&value).unwrap();
        bencher
            .with_inputs(|| {
                let sample = Data(sample.name(), &json);
                sample
            })
            .bench_values(|sample| {
                serde_json::from_str::<manifest::Manifest>(sample.content()).unwrap()
            });
    }
}

fn main() {
    divan::main();
}
