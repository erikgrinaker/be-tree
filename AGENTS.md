# AGENTS.md

## Project Overview

This will be a Rust implementation of an embedded 
[Bε-tree](https://www.usenix.org/system/files/login/articles/login_oct15_05_bender.pdf) storage
engine.

It's intended to form the foundation of a distributed database based on the
[Accord](https://cwiki.apache.org/confluence/download/attachments/188744725/Accord.pdf) consensus
algorithm, and needs to integrate well with it. But it will also be distributed as an open source
library for general public use.

## Working Agreements

- Document internal invariants.
- Write concise and clear doc comments for all data structures and APIs, even internal ones.

## Verification

Run the standard Rust checks after substantive changes to ensure basic correctness:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## When Editing

- Favor incremental steps that keep the crate compiling.
- Leave concise comments only where the storage semantics or invariants would otherwise be unclear.
