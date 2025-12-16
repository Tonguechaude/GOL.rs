//! Benchmarks for Game of Life simulation core logic

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use gol_simulation::rules::{calculate_neighbor_counts, should_cell_be_born, should_cell_survive};
use gol_simulation::CellPosition;
use rustc_hash::FxHashSet;

/// Generate a random pattern of alive cells for benchmarking
fn generate_pattern(size: usize) -> Vec<CellPosition> {
    let mut pattern = Vec::new();
    for x in 0..size as isize {
        for y in 0..size as isize {
            // Create a checkerboard-like pattern with some density
            if (x + y) % 3 == 0 {
                pattern.push(CellPosition { x, y });
            }
        }
    }
    pattern
}

/// Benchmark neighbor count calculation with different pattern sizes
fn bench_neighbor_counts(c: &mut Criterion) {
    let mut group = c.benchmark_group("neighbor_counts");

    for size in [10, 50, 100, 200].iter() {
        let pattern = generate_pattern(*size);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}x{}", size, size)),
            &pattern,
            |b, pattern| {
                b.iter(|| {
                    let result = calculate_neighbor_counts(black_box(pattern.iter().copied()));
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark rule evaluation (survival and birth)
fn bench_rules(c: &mut Criterion) {
    let mut group = c.benchmark_group("rules");

    group.bench_function("should_cell_survive", |b| {
        b.iter(|| {
            for i in 0..10 {
                black_box(should_cell_survive(black_box(i)));
            }
        });
    });

    group.bench_function("should_cell_be_born", |b| {
        b.iter(|| {
            for i in 0..10 {
                black_box(should_cell_be_born(black_box(i)));
            }
        });
    });

    group.finish();
}

/// Benchmark a complete generation cycle (neighbor counting + rule application)
fn bench_full_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_generation");

    for size in [10, 50, 100].iter() {
        let pattern = generate_pattern(*size);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}x{}", size, size)),
            &pattern,
            |b, pattern| {
                b.iter(|| {
                    let alive_positions: FxHashSet<CellPosition> =
                        pattern.iter().copied().collect();

                    // Calculate neighbor counts
                    let neighbor_counts =
                        calculate_neighbor_counts(black_box(alive_positions.iter().copied()));

                    // Determine deaths
                    let mut cells_to_kill = Vec::new();
                    for cell in &alive_positions {
                        let neighbor_count = neighbor_counts.get(cell).copied().unwrap_or(0);
                        if !should_cell_survive(neighbor_count) {
                            cells_to_kill.push(*cell);
                        }
                    }

                    // Determine births
                    let mut cells_to_spawn = Vec::new();
                    for (pos, count) in &neighbor_counts {
                        if should_cell_be_born(*count) && !alive_positions.contains(pos) {
                            cells_to_spawn.push(*pos);
                        }
                    }

                    black_box((cells_to_kill, cells_to_spawn))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark classic Game of Life patterns
fn bench_known_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("known_patterns");

    // Glider pattern
    let glider = vec![
        CellPosition { x: 0, y: 0 },
        CellPosition { x: 1, y: 0 },
        CellPosition { x: 2, y: 0 },
        CellPosition { x: 2, y: 1 },
        CellPosition { x: 1, y: 2 },
    ];

    group.bench_function("glider", |b| {
        b.iter(|| {
            let result = calculate_neighbor_counts(black_box(glider.iter().copied()));
            black_box(result)
        });
    });

    // Blinker pattern (oscillator)
    let blinker = vec![
        CellPosition { x: 0, y: 0 },
        CellPosition { x: 1, y: 0 },
        CellPosition { x: 2, y: 0 },
    ];

    group.bench_function("blinker", |b| {
        b.iter(|| {
            let result = calculate_neighbor_counts(black_box(blinker.iter().copied()));
            black_box(result)
        });
    });

    // Block pattern (still life)
    let block = vec![
        CellPosition { x: 0, y: 0 },
        CellPosition { x: 1, y: 0 },
        CellPosition { x: 0, y: 1 },
        CellPosition { x: 1, y: 1 },
    ];

    group.bench_function("block", |b| {
        b.iter(|| {
            let result = calculate_neighbor_counts(black_box(block.iter().copied()));
            black_box(result)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_neighbor_counts,
    bench_rules,
    bench_full_generation,
    bench_known_patterns
);
criterion_main!(benches);
