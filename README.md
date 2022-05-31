# expert-spoon
[Project board](https://github.com/orgs/Flavor-of-the-Code/projects/3)

## Tooling

### [rustfmt](https://rust-lang.github.io/rustfmt/?version=v1.4.38&search=)
```bash
cargo fmt
```

### [rustfix](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html#fix-your-code-with-rustfix)
```bash
cargo fix
```

### [clippy](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html#more-lints-with-clippy)
```bash
cargo clippy
```

## Production

```bash
cargo build --release
./target/release/expert-spoon
```

## Development

```bash
cargo build
./target/debug/expert-spoon
```

### Build & run

```bash
cargo run
```
### Check For Errors Without Building

```bash
cargo check
```

## [Tests](https://doc.rust-lang.org/cargo/guide/tests.html)

> Cargo looks for tests to run in two places: in each of your src files and any tests in tests/. Tests in your src files should be unit tests, and tests in tests/ should be integration-style tests. 

[Testing guide](https://doc.rust-lang.org/book/ch11-00-testing.html)

```bash
cargo test
```


