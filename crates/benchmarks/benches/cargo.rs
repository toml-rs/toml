#![allow(elided_lifetimes_in_paths)]

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
