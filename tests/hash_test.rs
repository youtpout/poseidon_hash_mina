use num_bigint::BigInt;
use poseidon_hash_mina::PoseidonHash;
use std::str::FromStr;

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

    #[test]
    fn test_poseidon_hash_multiple_elements() {
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
        let result = PoseidonHash::hash(input);
        let exepected: BigInt = BigInt::from_str(
            "9477952037298579560464731632706228830146195233473478290570101327698566553181",
        )
        .unwrap();
        assert_eq!(result, exepected);
        println!("Hash of [3412, 548748548]: {}", result);
    }
}
