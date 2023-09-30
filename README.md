# poseidon-benchmarks
Repository comparing different Poseidon implementations in Rust.

Repositories that are being used in this project are:
1. https://github.com/dusk-network/Poseidon252
2. https://github.com/CryptoExperts/poseidon
3. https://github.com/lambdaclass/lambdaworks
4. https://github.com/risc0/risc0 
5. https://github.com/lurk-lab/neptune

# Poseidon Parameters used in Benchmarks

Here we present the parameters of the different instantiations of Poseidon hash used in the benchmarks. Note that [Cryptoexperts](https://github.com/CryptoExperts/poseidon) only has the permutation function in C language, and henche, we constructed a sponge function in Rust for it.


| Repository   | Field | Security | S-box | Full rounds | Partial rounds| Rate|
| -------------| -------- | -------- |-------- |-------- |-------- |-------- |
| [Dusk-Network](https://github.com/dusk-network/Poseidon252) | Text     | Text     |
| [Risc0](https://github.com/risc0/risc0)        |
| [Neptune](https://github.com/lurk-lab/neptune)      |
| [Lambdaworks](https://github.com/lambdaclass/lambdaworks)  |
| [Cryptoexperts](https://github.com/CryptoExperts/poseidon)|
