/*
    Appellation: default <module>
    Contrib: @FL03
*/
use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion};
use rstmt::nrt::Triad;

const SAMPLES: usize = 50;
/// the default number of seconds a benchmark should complete in
const DURATION: u64 = 10;
/// benchmark for LPR transformations
fn bench_lpr_transformations(c: &mut Criterion) {
    // declare a group for transformations
    let mut group = c.benchmark_group("LPR");
    // configure the group
    group
        .sample_size(SAMPLES)
        .measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the inverse chain on a major triad created using different root notes
    for n in 0..12 {
        group.bench_with_input(BenchmarkId::new("leading", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::major(x),
                |triad| {
                    black_box(triad.leading().leading());
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("parallel", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::major(x),
                |triad| {
                    black_box(triad.parallel().parallel());
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_with_input(BenchmarkId::new("relative", n), &n, |b, &x| {
            b.iter_batched(
                || Triad::major(x),
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
criterion::criterion_group! { benches,
    bench_lpr_transformations,
}
// This macro expands to a function named `benches`, which uses the given config
criterion::criterion_main! {
    benches,
}
