use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use openbabel::fingerprint::Kind;
use similarity_metrics::{dist, load, measures};

fn run_metrics<T>(metric: fn(&[u32], &[u32]) -> T, fps: Vec<Vec<u32>>) -> Vec<T> {
    fps.iter()
        .flat_map(|x1| {
            fps.iter()
                .filter(move |x2| &x1 != x2)
                .map(move |x2| metric(x1, x2))
        })
        .collect::<Vec<T>>()
}

fn run_metrics_selfies(metric: fn(&str, &str) -> usize, fps: Vec<String>) -> Vec<usize> {
    fps.iter()
        .flat_map(|x1| {
            fps.iter()
                .filter(move |x2| &x1 != x2)
                .map(move |x2| metric(x1, x2))
        })
        .collect()
}

fn bench<T>(metric: fn(&[u32], &[u32]) -> T, fp: Kind) -> Vec<T> {
    let fps = load::gen_fps(fp, "test.mol");

    run_metrics(metric, fps)
}

fn bench_selfies(metric: fn(&str, &str) -> usize) -> Vec<usize> {
    let fps = load::load_plain("selfies_test.mol");

    run_metrics_selfies(metric, fps)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Similarity Metrics");
    let fps = vec![
        (Kind::FP2 { nbits: 2048 }, "Daylight (Clone)"),
        (Kind::ECFP2 { nbits: 2048 }, "ECFP (Radius 2)"),
        (Kind::ECFP4 { nbits: 2048 }, "ECFP (Radius 4)"),
        (Kind::ECFP6 { nbits: 2048 }, "ECFP (Radius 6)"),
        (Kind::ECFP8 { nbits: 2048 }, "ECFP (Radius 8)"),
        (Kind::ECFP10 { nbits: 2048 }, "ECFP (Radius 10)"),
    ];

    for fp in fps {
        group.bench_with_input(BenchmarkId::new("Tanimoto", fp.1), fp.1, |b, _i| {
            b.iter(|| bench(measures::tanimoto, fp.0.clone()))
        });
        group.bench_with_input(BenchmarkId::new("Soergel", fp.1), fp.1, |b, _i| {
            b.iter(|| bench(measures::soergel, fp.0.clone()))
        });
        group.bench_with_input(BenchmarkId::new("Russell RAO", fp.1), fp.1, |b, _i| {
            b.iter(|| bench(measures::russell_rao, fp.0.clone()))
        });
        group.bench_with_input(BenchmarkId::new("Hamming", fp.1), fp.1, |b, _i| {
            b.iter(|| bench(measures::hamming, fp.0.clone()))
        });
        group.bench_with_input(BenchmarkId::new("Forbes", fp.1), fp.1, |b, _i| {
            b.iter(|| bench(measures::forbes, fp.0.clone()))
        });
        group.bench_with_input(BenchmarkId::new("Euclidean", fp.1), fp.1, |b, _i| {
            b.iter(|| bench(measures::euclidean, fp.0.clone()))
        });
        group.bench_with_input(BenchmarkId::new("Dice", fp.1), fp.1, |b, _i| {
            b.iter(|| bench(measures::dice, fp.0.clone()))
        });
        group.bench_with_input(BenchmarkId::new("Cosine", fp.1), fp.1, |b, _i| {
            b.iter(|| bench(measures::cosine, fp.0.clone()))
        });
    }

    let edit_distances: Vec<(&str, fn(&str, &str) -> usize)> = vec![
        ("Hamming (With Selfies)", dist::hamming_strings),
        ("LCS", dist::lcs),
        ("Levenshtein", dist::l_distance),
    ];

    for distance in edit_distances {
        group.bench_function(distance.0, |b| b.iter(|| bench_selfies(distance.1)));
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
