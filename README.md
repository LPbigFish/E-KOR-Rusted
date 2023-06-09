# E-KOR Rusted

This Project is just me playing with the Blockchain technology and Cryptography. Hopefully I will release a working version someday.


## Acknowledgements

 - [The Bitcoin Math (Youtube)](https://www.youtube.com/@thebitcoinmathfgeiger8624)
 - [Learn me a Bitcoin](https://learnmeabitcoin.com/)

## Main Points

 - **Proof-of-Work** - I say that PoW is the only way how you can trust cryptocurrencies in general. It's the proof of the technology being backed up by the power it consumes during verification. The block time between blocks will be set to 40 seconds and the Block Reward will be derived from amount of Czech Crowns (CZK) "printed" in year 2004, which was +/- 48,000,000 coins, total +/- 223,000,000 CZK. So block reward will be set to 283 EKORs/block and will increase and decrease based on the Network usage.
 - **Proof-of-Proof** - I tried to figure out a concept of proving the work done by miners, that they really tried every combination from 0 to **n**, so every non-valid block gets additionally hashed with MD5, keeping 1 byte of each hash, then hashing the final byte array to form a **Proof-Hash**.
 - **Validation** - Validators will need to proof, that the miners work was documented correctly using the **Proof-Hash**. If the Validator's **Proof-Hash** is invalid compered to the miners **Proof-Hash**, the miners block will be declined and marked as invalid.
 - **Rust** - Rust being pretty new and low level makes it a great candidate for this Project. Supporting many features and being blazingly fast, sounds good to me. And featuring many libraries, with which I can implement an interface for the Nodes.
 - **Sha3 and Keccak** - Once I was on a convention about quantum computers and hearing the power it has made me paranoid about Sha2, so I chose to use Keccak and Sha3 for the Project, because they are believed to be post-quant asymmetric algorithms.
