use criterion::{Criterion, black_box, criterion_group, criterion_main};
use num_bigint::BigUint;
use poseidon_hash_mina::PoseidonHash;
// Import PoseidonHash from its module (update the path if needed)

fn benchmark_poseidon_hashes(c: &mut Criterion) {
    let input: Vec<BigUint> = vec![BigUint::from(3412u32), BigUint::from(548_748_548u32)];

    let mut group = c.benchmark_group("poseidon_group");

    group.bench_function("poseidon_hash", |b| {
        b.iter(|| {
            let input_clone = black_box(input.clone());
           let _ = PoseidonHash::hash(input_clone.clone());
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_poseidon_hashes);
criterion_main!(benches);
