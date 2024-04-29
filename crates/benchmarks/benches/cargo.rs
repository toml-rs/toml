#![allow(elided_lifetimes_in_paths)]

mod toml_edit {
    use toml_benchmarks::{manifest, Data, MANIFESTS};

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
    use toml_benchmarks::{manifest, Data, MANIFESTS};

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
    use toml_benchmarks::{manifest, Data, MANIFESTS};

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
