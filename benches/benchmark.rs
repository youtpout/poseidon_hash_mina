use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poseidon_hash_mina::{Fp, Sponge};

fn fp(s: &str) -> Fp {
    use num_bigint::BigInt;
    use std::str::FromStr;
    let p = BigInt::from_str(
        "28948022309329048855892746252171976963363056481941560715954676764349967630337",
    )
    .unwrap();
    let n = BigInt::from_str(s).unwrap();
    let n = ((n % &p) + &p) % &p;
    let (_, bytes) = n.to_bytes_be();
    let mut arr = [0u8; 32];
    arr[32 - bytes.len()..].copy_from_slice(&bytes);
    Fp::from_be_bytes(arr)
}

fn benchmark_poseidon_small(c: &mut Criterion) {
    let inputs = [Fp::from(3412u64), Fp::from(548_748_548u64)];

    c.bench_function("poseidon_hash_2_elements", |b| {
        b.iter(|| Sponge::hash(black_box(&inputs)))
    });
}

fn benchmark_poseidon_large(c: &mut Criterion) {
    let inputs = [
        fp("7263514276861361464633452875109919113182485937660059416421780822303488364810"),
        Fp::ZERO,
        fp("14045753958617862754670070034440311287432747428158303518301078328357441472472"),
        Fp::from(1u64),
        Fp::ZERO,
        Fp::ZERO,
        Fp::from(1_000_000_000u64),
        fp("23155254961521224263414982804952607540765699032359833230215492926554005931079"),
        Fp::ZERO,
        Fp::from(8_000_000_000u64),
        Fp::from(1_000_000_000u64),
        Fp::ZERO,
        Fp::from(1u64),
    ];

    c.bench_function("poseidon_hash_13_elements", |b| {
        b.iter(|| Sponge::hash(black_box(&inputs)))
    });
}

fn benchmark_fp_mul(c: &mut Criterion) {
    let a = fp("7263514276861361464633452875109919113182485937660059416421780822303488364810");
    let b = fp("14045753958617862754670070034440311287432747428158303518301078328357441472472");

    c.bench_function("fp_mul", |b_| {
        b_.iter(|| black_box(a).mul(black_box(b)))
    });
}

fn benchmark_fp_pow7(c: &mut Criterion) {
    let a = fp("7263514276861361464633452875109919113182485937660059416421780822303488364810");

    c.bench_function("fp_pow7", |b| {
        b.iter(|| black_box(a).pow7())
    });
}

fn benchmark_permute(c: &mut Criterion) {
    c.bench_function("poseidon_permute", |b| {
        b.iter(|| {
            let mut s = Sponge::new();
            s.permute();
            black_box(s.squeeze())
        })
    });
}

criterion_group!(
    benches,
    benchmark_poseidon_small,
    benchmark_poseidon_large,
    benchmark_fp_mul,
    benchmark_fp_pow7,
    benchmark_permute,
);
criterion_main!(benches);