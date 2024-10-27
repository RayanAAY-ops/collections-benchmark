use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn generate_hashmap(sample_size: usize) -> HashMap<u16, u16> {
    let mut rng = thread_rng();

    // Generate random keys and values
    let keys: Vec<_> = (0..sample_size).map(|_| rng.gen::<u16>()).collect();
    let values: Vec<_> = (0..sample_size).map(|_| rng.gen::<u16>()).collect();

    // Create HashMap from keys and values
    keys.into_iter().zip(values.into_iter()).collect()
}

fn generate_vec(sample_size: usize) -> Vec<u16> {
    let mut rng = thread_rng();

    // Generate random values
    (0..sample_size).map(|_| rng.gen::<u16>()).collect()
}

// Benchmark for HashMap lookup
fn benchmark_hashmap_lookup(c: &mut Criterion) {
    let sample_size = 100;
    let hashmap = generate_hashmap(sample_size);
    let keys: Vec<_> = hashmap.keys().copied().collect(); // Use keys for lookup

    c.bench_function("HashMap lookup", |b| {
        b.iter(|| {
            for &key in &keys {
                let _ = hashmap.get(&key);
            }
        });
    });
}

// Benchmark for Vec lookup
fn benchmark_vec_lookup(c: &mut Criterion) {
    let sample_size = 100;
    let vector = generate_vec(sample_size);
    let indices: Vec<usize> = (0..sample_size).collect(); // Use indices for lookup

    c.bench_function("Vec lookup", |b| {
        b.iter(|| {
            for &index in &indices {
                let _ = vector.get(index);
            }
        });
    });
}

// Criterion main function
criterion_group!(benches, benchmark_hashmap_lookup, benchmark_vec_lookup);
criterion_main!(benches);
