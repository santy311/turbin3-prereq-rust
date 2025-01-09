# Turbin3 Q1 Prerequisite - Rust

This repository contains the prerequisite task.

### 1. Create a new Keypair

Generates a keypair to be used for the task

```
$cargo test keygen -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running unittests src/lib.rs (target/debug/deps/rust-17e0037ad7a88a8d)

running 1 test
You've generated a new Solana wallet: HAGrivFdpWhe6FPSFyoHt76WkUHVuMARrZWxe1ccPNM9

To save your wallet, copy and paste the following into a JSON file:
[REDACTED]
```

### 2. Claim Token Airdrop

Request 2 SOL tokens on the Solana devnet for testing.

```
$cargo test airdrop -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.16s
     Running unittests src/lib.rs (target/debug/deps/rust-17e0037ad7a88a8d)

running 1 test
Success! Check out your TX here:
https://explorer.solana.com/tx/29p1CspzzQRdoFkTGeHos6aQ74u6gvedwgzcFg3zVJfdnXEZ8aSE8ndwuEZhPYDCBGap7wcY5kK7M8LjtShKCCNi?cluster=devnet

```

[explorer link](https://explorer.solana.com/tx/29p1CspzzQRdoFkTGeHos6aQ74u6gvedwgzcFg3zVJfdnXEZ8aSE8ndwuEZhPYDCBGap7wcY5kK7M8LjtShKCCNi?cluster=devnet)

### 3. Transfer tokens to your Turbin3 Address

Transfer 0.1 Sol (100_000_000) from created account to Turbin3 Wallet

```
$cargo test transfer_sol -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.99s
     Running unittests src/lib.rs (target/debug/deps/rust-17e0037ad7a88a8d)

running 1 test
Success! Check out your TX here:
https://explorer.solana.com/tx/5T2HuVXppcCECfbK4GJUAWkwPoeERJQfUSRRWJ8R6pid9mMbZDRiMPo6EWD82LTNLpW4fn33QAsr9g4YRAgRQVni?cluster=devnet
```

[explorer link](https://explorer.solana.com/tx/5T2HuVXppcCECfbK4GJUAWkwPoeERJQfUSRRWJ8R6pid9mMbZDRiMPo6EWD82LTNLpW4fn33QAsr9g4YRAgRQVni?cluster=devnet)

### 4. Empty devnet wallet into Turbin3 wallet

Drain the SOL from created wallet to Turbin3 Wallet

```
$cargo test empty_wallet -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.04s
     Running unittests src/lib.rs (target/debug/deps/rust-17e0037ad7a88a8d)

running 1 test
Fee: 5000
Success! Check out your TX here:
https://explorer.solana.com/tx/2hSSYNerwJ2CDcvbDhACHfawkNXqiizjswmXAKW4GCNKNcS58Apq8MQ1wzKo9BK78KQQqpFGfTr9DD6FzpVNDGNu?cluster=devnet
```

[explorer link](https://explorer.solana.com/tx/2hSSYNerwJ2CDcvbDhACHfawkNXqiizjswmXAKW4GCNKNcS58Apq8MQ1wzKo9BK78KQQqpFGfTr9DD6FzpVNDGNu?cluster=devnet)

### 5. Submit your completion of the Turbin3 pre-requisites program

Use the IDL and create a transaction to enrol which will create a PDA for you.

```
$cargo test complete_prereq -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.11s
     Running unittests src/lib.rs (target/debug/deps/rust-17e0037ad7a88a8d)

running 1 test
Success! Check out your TX here:
https://explorer.solana.com/tx/2jozQLNe8cVNsEF5CkcN2mMkymzg7786gK7NeYXGAjnytbpxT5ssyygCwkKpY2ZyRxpdCH6MstcET4Cq562JHVRt?cluster=devnet
```

[explorer link](https://explorer.solana.com/tx/2jozQLNe8cVNsEF5CkcN2mMkymzg7786gK7NeYXGAjnytbpxT5ssyygCwkKpY2ZyRxpdCH6MstcET4Cq562JHVRt?cluster=devnet)
