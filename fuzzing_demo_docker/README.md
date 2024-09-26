## Steps to Fuzz using Docker 

1. Clone the repo
```bash
# git clone https://github.com/rachsrl/fuzzing-polkadot-sdk.git
```

2. Build the docker image
```bash
# cd fuzzing-polkadot-sdk/fuzzing_demo_docker
# docker build -t ziggy .
```

3. Run the docker image. Provide absolute path to the `fuzzing-polkadot-sdk` repo
```bash
# docker run -it -v /path/to/fuzzing-polkadot-sdk:/target ziggy
```

4. Fuzz the solochain-template
```bash
# cd target/templates/solochain/runtime/fuzz
# cargo ziggy fuzz -j 4 --no-honggfuzz -G 32
``` 

## Steps to fuzz without Docker

1. Install Rust
```bash
# curl --proto '=https’ --tlsv1.2 -sSf https://sh.rustup.rs | sh
# rustup install nightly
# rustup default nightly
# rustup target add wasm32-unknown-unknown
```

2. Install fuzzing tools (ziggy, AFL++, Honggfuzz and grcov)
```bash
# cargo install ziggy cargo-afl honggfuzz grcov
```

3. Clone the repo
```bash
# git clone https://github.com/rachsrl/fuzzing-polkadot-sdk.git 
```

4. Fuzz the solochain-template
```bash
# cd fuzzing-polkadot-sdk/templates/solochain/runtime/fuzz
# cargo-afl afl system-config
# cargo ziggy fuzz -j 4 --no-honggfuzz -G 32
```

## Analyzing the crash

```bash
cargo ziggy run -i output/solochain-template-fuzzer/crashes/<directory_name>
```
