# Poseidon Hash Benchmarks in Rust
This is a repository comparing different Poseidon implementations in Rust. This project is supported by an [Ethereum Foundation](https://ethereum.foundation/) grant FY23-1156.

Repositories that are being used in this project are:
1. https://github.com/dusk-network/Poseidon252
2. https://github.com/CryptoExperts/poseidon
3. https://github.com/risc0/risc0 
4. https://github.com/lurk-lab/neptune

To run the benchmarks use the command `cargo +nightly bench`.

# Default Poseidon parameters

Here we present the default parameters of the different instantiations of Poseidon hash used in the benchmarks. Note that [Cryptoexperts](https://github.com/CryptoExperts/poseidon) only has the permutation function in C language, therefore, we constructed a sponge function in Rust for it. The Risc0 implementation has two versions, one with a field of 256 bits and another with the babybear field of 31 bits.


| Repository   | Field | Security | S-box | Full rounds | Partial rounds| Width|
| -------------| -------- | -------- |-------- |-------- |-------- |-------- |
| [Dusk-Network](https://github.com/dusk-network/Poseidon252) | BLS12-381 scalar     |  128    | $x^5$ |  8 | 59| 5|
| [Risc0](https://github.com/risc0/risc0)        | 256 bits/Babybear |  128 |  $x^8$ |  8 | 42/21 | 3/24 |
| [Neptune](https://github.com/lurk-lab/neptune)      | BLS12-381 scalar | 128 |  $x^5$ |  8 | 55 | 3 |
| [Cryptoexperts](https://github.com/CryptoExperts/poseidon)| 256 bits | 128 |  $x^3$ |  8 | 83 | 3 |

# Results

All the results presented here were executed over an Intel Xeon CPU of 2.40GHz. See [here](https://mdvillagra.github.io/poseidon-benchmarks/) for a list of detailed results.

* [Default values from corresponding repositories](https://mdvillagra.github.io/poseidon-benchmarks/criterion/Poseidon-all/report/index.html).
  
  The inputs in this case were randomly generated using the default instantiations of the repositories. The table below shows the number of bits required by the field elements. Recall that each element from the scalar field of BLS12-381 uses 255 bits, cryptoexperts uses four limbs of unsigned 64 bits integers, and each element of Babybear uses 31 bits.
  
| Number of elements | BLS12-381 | 256 bits | Babybear |
| -------------------- | ------------------ | --------- | ----------- | 
| 4                                 | 1020      | 1024        | 124      |
| 8                                 | 2040      | 2048        | 248      |
| 12                                | 3060      | 3072        | 372      |
| 16                                | 1530      | 4096        | 496      |
| 20                               | 5100      | 5120        | 620      |
| 24                               | 6120      | 6144        | 744      |
| 28                               | 7140      | 7168        | 868      |
