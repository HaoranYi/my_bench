use criterion::{black_box, criterion_group, criterion_main, Criterion};

// benchmark solana account hash performance
fn criterion_benchmark(c: &mut Criterion) {
    let bytes: Vec<u8> = (0..=255_u8).collect();

    c.bench_function("h1", |b| {
        b.iter(|| {
            //blake3::hash(&bytes[0..89]
            let mut hasher = blake3::Hasher::new();
            hasher.update(&bytes[0..24]);

            hasher.update(&bytes[24..89]);
            hasher.finalize()
        })
    });
    c.bench_function("h2", |b| {
        b.iter(|| {
            let mut hasher = blake3::Hasher::new();
            hasher.update(&bytes[0..8]);
            hasher.update(&bytes[8..16]);
            hasher.update(&bytes[16..24]);
            hasher.update(&bytes[24..25]);
            hasher.update(&bytes[25..57]);
            hasher.update(&bytes[57..89]);
            hasher.finalize()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
