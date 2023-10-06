# poseidon-benchmarks
This is a repository comparing different Poseidon implementations in Rust.

Repositories that are being used in this project are:
1. https://github.com/dusk-network/Poseidon252
2. https://github.com/CryptoExperts/poseidon
3. https://github.com/risc0/risc0 
4. https://github.com/lurk-lab/neptune

# Default Poseidon parameters

Here we present the default parameters of the different instantiations of Poseidon hash used in the benchmarks. Note that [Cryptoexperts](https://github.com/CryptoExperts/poseidon) only has the permutation function in C language, therefore, we constructed a sponge function in Rust for it.


| Repository   | Field | Security | S-box | Full rounds | Partial rounds| Width|
| -------------| -------- | -------- |-------- |-------- |-------- |-------- |
| [Dusk-Network](https://github.com/dusk-network/Poseidon252) | BLS12-381 scalar     |  128    | $x^5$ |  8 | 59| 5|
| [Risc0](https://github.com/risc0/risc0)        | Babybear |  128 |  $x^8$ |  8 | 42 | 3 |
| [Neptune](https://github.com/lurk-lab/neptune)      | BLS12-381 scalar | 128 |  $x^5$ |  8 | 55 | 3 |
| [Cryptoexperts](https://github.com/CryptoExperts/poseidon)| $2^{256}-1$ | 128 |  $x^3$ |  8 | 83 | 3 |

# Results
* [Default values from corresponding repositories](https://mdvillagra.github.io/poseidon-benchmarks/Poseidon-Xeon/report/index.html).
  
  The inputs in this case were randomly generated using the default instantiations of the repositories. The table below shows the number of field elements used for each input and its corresponding number of bits. Recall that each element from the scalar field of BLS12-381 uses 255 bits, cryptoexperts uses four limbs of unsigned 64 bits integers, and each element of Babybear uses 31 bits.
  
  | Input | Number of elements | BLS12-381 | $2^{256}-1$ | Babybear |
  | ----- | ------------------ | --------- | ----------- | -------- |
  | 0     | 4                  | 1020      | 1024        | 124      |
  | 1     | 8                  | 2040      | 2048        | 248      |
  | 2     | 12                 | 3060      | 3072        | 372      |
  | 3     | 16                 | 1530      | 4096        | 496      |
  | 4     | 20                 | 5100      | 5120        | 620      |
  | 5     | 24                 | 6120      | 6144        | 744      |
  | 6     | 28                 | 7140      | 7168        | 868      |
  
* [Cryptoexperts vs Risc0](https://mdvillagra.github.io/poseidon-benchmarks/Poseidon-cryptoexperts-vs-risc0/report/index.html).
