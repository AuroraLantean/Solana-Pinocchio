# Solana-Pinocchio

## Instroduction to Pinocchio

Pinocchio is a no external dependencies library to make Solana programs in Rust, like Anchor but without auto IDL generation, nor using solana-program. We manually use Shank to generate IDL, SolanaProgram to deploy it to a local network, then run TypeScript tests with IDL.

The only dependencies are types from the Solana SDK. This mitigates dependency issues and offers an efficient zero-copy library to write programs, optimized in terms of both compute units consumption and binary size.

### Why Pinocchio

- Zero-dependency. Additional dependencies can be added on
- High Performance & Lower Compute Units
- Zero-copy types to avoid cloning or copying in deserialization
- Provide safe/unsafe function variants: safe Rust do all checks for you, while{} unsafe Rust comes with speed
- Small Binaries: no no_std the Rust standard library

### Environment

Rust: 1.92.0 (ded5c06cf 2025-12-08);
solana-cli: 3.0.12 or 2.3.13;
BunJs:  1.3.3; PNPM: 10.25.0

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

### Build and Deploy the Program locally

```bash
cargo build-sbf
solana program deploy --program-id target/deploy/pinocchio_vault-keypair.json target/deploy/pinocchio_vault.so --url localhost 
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

### LiteSVM

Fast and lightweight library for testing Solana programs.

```bash
pnpm add -D litesvm @solana/web3.js @solana/spl-token
```

### Reference

- Pinocchio: <https://github.com/anza-xyz/pinocchio>
- Bun Js Test: <https://bun.com/docs/test>
- Solana Kit: <https://www.solanakit.com/>
- LiteSVM: <https://github.com/LiteSVM/litesvm>
- Gill: <https://www.gillsdk.com/docs/guides/tokens/create-token>
- Quicknode: <https://www.quicknode.com/guides/solana-development/pinocchio/how-to-build-and-deploy-a-solana-program-using-pinocchio>
- Create Token Mint via Rust and JavaScript: <https://solana.com/docs/tokens/basics/create-mint>
- Create and Mint SPL Tokens via Anchor: <https://www.quicknode.com/guides/solana-development/anchor/create-tokens>
- Create Token Account via Anchor: <https://www.anchor-lang.com/docs/tokens/basics/create-token-account>
- Get Token Account: <https://solana.com/developers/cookbook/tokens/get-token-account>