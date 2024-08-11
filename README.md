# learn-rust-in-a-month-of-lunches

Examples for [Learn Rust in a Month of Lunches](https://www.manning.com/books/learn-rust-in-a-month-of-lunches)

```bash
# Build all examples.
cargo build --examples
cargo build --examples -v

# Build a single example.
cargo build --example ch-01-hello

# Run a single example.
cargo run --example ch-01-hello
```

```bash
# Check for formatting issues.
cargo fmt --all -- --check

# Check for common errors.
cargo clippy -- -Dwarnings
cargo clippy -- -Wclippy::pedantic
cargo clippy -- -Wclippy::restriction
```
