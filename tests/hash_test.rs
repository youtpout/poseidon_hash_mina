use num_bigint::BigInt;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use poseidon_hash_mina::{Fp, Sponge};
    use std::str::FromStr;

    // Convert a decimal string (including negative) to Fp via mod p
    fn fp(s: &str) -> Fp {
        let p = BigInt::from_str(
            "28948022309329048855892746252171976963363056481941560715954676764349967630337",
        )
        .unwrap();
        let n = BigInt::from_str(s).unwrap();
        let n = ((n % &p) + &p) % &p; // handles negatives
        let (_, bytes) = n.to_bytes_be();
        let mut arr = [0u8; 32];
        arr[32 - bytes.len()..].copy_from_slice(&bytes);
        Fp::from_be_bytes(arr)
    }

    fn expected(s: &str) -> Fp {
        fp(s)
    }

    #[test]
    fn test_poseidon_hash_empty() {
        let result = Sponge::hash(&[]);
        println!("Hash of empty input: {:?}", result.to_be_bytes());
    }

    #[test]
    fn test_poseidon_hash_single_element() {
        let result = Sponge::hash(&[Fp::from(12u64)]);
        assert_eq!(
            result,
            expected("20307190475163560179843878304233687113040243867319358507811895775846718326775")
        );
        println!("Hash of [12]: {:?}", result.to_be_bytes());
    }

    #[test]
    fn test_poseidon_hash_two_elements() {
        let result = Sponge::hash(&[Fp::from(3412u64), Fp::from(548748548u64)]);
        assert_eq!(
            result,
            expected("24245350037390325723675562428846509781869515058976947458013661211417354108422")
        );
        println!("Hash of [3412, 548748548]: {:?}", result.to_be_bytes());
    }

    #[test]
    fn test_poseidon_hash_multiple_elements() {
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
        let result = Sponge::hash(&inputs);
        assert_eq!(
            result,
            expected("9477952037298579560464731632706228830146195233473478290570101327698566553181")
        );
        println!("Hash of multiple elements: {:?}", result.to_be_bytes());
    }

    #[test]
    fn test_poseidon_hash_negative_elements() {
        let inputs = [
            fp("-35545"),
            Fp::ZERO,
            fp("-7878454"),
            fp("45524"),
            fp("-1"),
        ];
        let result = Sponge::hash(&inputs);
        assert_eq!(
            result,
            expected("17944732201997716732580423582703197695777318095974149487644452711464169895343")
        );
        println!("Hash of negative elements: {:?}", result.to_be_bytes());
    }
}