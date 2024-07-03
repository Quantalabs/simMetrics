use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use similarity_metrics::measures::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Similarity Metrics");

    for i in (0..=4096).step_by(256) {
        let f1: Vec<u8> = (0..i)
            .map(|_| thread_rng().gen_range(0.0f64..1.0).round() as u8)
            .collect();
        let f2: Vec<u8> = (0..i)
            .map(|_| thread_rng().gen_range(0.0f64..1.0).round() as u8)
            .collect();

        group.bench_with_input(
            BenchmarkId::new("Tanimoto", i),
            &(f1.clone(), f2.clone()),
            |b, (f1, f2)| b.iter(|| tanimoto(f1, f2)),
        );
        group.bench_with_input(
            BenchmarkId::new("Euclidean", i),
            &(f1.clone(), f2.clone()),
            |b, (f1, f2)| b.iter(|| euclidean(f1, f2)),
        );
        group.bench_with_input(
            BenchmarkId::new("Hamming", i),
            &(f1.clone(), f2.clone()),
            |b, (f1, f2)| b.iter(|| hamming(f1, f2)),
        );
        group.bench_with_input(
            BenchmarkId::new("Dice", i),
            &(f1.clone(), f2.clone()),
            |b, (f1, f2)| b.iter(|| dice(f1, f2)),
        );
        group.bench_with_input(
            BenchmarkId::new("Cosine", i),
            &(f1.clone(), f2.clone()),
            |b, (f1, f2)| b.iter(|| cosine(f1, f2)),
        );
        group.bench_with_input(
            BenchmarkId::new("Russell RAO", i),
            &(f1.clone(), f2.clone()),
            |b, (f1, f2)| b.iter(|| russell_rao(f1, f2)),
        );
        group.bench_with_input(
            BenchmarkId::new("Forbes", i),
            &(f1.clone(), f2.clone()),
            |b, (f1, f2)| b.iter(|| forbes(f1, f2)),
        );
        group.bench_with_input(
            BenchmarkId::new("Soergel", i),
            &(f1.clone(), f2.clone()),
            |b, (f1, f2)| b.iter(|| soergel(f1, f2)),
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
