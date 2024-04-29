#![allow(elided_lifetimes_in_paths)]

mod toml_edit {
    use super::{manifest, Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn document(sample: &Data) -> ::toml_edit::DocumentMut {
        sample.content().parse().unwrap()
    }

    #[divan::bench(args=MANIFESTS)]
    fn manifest(sample: &Data) -> manifest::Manifest {
        ::toml_edit::de::from_str(sample.content()).unwrap()
    }
}

mod toml {
    use super::{manifest, Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn document(sample: &Data) -> ::toml::Value {
        sample.content().parse().unwrap()
    }

    #[divan::bench(args=MANIFESTS)]
    fn manifest(sample: &Data) -> manifest::Manifest {
        ::toml::de::from_str(sample.content()).unwrap()
    }
}

mod toml_v05 {
    use super::{manifest, Data, MANIFESTS};

    #[divan::bench(args=MANIFESTS)]
    fn document(sample: &Data) -> ::toml_old::Value {
        sample.content().parse().unwrap()
    }

    #[divan::bench(args=MANIFESTS)]
    fn manifest(sample: &Data) -> manifest::Manifest {
        ::toml_old::de::from_str(sample.content()).unwrap()
    }
}

fn main() {
    divan::main();
}

#[derive(Debug)]
pub struct Data(&'static str, &'static str);

impl Data {
    pub const fn name(&self) -> &'static str {
        self.0
    }

    pub const fn content(&self) -> &'static str {
        self.1
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

const MANIFESTS: &[Data] = &[Data("0-minimal", MINIMAL), Data("1-medium", MEDIUM)];

const MINIMAL: &str = r#"
[package]
name = "bar"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
"#;

const MEDIUM: &str = include_str!("Cargo.cargo.toml");

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
