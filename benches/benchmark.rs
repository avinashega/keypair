use criterion::{criterion_group, criterion_main, Criterion};
use keypair_secp256k1::generate_keypair_and_address;

fn benchmark_keypair_generation(c: &mut Criterion) {
    c.bench_function("keypair_generation", |b| {
        b.iter(|| {
            generate_keypair_and_address()
        })
    });
}

criterion_group!(benches, benchmark_keypair_generation);
criterion_main!(benches);