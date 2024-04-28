# learn-rust-in-a-month-of-lunches

Examples for [Learn Rust in a Month of Lunches](https://www.manning.com/books/learn-rust-in-a-month-of-lunches)

```bash
# Build all examples.
cargo build --examples

# Build a single example.
cargo build --example ch-01-hello

# Run a single example.
cargo run --example ch-01-hello
```

```bash
# Check for formatting issues.
cargo fmt --all -- --check

# Check for common errors.
cargo clippy -- -Wclippy::pedantic -Wrustdoc::private_intra_doc_links -Wrustdoc::broken-intra-doc-links -Aclippy::single-match-else -Aclippy::default-trait-access -Aclippy::missing-panics-doc -Wclippy::missing-errors-doc
```