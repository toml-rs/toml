fn main() -> Result<(), lexopt::Error> {
    let args = Args::parse()?;

    match args.parser {
        Parser::Document => {
            let _doc = CARGO_MANIFEST.parse::<toml_edit::DocumentMut>().unwrap();
            #[cfg(debug_assertions)] // Don't interefere with profiling
            drop(_doc);
        }
        Parser::De => {
            let _doc = toml::from_str::<manifest::Manifest>(CARGO_MANIFEST).unwrap();
            #[cfg(debug_assertions)] // Don't interefere with profiling
            drop(_doc);
        }
        Parser::Table => {
            let _doc = CARGO_MANIFEST.parse::<toml::Table>().unwrap();
            #[cfg(debug_assertions)] // Don't interefere with profiling
            drop(_doc);
        }
    }
    Ok(())
}

struct Args {
    parser: Parser,
}

impl Args {
    fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut parser = Parser::Document;

        let mut args = lexopt::Parser::from_env();
        while let Some(arg) = args.next()? {
            match arg {
                Long("parser") => {
                    let value = args.value()?;
                    parser = match &value.to_str() {
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
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(Self { parser })
    }
}

enum Parser {
    Document,
    De,
    Table,
}

mod manifest {
    use std::collections::HashMap;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub(crate) struct Manifest {
        package: Package,
        #[serde(default)]
        lib: Option<Lib>,
        #[serde(default)]
        bin: Vec<Bin>,
        #[serde(default)]
        features: HashMap<String, Vec<String>>,
        #[serde(default)]
        dependencies: HashMap<String, Dependency>,
        #[serde(default)]
        build_dependencies: HashMap<String, Dependency>,
        #[serde(default)]
        dev_dependencies: HashMap<String, Dependency>,
        #[serde(default)]
        target: HashMap<String, Target>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub(crate) struct Package {
        name: String,
        version: String,
        #[serde(default)]
        edition: Option<String>,
        #[serde(default)]
        authors: Vec<String>,
        #[serde(default)]
        license: Option<String>,
        #[serde(default)]
        homepage: Option<String>,
        #[serde(default)]
        repository: Option<String>,
        #[serde(default)]
        documentation: Option<String>,
        #[serde(default)]
        readme: Option<String>,
        #[serde(default)]
        description: Option<String>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub(crate) struct Lib {
        name: String,
        #[serde(default)]
        path: Option<String>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub(crate) struct Bin {
        name: String,
        #[serde(default)]
        test: bool,
        #[serde(default)]
        doc: bool,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    #[serde(untagged)]
    pub(crate) enum Dependency {
        Version(String),
        Full(DependencyFull),
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub(crate) struct DependencyFull {
        #[serde(default)]
        version: Option<String>,
        #[serde(default)]
        path: Option<String>,
        #[serde(default)]
        default_features: bool,
        #[serde(default)]
        optional: bool,
        #[serde(default)]
        features: Vec<String>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub(crate) struct Target {
        #[serde(default)]
        dependencies: HashMap<String, Dependency>,
        #[serde(default)]
        build_dependencies: HashMap<String, Dependency>,
        #[serde(default)]
        dev_dependencies: HashMap<String, Dependency>,
    }
}

const CARGO_MANIFEST: &str = include_str!("../benches/Cargo.cargo.toml");
