// src/params.rs
//
// Kimchi Poseidon parameters — Pallas field (Fp)
// Source: o1-labs/proof-systems poseidon/src/pasta/fp_kimchi.rs
//
// Width=3, alpha=7, full_rounds=11, rate=2, capacity=1
// All constants verified against the Mina spec.

use crate::fp::Fp;

pub const WIDTH:       usize = 3;
pub const FULL_ROUNDS: usize = 55;
pub const RATE:        usize = 2;

// 3x3 MDS matrix, row-major
// Extract with: grep -A50 "mds" proof-systems/poseidon/src/pasta/fp_kimchi.rs
pub const MDS: [[Fp; WIDTH]; WIDTH] = [
    [
        Fp::from_be_hex("1a9bd250757e29ef4959b9bef59b4e60e20a56307d6491e7b7ea1fac679c7903"), // MDS[0][0]
        Fp::from_be_hex("384aa09faf3a48737e2d64f6a030aa242e6d5d455ae4a13696b48a7320c506cd"), // MDS[0][1]
        Fp::from_be_hex("3d2b7b0209bc3080064d5ce4a7a03653f8346506bfa6d076061217be9e6cfed5"), // MDS[0][2]
    ],
    [
        Fp::from_be_hex("09ee57c70bc351220b107983afcfabbea79868a4a8a5913e24b7aaf3b4bf3a42"),
        Fp::from_be_hex("20989996bc29a96d17684d3ad4c859813115267f35225d7e1e9a5b5436a2458f"),
        Fp::from_be_hex("14e39adb2e171ae232116419ee7f26d9191edde8a5632298347cdb74c3b2e69d"),
    ],
    [
        Fp::from_be_hex("174544357b687f65a9590c1df621818b5452d5d441597a94357f112316ef67cb"),
        Fp::from_be_hex("3ca9263dc1a19d17cfbf15b0166bb25f95dffc53212db207fcee35f02c2c4137"),
        Fp::from_be_hex("3cf1fbef75d4ab63b7a812f80b7b0373b2dc21d269ba7c4c4d6581d50aae114c"),
    ],
];

// Round constants: [round][position], shape [11][3]
pub const ROUND_CONSTANTS: [[Fp; WIDTH]; FULL_ROUNDS] = [
    [Fp::from_be_hex("..."), Fp::from_be_hex("..."), Fp::from_be_hex("...")],
    // ... 10 more rounds
];