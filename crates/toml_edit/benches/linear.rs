use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn map(c: &mut Criterion) {
    let mut group = c.benchmark_group("map");
    let samples = [10, 100];
    for sample in samples {
        let mut s = String::new();
        for i in 0..sample {
            s += &format!("[header_no_{}]\n", i);
            s += "entry = 42\n"
        }
        let len = s.len();
        group.throughput(Throughput::Bytes(len as u64));

        group.bench_with_input(BenchmarkId::new("toml_edit", sample), &sample, |b, _| {
            let s = s.clone();
            s.parse::<toml_edit::Document>().unwrap();
            let s = black_box(s);
            b.iter(|| {
                black_box(s.parse::<toml_edit::Document>().unwrap());
            })
        });
        #[cfg(feature = "easy")]
        group.bench_with_input(
            BenchmarkId::new("toml_edit::easy", sample),
            &sample,
            |b, _| {
                let s = s.clone();
                s.parse::<toml_edit::easy::Value>().unwrap();
                let s = black_box(s);
                b.iter(|| {
                    black_box(s.parse::<toml_edit::easy::Value>().unwrap());
                })
            },
        );
        group.bench_with_input(BenchmarkId::new("toml", sample), &sample, |b, _| {
            let s = s.clone();
            s.parse::<toml::Value>().unwrap();
            let s = black_box(s);
            b.iter(|| {
                black_box(s.parse::<toml::Value>().unwrap());
            })
        });
    }
    group.finish();
}

fn array(c: &mut Criterion) {
    let mut group = c.benchmark_group("array");
    let samples = [10, 100];
    for sample in samples {
        let mut s = String::new();
        for _ in 0..sample {
            s += "[[header]]\n";
            s += "entry = 42\n"
        }
        let len = s.len();
        group.throughput(Throughput::Bytes(len as u64));

        group.bench_with_input(BenchmarkId::new("toml_edit", sample), &sample, |b, _| {
            let s = s.clone();
            s.parse::<toml_edit::Document>().unwrap();
            let s = black_box(s);
            b.iter(|| {
                black_box(s.parse::<toml_edit::Document>().unwrap());
            })
        });
        group.bench_with_input(BenchmarkId::new("toml", sample), &sample, |b, _| {
            let s = s.clone();
            s.parse::<toml::Value>().unwrap();
            let s = black_box(s);
            b.iter(|| {
                black_box(s.parse::<toml::Value>().unwrap());
            })
        });
    }
    group.finish();
}

criterion_group!(benches, map, array);
criterion_main!(benches);
