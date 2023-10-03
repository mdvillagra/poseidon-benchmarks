# poseidon-benchmarks
This is a repository comparing different Poseidon implementations in Rust.

Repositories that are being used in this project are:
1. https://github.com/dusk-network/Poseidon252
2. https://github.com/CryptoExperts/poseidon
3. https://github.com/lambdaclass/lambdaworks
4. https://github.com/risc0/risc0 
5. https://github.com/lurk-lab/neptune

# Poseidon parameters used in benchmarks

Here we present the default parameters of the different instantiations of Poseidon hash used in the benchmarks. Note that [Cryptoexperts](https://github.com/CryptoExperts/poseidon) only has the permutation function in C language, therefore, we constructed a sponge function in Rust for it.


| Repository   | Field | Security | S-box | Full rounds | Partial rounds| Width|
| -------------| -------- | -------- |-------- |-------- |-------- |-------- |
| [Dusk-Network](https://github.com/dusk-network/Poseidon252) | BLS12-381 scalar     |  128    | $x^5$ |  8 | 59| 5|
| [Risc0](https://github.com/risc0/risc0)        | Babybear |  128 |  $x^8$ |  8 | 42 | 3 |
| [Neptune](https://github.com/lurk-lab/neptune)      | BLS12-381 scalar | 128 |  $x^5$ |  8 | 55 | 3 |
| [Lambdaworks](https://github.com/lambdaclass/lambdaworks)  |  BLS12-381 scalar | 128 |  $x^5$ |  8 | 56 | 3 |
| [Cryptoexperts](https://github.com/CryptoExperts/poseidon)| $2^{256}-1$ | 128 |  $x^3$ |  8 | 83 | 3 |
