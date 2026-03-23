#![allow(elided_lifetimes_in_paths)]

mod toml_parser {
    use toml_benchmarks::{Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn tokens(sample: &Data<'static>) -> Option<::toml_parser::lexer::Token> {
        let source = ::toml_parser::Source::new(sample.content());
        source.lex().last()
    }

    #[divan::bench(args=MANIFESTS)]
    fn events(sample: &Data<'static>) {
        let source = ::toml_parser::Source::new(sample.content());
        let tokens = source.lex().into_vec();
        let mut errors = Vec::new();
        ::toml_parser::parser::parse_document(
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
                #[cfg(feature = "unsafe")] // SAFETY: `EventReceiver` should always receive valid
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
                #[cfg(feature = "unsafe")] // SAFETY: `EventReceiver` should always receive valid
                // spans
                let raw = unsafe { self.source.get_unchecked(event) };
                #[cfg(not(feature = "unsafe"))]
                let raw = self.source.get(event).unwrap();
                let mut decoded = std::borrow::Cow::Borrowed("");
                let kind = raw.decode_scalar(&mut decoded, error);
                std::hint::black_box(kind);
            }
        }

        let source = ::toml_parser::Source::new(sample.content());
        let tokens = source.lex().into_vec();
        let mut errors = Vec::new();
        let mut events = Void { source: &source };
        let mut receiver = toml_parser::parser::ValidateWhitespace::new(&mut events, source);
        ::toml_parser::parser::parse_document(&tokens, &mut receiver, &mut errors);
    }
}

mod toml_edit {
    use toml_benchmarks::{Data, MANIFESTS, manifest};

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
    use toml_benchmarks::{Data, MANIFESTS, manifest};

    #[divan::bench(args=MANIFESTS)]
    fn detable(sample: &Data<'static>) -> serde_spanned::Spanned<::toml::de::DeTable<'static>> {
        let table = toml::de::DeTable::parse(sample.content()).unwrap();
        table
    }

    #[divan::bench(args=MANIFESTS)]
    fn detable_owned(
        sample: &Data<'static>,
    ) -> serde_spanned::Spanned<::toml::de::DeTable<'static>> {
        let mut table = toml::de::DeTable::parse(sample.content()).unwrap();
        table.get_mut().make_owned();
        table
    }

    #[divan::bench(args=MANIFESTS)]
    fn document(sample: &Data<'static>) -> ::toml::Table {
        sample.content().parse().unwrap()
    }

    #[divan::bench(args=MANIFESTS)]
    fn manifest(sample: &Data<'static>) -> manifest::Manifest {
        ::toml::de::from_str(sample.content()).unwrap()
    }
}

mod toml_v05 {
    use toml_benchmarks::{Data, MANIFESTS, manifest};

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
    use toml_benchmarks::{Data, MANIFESTS, manifest};

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
