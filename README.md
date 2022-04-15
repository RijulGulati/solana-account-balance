# Solana Account Balance crate

[![crates-io](https://img.shields.io/badge/crates.io-v0.1.0-blue)](https://crates.io/crates/solana-account-balance)

solana-account-balance is a simple library crate to fetch Account Balance from Solana Clusters.
The library supports fetching account balance from following Clusters:

- Mainnet Beta
- Testnet
- Devnet

This library uses [`solana-sdk`](https://crates.io/crates/solana-sdk) and [`solana-client`](https://crates.io/crates/solana-client) crates as dependencies to connect with solana cluster and get account balance.

## Build

**Build library**

```bash
$ cargo build
```

**Running Tests**

```bash
$ cargo test
```

## Usage

- Include `solana-account-balance` as dependency in Cargo.toml file.

```toml
[dependencies]
solana-account-balance = "0.1.0"
```

- Example code

```rust
use solana_account_balance::Cluster;

fn main() {
    let pubkey = "9aavjzd4iAbiJHawgS7kunfCJefSRRVKso61vzAX9Ho5";
    let balance = solana_account_balance::get_solana_balance(pubkey, Cluster::Devnet.unwrap();
    println!("{:?}", balance);
}
```

# License

MIT License

Copyright (c) 2022 Rijul Gulati

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
