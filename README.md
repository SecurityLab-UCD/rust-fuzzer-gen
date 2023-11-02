# RustFuzzerGen

Convert libFuzzer files to Unit Tests for Rust.

## Usage

**NOTE**
The process is subject to change as the project matures.

### Getting Repo

```bash
mkdir -p data && cd data
get clone https://github.com/marshallpierce/rust-base64
cd ..
```

### Transforming Fuzzers

```bash
cargo run data/rust-base64
```

### Fuzzing with data recorded

```bash
cd data/rust-base64
cargo fuzz run roundtrip > record.txt
head record.txt
```