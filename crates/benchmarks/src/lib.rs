#[derive(Copy, Clone, Debug)]
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

pub const MANIFESTS: &[Data] = &[Data("0-new", NEW), Data("1-medium", MEDIUM)];

const NEW: &str = r#"
[package]
name = "bar"
version = "0.1.0"
edition = "2018"

[dependencies]
"#;

const MEDIUM: &str = include_str!("Cargo.cargo.toml");

pub mod manifest {
    use std::collections::HashMap;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Manifest {
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
        name: Option<String>,
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
