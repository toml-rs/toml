#[derive(Copy, Clone, Debug)]
/// Represents a named piece of data, e.g., a manifest file.
pub struct Data<'s> {
    /// Name of the data entry
    pub name: &'s str,
    /// Content associated with the data entry
    pub content: &'s str,
}

impl<'s> Data<'s> {
    /// Returns the name of the data
    pub const fn name(&self) -> &'s str {
        self.name
    }

    /// Returns the content of the data
    pub const fn content(&self) -> &'s str {
        self.content
    }
}

impl std::fmt::Display for Data<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

pub const MANIFESTS: &[Data<'static>] = &[
    Data {
        name: "0-new",
        content: NEW,
    },
    Data {
        name: "1-medium",
        content: MEDIUM,
    },
    Data {
        name: "2-features",
        content: FEATURES,
    },
];

/// New minimal Cargo manifest
const NEW: &str = r#"
[package]
name = "bar"
version = "0.1.0"
edition = "2018"

[dependencies]
"#;

/// Medium manifest included from external file
const MEDIUM: &str = include_str!("Cargo.cargo.toml");

/// Features manifest included from external file
const FEATURES: &str = include_str!("Cargo.web-sys.toml");

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
