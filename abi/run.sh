rustc +stage1 -C debuginfo=2 --emit llvm-ir --crate-type rlib --color=always --edition 2024 --target aarch64-apple-darwin abi.rs
