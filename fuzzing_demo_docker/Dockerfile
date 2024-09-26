# docker build -t ziggy .
# docker run -it -v /path/to/fuzzing-polkadot-sdk:/target ziggy
# > cd target/templates/solochain/runtime/fuzz
# > cargo ziggy fuzz -j 4 --no-honggfuzz -G 32

FROM rust:1.81.0-bookworm

RUN cargo install ziggy cargo-afl

ENV SKIP_WASM_BUILD=1
ENV AFL_SKIP_CPUFREQ=1
