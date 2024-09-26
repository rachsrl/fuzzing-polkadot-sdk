## Fuzzing Steps using Docker 

1. Get the Dockerfile

2. Build the docker image
```bash
docker build -t fuzzing-demo .
```

3. Run the docker image in an interactive shell
```bash
docker run -it fuzzing-demo
```

You should now be in `fuzzing-polkadot-sdk` directory

4. Fuzz the solochain-template
```bash
cd /templates/solochain/runtime/fuzz
SKIP_WASM_BUILD=1 cargo ziggy fuzz -j 4 --no-honggfuzz
``` 

## Fuzzing steps without using Docker

1. Install Rust
```bash
curl --proto '=https’ --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
```

2. Install fuzzing tools (ziggy, AFL++, Honggfuzz and grcov)
```bash
cargo install ziggy 
cargo-afl honggfuzz grcov
```

3. Clone the repo
```bash
git clone https://github.com/rachsrl/fuzzing-polkadot-sdk.git -b fuzzing_demo 
```

4. Fuzz the solochain-template
```bash
cd fuzzing-polkadot-sdk/templates/solochain/runtime/fuzz
cargo-afl afl system-config
SKIP_WASM_BUILD=1 cargo ziggy fuzz -j 4 --no-honggfuzz
```

## Analysing the crash

```bash
cargo ziggy run -i output/solochain-template-fuzzer/crashes/<directory_name>
```
