fn main() -> Result<(), lexopt::Error> {
    let args = Args::parse()?;

    match args.parser {
        Parser::Tokens => {
            let source = ::toml_parse::Source::new(args.data.content());
            let _tokens = source.lex().into_vec();
            let _tokens = std::hint::black_box(_tokens);
            #[cfg(debug_assertions)] // Don't interefere with profiling
            println!("{_tokens:?}");
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
                        Some("document") => Parser::Document,
                        Some("de") => Parser::De,
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
    Document,
    De,
    Table,
}
