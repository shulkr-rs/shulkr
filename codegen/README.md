# codegen

A simple code generator for [shulkr](https://crates.io/crates/shulkr).

It turns [shulkr-data](../crates/shulkr-data)'s bundled Minecraft assets into generated Rust source under [../crates/shulkr/generated/](../crates/shulkr/generated/).

## Usage

Run from the workspace root:
```
cargo run -p codegen
```

Output is only rewritten when its content actually changed, so re-running
after a no-op doesn't create noisy diffs. Changed files under
`crates/shulkr/generated/` need to be committed after running.

To add a new datapack registry, add its folder name to
`DATAPACK_REGISTRIES` in `src/main.rs`.
