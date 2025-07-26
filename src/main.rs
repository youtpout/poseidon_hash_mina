use num_bigint::BigInt;
use poseidon_hash_mina::PoseidonHash;

// Example usage
fn main() {
    // Example: Hash a single value
    let input = vec![BigInt::from(12)];
    let hash_result = PoseidonHash::hash(input);
    println!("Poseidon hash of [12]: {}", hash_result);

    // Example: Hash multiple values
    let input = vec![BigInt::from(3412), BigInt::from(548748548)];
    let hash_result = PoseidonHash::hash(input);
    println!("Poseidon hash of [3412, 548748548]: {}", hash_result);
}
