//! Benchmarks for rendering-related calculations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Benchmark coordinate conversions
fn bench_coordinate_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("coordinate_conversions");

    group.bench_function("isize_to_f32", |b| {
        b.iter(|| {
            let mut sum = 0.0_f32;
            for i in -100..100_isize {
                sum += black_box(i as f32);
            }
            black_box(sum)
        });
    });

    group.bench_function("f32_to_isize", |b| {
        b.iter(|| {
            let mut sum = 0_isize;
            for i in -100..100 {
                let f = i as f32;
                sum += black_box(f.round() as isize);
            }
            black_box(sum)
        });
    });

    group.finish();
}

/// Benchmark grid line calculations
fn bench_grid_calculations(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_calculations");

    for grid_size in [10, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(grid_size),
            grid_size,
            |b, &size| {
                b.iter(|| {
                    let mut line_count = 0;
                    for x in -size..=size {
                        for y in -size..=size {
                            // Simulate grid line position calculation
                            let _pos_x = black_box(x as f32 - 0.5);
                            let _pos_y = black_box(y as f32 - 0.5);
                            line_count += 1;
                        }
                    }
                    black_box(line_count)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_coordinate_conversions, bench_grid_calculations);
criterion_main!(benches);
