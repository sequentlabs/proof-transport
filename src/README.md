# Proof-Transport
[![Build](https://img.shields.io/github/actions/workflow/status/sequentlabs/proof-transport/ci.yml?branch=main)](https://github.com/sequentlabs/proof-transport/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Live Demo](https://img.shields.io/badge/demo-online-green)](https://sequentlabs.github.io/proof-transport/)

**One-liner:** keep proofs valid when the proof kernel changes. Transport sequent proofs across rule-set updates (e.g., remove `Cut`) and emit stability certificates.

## Quickstart
```bash
cargo build --release
./target/release/tesl version
