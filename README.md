# rust-fuzzer-gen
Convert libFuzzer `fuzz_target!` files to Unit Tests `#[test]` for Rust.

## Setup

- Rust nightly
  ```bash
  rustup install nightly
  rustup default nightly
  ```

- cargo-fuzz
  ```bash
  cargo install cargo-fuzz
  ```


## Usage

**NOTE**
The process is subject to change as the project matures.


### Getting Repo

```bash
mkdir -p data && cd data
git clone https://github.com/marshallpierce/rust-base64
cd ..
```

### Transforming Fuzzers

```bash
cargo run data/rust-base64
```

This step does two things `data/rust-base64`

1. modify fuzzer files in `\fuzz` and copy a backup `\fuzz-bak`
2. generate unit tests `\tests-gen` based on `\fuzz`

To restore to previous stage, use `rm fuzz tests-gen -rf && mv fuzz-bak fuzz`.

### Fuzzing with data recorded

```bash
cd data/rust-base64
cargo fuzz run roundtrip > record.txt
head record.txt
```

### Using as a package

```bash
cargo install --git https://github.com/SecurityLab-UCD/rust-fuzzer-gen.git
rust-fuzzer-gen <patht to a target repo>
```
