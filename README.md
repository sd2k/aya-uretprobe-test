# aya-uretprobe-test

Repo containing a reproducible example of a problem I'm having with aya and uretprobes, probably just stemming from my lack of understanding.

Specifically, I can't tell how to decide how struct args and return values should be correctly retrieved. It seems like the fields of the struct are passed/returned as separate args, is this something to do with the calling convention?

## Prerequisites

1. Install a rust stable toolchain: `rustup install stable`
1. Install a rust nightly toolchain: `rustup install nightly`
1. Install bpf-linker: `cargo install bpf-linker`

## Run probe

```bash
RUST_LOG=info cargo xtask run
```

## Run probed executable

```bash
cargo run --bin targettest
```
