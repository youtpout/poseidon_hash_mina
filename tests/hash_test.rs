use num_bigint::BigInt;
use std::str::FromStr;
use poseidon_hash_mina::PoseidonHash;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_poseidon_hash_empty() {
        let input = vec![];
        let result = PoseidonHash::hash(input);
        println!("Hash of empty input: {}", result);
    }

    #[test]
    fn test_poseidon_hash_single_element() {
        let input = vec![BigInt::from(12)];
        let result: BigInt = PoseidonHash::hash(input);
        let exepected: BigInt = BigInt::from_str(
            "20307190475163560179843878304233687113040243867319358507811895775846718326775",
        )
        .unwrap();
        assert_eq!(result, exepected);
        println!("Hash of [12]: {}", result);
    }

    #[test]
    fn test_poseidon_hash_two_elements() {
        let input = vec![BigInt::from(3412), BigInt::from(548748548)];
        let result = PoseidonHash::hash(input);
        let exepected: BigInt = BigInt::from_str(
            "24245350037390325723675562428846509781869515058976947458013661211417354108422",
        )
        .unwrap();
        assert_eq!(result, exepected);
        println!("Hash of [3412, 548748548]: {}", result);
    }
}
