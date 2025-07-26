use primitive_types::U256;
use poseidon_hash_mina::PoseidonHash;

// Example usage
fn main() {
    // Example: Hash a single value
    let input: Vec<U256> = vec![U256::from(12u32)];
    let hash_result: U256 = PoseidonHash::hash(input);
    println!("Poseidon hash of [12]: {}", hash_result);

    // Example: Hash multiple values
    let input: Vec<U256> = vec![U256::from(3412u32), U256::from(548748548u32)];
    let hash_result = PoseidonHash::hash(input);
    println!("Poseidon hash of [3412, 548748548]: {}", hash_result);
}
