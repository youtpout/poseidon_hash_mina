use criterion::{Criterion, black_box, criterion_group, criterion_main};
use num_bigint::{BigInt, BigUint};
use poseidon_hash_mina::PoseidonHash;
use std::str::FromStr;
// Import PoseidonHash from its module (update the path if needed)

fn benchmark_poseidon_hashes(c: &mut Criterion) {
    let input = vec![BigInt::from(3412), BigInt::from(548_748_548)];

    let mut group = c.benchmark_group("poseidon_group");

    group.bench_function("poseidon_hash", |b| {
        b.iter(|| {
            let input_clone = black_box(input.clone());
            let _ = PoseidonHash::hash(input_clone.clone());
        });
    });

    group.finish();
}

fn benchmark_big_poseidon_hashes(c: &mut Criterion) {
    let input = vec![
        BigInt::from_str(
            "7263514276861361464633452875109919113182485937660059416421780822303488364810",
        )
        .unwrap(),
        BigInt::ZERO,
        BigInt::from_str(
            "14045753958617862754670070034440311287432747428158303518301078328357441472472",
        )
        .unwrap(),
        BigInt::from(1u32),
        BigInt::ZERO,
        BigInt::ZERO,
        BigInt::from(1000000000u32),
        BigInt::from_str(
            "23155254961521224263414982804952607540765699032359833230215492926554005931079",
        )
        .unwrap(),
        BigInt::ZERO,
        BigInt::from(8000000000u64),
        BigInt::from(1000000000u32),
        BigInt::ZERO,
        BigInt::from(1u32),
    ];

    let mut group = c.benchmark_group("poseidon_group");

    group.bench_function("big_poseidon_hash", |b| {
        b.iter(|| {
            let input_clone = black_box(input.clone());
            let _ = PoseidonHash::hash(input_clone.clone());
        });
    });

    group.finish();
}

fn bench_from_str(c: &mut Criterion) {
    let val_str = "12035446894107573964500871153637039653510326950134440362813193268448863222019";

    c.bench_function("BigInt::from_str", |b| {
        b.iter(|| {
            let _ = BigInt::from_str(val_str).unwrap();
        })
    });
}

fn bench_bigint_from_str(c: &mut Criterion) {
    let val_str = "12035446894107573964500871153637039653510326950134440362813193268448863222019";

    c.bench_function("BigInt::from_str", |b| {
        b.iter(|| {
            let _ = BigInt::from_str(val_str).unwrap();
        })
    });
}

fn bench_bigint_from_bytes(c: &mut Criterion) {
    let val_str = "12035446894107573964500871153637039653510326950134440362813193268448863222019";
    let bigint = BigInt::from_str(val_str).unwrap();
    let bytes = bigint.to_signed_bytes_be();

    c.bench_function("BigInt::from_signed_bytes_be", |b| {
        b.iter(|| {
            let _ = BigInt::from_signed_bytes_be(&bytes);
        })
    });
}

fn bench_biguint_from_str(c: &mut Criterion) {
    let val_str = "12035446894107573964500871153637039653510326950134440362813193268448863222019";

    c.bench_function("BigUint::from_str", |b| {
        b.iter(|| {
            let _ = BigUint::from_str(val_str).unwrap();
        })
    });
}

fn bench_biguint_from_bytes(c: &mut Criterion) {
    let val_str = "12035446894107573964500871153637039653510326950134440362813193268448863222019";
    let biguint = BigUint::from_str(val_str).unwrap();
    let bytes = biguint.to_bytes_be();

    c.bench_function("BigUint::from_bytes_be", |b| {
        b.iter(|| {
            let _ = BigUint::from_bytes_be(&bytes);
        })
    });
}

criterion_group!(
    benches,
    benchmark_poseidon_hashes,
    benchmark_big_poseidon_hashes,
    bench_bigint_from_str,
    bench_bigint_from_bytes,
    bench_biguint_from_str,
    bench_biguint_from_bytes
);
criterion_main!(benches);
