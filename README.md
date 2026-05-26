<img width="1920" height="932" alt="image" src="https://github.com/user-attachments/assets/a67d1f4f-b8a4-4e36-b9e8-e63e980ff2d8" />


# EarthForward
A simple fundraising app where charities launch environmental projects and donors fund them directly on Stellar.

## Problem and Solution
* **Problem:** Local charity leaders in Southeast Asia cannot get money from global donors because donors do not trust how the money is spent, which causes local environmental projects to run out of funds and stop.
* **Solution:** EarthForward lets these charities set up public fundraising pages where global donors can send digital dollars (USDC) safely, using Stellar smart contracts to hold and track the money openly on the network.

## Timeline
* **Day 1–3: Smart Contract Setup** Write the core contract logic (`lib.rs`) allowing initialization, donation tracking, and creator-only fund withdrawals.
* **Day 4–5: Local Testing** Implement the 5-part test framework suite covering standard usage flows, edge case boundary limits, and strict permission models to guarantee fund safety.
* **Day 6–8: Web Application Development** Build a clean front-end interface using React/Next.js featuring real-time fundraising progress trackers and simplified wallet injection components.
* **Day 9–10: Network Deployment & Integration** Deploy the compiled contract to the Stellar Testnet, link the contract with the user interface, and execute the final end-to-end demo walkthrough.

## Stellar Features Used
* **USDC Stablecoin Integration:** Acts as the primary funding asset, allowing real-world fiat value to move transparently across borders in seconds for less than a penny without volatile price shifts.
* **Soroban Smart Contracts:** Houses the public, tamper-proof business logic that isolates collected donations inside a decentralized vault, keeping it fully independent from third-party custody.
* **On-Chain Event Tracking:** Leverages instance storage lookups to feed the front-end dashboard accurate project updates directly from the network ledger.

## Vision and Purpose
EarthForward is designed to replace blind trust with open, verifiable cryptographic proof. By eliminating traditional processing intermediaries and administrative gray zones, our platform aims to empower local grassroots climate leaders throughout Southeast Asia with instant access to global capital channels, ensuring every digital dollar directly supports active field recovery initiatives.

## Prerequisites
* **Rust Toolchain:** Version 1.75 or higher
* **Target Architecture:** `wasm32-unknown-unknown` installed via `rustup target add`
* **Soroban CLI:** Version 21.0.0 or higher

## How to build
Compile the smart contract to optimization-ready WebAssembly (Wasm) bytecode:
```bash
soroban contract build

## Build the Contract

```bash
soroban contract build
```

## Run Tests
```bash
cargo test
```

## Deploy to Stellar Testnet
```bash
soroban contract deploy \
  --network testnet \
  --source alice \
  --wasm target/wasm32-unknown-unknown/release/earth_forward.wasm
```

## Sample Contract Invocation
```bash
soroban contract invoke \
  --id CBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB \
  --network testnet \
  --source bob \
  -- \
  donate \
  --donor GDBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB \
  --amount 500
```

## Project Structure
```text
earthforward/
├── contracts/
├── src/
├── tests/
├── Cargo.toml
└── README.md
```

## Contract ID
CDEOS3UTVZHP3SJY3257QFJJLTPBZXPXUUHISLGEVUWK3NMQ7TELAWI5

## Stellar Link
https://stellar.expert/explorer/testnet/contract/CDEOS3UTVZHP3SJY3257QFJJLTPBZXPXUUHISLGEVUWK3NMQ7TELAWI5

## License
This project is open-source software licensed under the MIT License.


