# poseidon_hash_mina

[Adapted from the c# code](https://github.com/youtpout/MinaSignerNet/blob/77bc07d79d186b0eb0a2031cc7092581ba8f61c3/MinaSignerNet/MinPoseidon.cs#L11)

The goal of this project is to find the most simple implementation for mina poseidon hash, currenly we use bigint, we can probably replace all bigint by u8 array or vector, mina field size is 255 bits

## Benchmark

Ryzen 7 7950x on wsl ubuntu 24.04

poseidon_group/poseidon_hash
                        time:   [198.28 µs 199.33 µs 200.27 µs]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild


Use fix array and constant byte array for mds

poseidon_group/poseidon_hash
                        time:   [188.94 µs 189.22 µs 189.52 µs]
                        change: [+1.6061% +1.9154% +2.2505%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking poseidon_group/big_poseidon_hash: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.4s, enable flat sampling, or reduce sample count to 60.
poseidon_group/big_poseidon_hash
                        time:   [1.2665 ms 1.2688 ms 1.2710 ms]
                        change: [-4.1361% -3.8699% -3.6106%] (p = 0.00 < 0.05)
                        Performance has improved.