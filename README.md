# poseidon_hash_mina

[Adapted from the c# code](https://github.com/youtpout/MinaSignerNet/blob/77bc07d79d186b0eb0a2031cc7092581ba8f61c3/MinaSignerNet/MinPoseidon.cs#L11)

The goal of this project is to find the most simple implementation for mina poseidon hash, currenly we use bigint, we can probably replace all bigint by u8 array or vector, mina field size is 255 bits

## Benchmark
poseidon_group/poseidon_hash
                        time:   [198.28 µs 199.33 µs 200.27 µs]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild