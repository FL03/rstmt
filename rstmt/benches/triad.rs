/*
    Appellation: default <module>
    Contrib: @FL03
*/
use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion};
use rstmt::nrt::{Triad, TriadBase};
use rstmt::{Augmented, Diminished, Major, Minor};

const SAMPLES: usize = 50;
/// the default number of seconds a benchmark should complete in
const DURATION: u64 = 10;
/// benchmark initialization routines for creating triads
fn bench_triad_create(c: &mut Criterion) {
    // declare a group for transformations
    let mut group = c.benchmark_group("TriadBase::create");
    // configure the group
    group
        .sample_size(SAMPLES)
        .measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the inverse chain on a major triad created using different root notes
    for n in 0..12 {
        group.bench_function(BenchmarkId::new("major", n), |b| {
            b.iter(|| TriadBase::from_root_with_class(black_box(n), Major));
        });
        group.bench_function(BenchmarkId::new("minor", n), |b| {
            b.iter(|| TriadBase::from_root_with_class(black_box(n), Minor));
        });
        group.bench_function(BenchmarkId::new("diminished", n), |b| {
            b.iter(|| TriadBase::from_root_with_class(black_box(n), Diminished));
        });
        group.bench_function(BenchmarkId::new("augmented", n), |b| {
            b.iter(|| TriadBase::from_root_with_class(black_box(n), Augmented));
        });
    }

    group.finish();
}
/// benchmark for LPR transformations on major / minor triads
fn bench_triad_transform(c: &mut Criterion) {
    // declare a group for transformations
    let mut group = c.benchmark_group("TriadBase::transform");
    // configure the group
    group
        .sample_size(SAMPLES)
        .measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the inverse chain on a major triad created using different root notes
    for n in 0..12 {
        group.bench_with_input(BenchmarkId::new("major:leading", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::major(x),
                |triad| {
                    black_box(triad.leading().leading());
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("major:parallel", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::major(x),
                |triad| {
                    black_box(triad.parallel().parallel());
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("major:relative", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::major(x),
                |triad| {
                    black_box(triad.relative().relative());
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("minor:leading", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::minor(x),
                |triad| {
                    black_box(triad.leading().leading());
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("minor:parallel", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::minor(x),
                |triad| {
                    black_box(triad.parallel().parallel());
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("minor:relative", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::minor(x),
                |triad| {
                    black_box(triad.relative().relative());
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}
// initialize the benchmark group
criterion::criterion_group! {
    benches,
    bench_triad_create,
    bench_triad_transform,
}
// This macro expands to a function named `benches`, which uses the given config
criterion::criterion_main! {
    benches,
}
