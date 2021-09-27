# Solana Payment Splitter
## Purpose
Composable payment splitter. Splits every SOL payment into half. Need to split into 4? Create 2 node splitter and 1 root splitter.
## Coding
### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana v1.6.2 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```
