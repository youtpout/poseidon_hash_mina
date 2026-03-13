// src/fp.rs
use crypto_bigint::{AddMod, MulMod, SubMod, U256};

// Pallas modulus p, little-endian (crypto-bigint uses big-endian hex literals)
// p = 2^254 + 45560315531419706090280762371685220353
const MODULUS: U256 =
    U256::from_be_hex("40000000000000000000000000000000224698fc094cf91b992d30ed00000001");

// Precomputed: p - 2, for Fermat inverse (x^(p-2) mod p)
const MODULUS_MINUS_2: U256 =
    U256::from_be_hex("40000000000000000000000000000000224698fc094cf91b992d30ecffffffff");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp(U256);

impl Fp {
    pub const ZERO: Self = Fp(U256::ZERO);
    pub const ONE: Self = Fp(U256::ONE);

    pub const fn from_be_hex(s: &str) -> Self {
        Fp(U256::from_be_hex(s))
    }

    // -- Arithmetic --
    // add_mod / sub_mod: pure carry arithmetic in crypto-bigint, no syscall needed
    #[inline(always)]
    pub fn add(self, rhs: Self) -> Self {
        Fp(self.0.add_mod(&rhs.0, &MODULUS))
    }

    #[inline(always)]
    pub fn sub(self, rhs: Self) -> Self {
        Fp(self.0.sub_mod(&rhs.0, &MODULUS))
    }

    // mul_mod: → syscall_uint256_mulmod in zkVM via SP1 patch, schoolbook on native
    #[inline(always)]
    pub fn mul(self, rhs: Self) -> Self {
        Fp(self.0.mul_mod(&rhs.0, &MODULUS))
    }

    // S-box: x^7 with 4 multiplications (squaring chain)
    // x^2 → x^4 → x^6 → x^7
    #[inline(always)]
    pub fn pow7(self) -> Self {
        let x2 = self.mul(self);
        let x4 = x2.mul(x2);
        let x6 = x4.mul(x2);
        x6.mul(self)
    }

    // Serialization helpers
    pub fn to_be_bytes(self) -> [u8; 32] {
        self.0.to_be_bytes()
    }

    pub fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Fp(U256::from_be_bytes(bytes))
    }

    pub fn from(v: u64) -> Self {
        Fp(U256::from(v))
    }
}
