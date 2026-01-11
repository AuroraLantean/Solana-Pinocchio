# Solana-Pinocchio

## Instroduction to Pinocchio

Pinocchio is a no external dependencies library to make Solana programs in Rust, like Anchor but without auto IDL generation, nor using solana-program. We manually use Shank to generate IDL, SolanaProgram to deploy it to a local network, then run TypeScript tests with IDL.

The only dependencies are types from the Solana SDK. This mitigates dependency issues and offers an efficient zero-copy library to write programs, optimized in terms of both compute units consumption and binary size.

### Why Pinocchio

- Zero-dependency. Additional dependencies can be added on
- High Performance Execution & Lower Compute Units
- Zero-copy types to avoid cloning or copying in deserialization
- Provide safe/unsafe function variants: safe Rust do all checks for you, while{} unsafe Rust comes with speed
- Small Binaries: no no_std the Rust standard library
- Fine-grained control over account fields,

The key difference is in the implementation of AccountInfo. While solana-program writes data to an AccountInfo struct that owns the data, Pinocchio’s AccountInfo struct is itself just a pointer to the underlying input data that represents the account. This reduces the amount of data needed to be copied, saving a lot of CUs.

### Why skipping solana-program

the solana-program crate brings overhead. extra deserialization, hidden allocations, and increased binary size.

### Why LiteSVM

LiteSVM is a lightweight library for testing Solana Programs. Unlike other testing approaches that spin up separate validator processes, LiteSVM embeds the VM inside your tests, making them execute much faster.

- Everything is Synchronous
- Direct State Manipulation
- Time Manipulation
- Errors are Immediate and Clear
- Better than Mollusk testing environment because LiteSVM runs a full, lightweight Solana Virtual Machine (SVM) for broader testing (Rust, TS for testing frontend libraries aka End-To-End testing, Python), offering greater realism and speed for complex scenarios. Additionally, LiteSVM offers Time Travel testing.

```rust
cargo add --dev litesvm litesvm-token solana-sdk
```

### Environment

Rust: 1.92.0 (ded5c06cf 2025-12-08);
solana-cli: 3.0.12 or 2.3.13;
BunJs:  1.3.5; PNPM: 10.27.0

Install Solana CLI: <https://solana.com/docs/intro/installation> Or <https://docs.anza.xyz/cli/install>

Install Biome: <https://biomejs.dev/guides/getting-started/>

## Setup a new Pinocchio project

```bash
cargo new program-name --lib --edition 2021
cd program-name
cargo add pinocchio pinocchio-system pinocchio-log pinocchio-pubkey shank
bun init
```

Add config in `Cargo.toml`

```toml
[lib]
crate-type = ["lib", "cdylib"]
```

### Make Program ID

```bash
solana-keygen new -o target/deploy/pinocchio_vault-keypair.json
solana address -k target/deploy/pinocchio_vault-keypair.json
```

Paste it into lib.rs > declare_id! macro.

### Build the program

```bash
cargo build-sbf
```

### Make IDL via Shank for new instruction layout

```bash
cargo install shank-cli
shank idl -o idl
```

The idl folder in our project now contains pinocchio_vault.json which is our program's IDL.

### Make a client via @solana/kit and Codama

Codama takes the Shank IDL and emits a TypeScript client. The generated code includes instruction builders, account types, and small conveniences that keep your client code focused on composing transactions.

```bash
  pnpm install
  pnpm dlx codama init
  pnpm dlx codama run js
```

You'll see a clients/js/src/generated/ folder in our project with the program types our client code uses to send transactions to our program.

### Run Tests via LiteSVM

Write tests in Rust:

```bash
cargo add --dev litesvm litesvm-token solana-sdk
```

Write tests in TypeScript
See tutorial: <https://litesvm.github.io/litesvm/tutorial.html>

```bash
pnpm add -D litesvm @solana/web3.js @solana/spl-token
bun test ./tests/litesvm1.ts
```

### Run Tests via Solana-Test-Validator

Open two terminals at this project directory:
Run in terminal 1:

```bash
solana-test-validator -r
```

Build, deploy the Program locally, and run tests in terminal 2:

```bash
cargo build-sbf
solana program deploy --program-id target/deploy/pinocchio_vault-keypair.json target/deploy/pinocchio_vault.so --url localhost
bun test ./tests/test1.ts
```

### TODO

- Update Dependencies: <https://github.com/anza-xyz/pinocchio/releases>
- Codama can generate TS and Rust client: <https://solana.com/docs/programs/codama/clients>, but not yet Go client: <https://github.com/codama-idl/codama/issues/973>
- Mint2022 layout: <https://rareskills.io/post/token-2022>
- StackExchange: <https://solana.stackexchange.com/questions/23991/how-to-generate-a-go-client-from-pinocchio-rust>
- Anchor Go: <https://github.com/gagliardetto/anchor-go>
- Anchor Go fork by Fragmetric-labs: <https://github.com/fragmetric-labs/solana-anchor-go>
- Solana Go SDK: <https://github.com/gagliardetto/solana-go>
- Developing a Native Rust program and its Rust client: <https://solana.com/docs/programs/rust>
- Rust(2 years old): <https://github.com/cenwadike/solana-program-client>

- Solana Kite to replace some SolanaKit: <https://github.com/solanakite/kite>; AND <https://solanakite.org/docs>

### References

- Pinocchio: <https://github.com/anza-xyz/pinocchio>
- Native Development: <https://solana.com/docs/programs/rust>
- Bun Js Test: <https://bun.com/docs/test>
- Solana Kit: <https://www.solanakit.com/docs/getting-started/send-transaction>
- Solana Kit Account: <https://github.com/anza-xyz/kit/tree/main/packages/accounts>
- LiteSVM Docs: <https://www.litesvm.com/docs/getting-started>
- LiteSVM GitHub: <https://github.com/LiteSVM/litesvm>
- LiteSVM Example by Quicknode: <https://github.com/quiknode-labs/you-will-build-a-solana-program>
- LiteSVM in JS: <https://litesvm.github.io/litesvm/tutorial.html>
- Gill: <https://www.gillsdk.com/docs/guides/tokens/create-token>
- Quicknode: <https://www.quicknode.com/guides/solana-development/pinocchio/how-to-build-and-deploy-a-solana-program-using-pinocchio>
- How to Build Programs with Pinocchio: <https://www.helius.dev/blog/pinocchio#how-is-pinocchio-more-performant-than-solana-program>
- Create Token Mint via Rust and JavaScript: <https://solana.com/docs/tokens/basics/create-mint>
- Create and Mint SPL Tokens via Anchor: <https://www.quicknode.com/guides/solana-development/anchor/create-tokens>
- Create Token Account via Anchor: <https://www.anchor-lang.com/docs/tokens/basics/create-token-account>
- Get Token Account: <https://solana.com/developers/cookbook/tokens/get-token-account>
- Account Deserialization by QuickNode: <https://www.quicknode.com/guides/solana-development/tooling/web3-2/account-deserialization>
- Solana Kit decoder with fixed size codec: <https://github.com/anza-xyz/kit/tree/main/packages/codecs-core#fixed-size-and-variable-size-codecs>
