# Fuzz oxc parser

## Using `shift`

```bash
pnpm install
pnpm run start
```

## Using `cargo fuzz`

```bash
cargo install cargo-fuzz
```
### Run

Run fuzzer for the parser, for 15 minutes.

```bash
cd fuzz
rustup default nightly
cargo +nightly fuzz run --sanitizer none parser -- -only_ascii=1 -max_total_time=900 -timeout=5
```
