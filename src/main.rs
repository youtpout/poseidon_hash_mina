use num_bigint::BigUint;
use poseidon_hash_mina::PoseidonHash;

// Example usage
fn main() {
    // Example: Hash a single value
    let input: Vec<BigUint> = vec![BigUint::from(12u32)];
    let hash_result: BigUint = PoseidonHash::hash(input);
    println!("Poseidon hash of [12]: {}", hash_result);

    // Example: Hash multiple values
    let input: Vec<BigUint> = vec![BigUint::from(3412u32), BigUint::from(548748548u32)];
    let hash_result = PoseidonHash::hash(input);
    println!("Poseidon hash of [3412, 548748548]: {}", hash_result);
}
