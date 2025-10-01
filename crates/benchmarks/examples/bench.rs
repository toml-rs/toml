fn main() -> Result<(), lexopt::Error> {
    let args = Args::parse()?;

    match args.parser {
        Parser::Tokens => {
            let source = ::toml_parser::Source::new(args.data.content());
            let _tokens = source.lex().into_vec();
            let _tokens = std::hint::black_box(_tokens);
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_tokens:?}");
        }
        Parser::Events => {
            let source = ::toml_parser::Source::new(args.data.content());
            let tokens = source.lex().into_vec();
            let mut events = Vec::with_capacity(tokens.len());
            let mut _errors = Vec::with_capacity(tokens.len());
            ::toml_parser::parser::parse_document(&tokens, &mut events, &mut _errors);
            let _events = std::hint::black_box(events);
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_events:?}");
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_errors:?}");
        }
        Parser::Decoded => {
            let source = ::toml_parser::Source::new(args.data.content());
            let tokens = source.lex().into_vec();
            let mut events = Vec::<toml_parser::parser::Event>::with_capacity(tokens.len());
            let mut receiver = toml_parser::parser::ValidateWhitespace::new(&mut events, source);
            let mut _errors = Vec::with_capacity(tokens.len());
            ::toml_parser::parser::parse_document(&tokens, &mut receiver, &mut _errors);
            for event in &events {
                if event.kind() == ::toml_parser::parser::EventKind::SimpleKey {
                    #[cfg(feature = "unsafe")]
                    // SAFETY: `EventReceiver` should always receive valid
                    // spans
                    let raw = unsafe { source.get_unchecked(event) };
                    #[cfg(not(feature = "unsafe"))]
                    let raw = source.get(event).unwrap();
                    let mut decoded = std::borrow::Cow::Borrowed("");
                    raw.decode_key(&mut decoded, &mut _errors);
                    std::hint::black_box(decoded);
                } else if event.kind() == ::toml_parser::parser::EventKind::Scalar {
                    #[cfg(feature = "unsafe")]
                    // SAFETY: `EventReceiver` should always receive valid
                    // spans
                    let raw = unsafe { source.get_unchecked(event) };
                    #[cfg(not(feature = "unsafe"))]
                    let raw = source.get(event).unwrap();
                    let mut decoded = std::borrow::Cow::Borrowed("");
                    let kind = raw.decode_scalar(&mut decoded, &mut _errors);
                    std::hint::black_box(decoded);
                    std::hint::black_box(kind);
                }
            }

            let _events = std::hint::black_box(events);
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_events:?}");
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_errors:?}");
        }
        Parser::Document => {
            let _doc = args
                .data
                .content()
                .parse::<toml_edit::DocumentMut>()
                .unwrap();
            let _doc = std::hint::black_box(_doc);
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_doc:?}");
        }
        Parser::De => {
            let _doc =
                toml::from_str::<toml_benchmarks::manifest::Manifest>(args.data.content()).unwrap();
            let _doc = std::hint::black_box(_doc);
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_doc:?}");
        }
        Parser::DeTable => {
            let mut _doc = toml::de::DeTable::parse(args.data.content()).unwrap();
            _doc.get_mut().make_owned();
            let _doc = std::hint::black_box(_doc);
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_doc:?}");
        }
        Parser::Table => {
            let _doc = args.data.content().parse::<toml::Table>().unwrap();
            let _doc = std::hint::black_box(_doc);
            #[cfg(debug_assertions)] // Don't interfere with profiling
            println!("{_doc:?}");
        }
    }
    Ok(())
}

struct Args {
    parser: Parser,
    data: toml_benchmarks::Data<'static>,
}

impl Args {
    fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut parser = Parser::Document;

        let mut args = lexopt::Parser::from_env();
        let mut data_name = "1-medium".to_owned();
        while let Some(arg) = args.next()? {
            match arg {
                Long("parser") => {
                    let value = args.value()?;
                    parser = match &value.to_str() {
                        Some("tokens") => Parser::Tokens,
                        Some("events") => Parser::Events,
                        Some("decoded") => Parser::Decoded,
                        Some("document") => Parser::Document,
                        Some("de") => Parser::De,
                        Some("detable") => Parser::DeTable,
                        Some("table") => Parser::Table,
                        _ => {
                            return Err(lexopt::Error::UnexpectedValue {
                                option: "parser".to_owned(),
                                value: value.clone(),
                            });
                        }
                    };
                }
                Long("manifest") => {
                    data_name = args.value()?.string()?;
                }
                _ => return Err(arg.unexpected()),
            }
        }
        let data = toml_benchmarks::MANIFESTS
            .iter()
            .find(|d| d.name() == data_name)
            .ok_or_else(|| lexopt::Error::UnexpectedValue {
                option: "manifest".to_owned(),
                value: data_name.into(),
            })?;

        Ok(Self {
            parser,
            data: *data,
        })
    }
}

enum Parser {
    Tokens,
    Events,
    Decoded,
    Document,
    De,
    DeTable,
    Table,
}
