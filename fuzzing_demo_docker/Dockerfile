# Start with Ubuntu base image
FROM ubuntu:latest

# Set environment variable for non-interactive apt-get
ENV DEBIAN_FRONTEND=noninteractive

# Update and install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    clang \
    libclang-dev \
    pkg-config \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set environment variables for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Cargo tools: ziggy, cargo-afl, and hfuzz
RUN cargo install ziggy \
    && cargo install cargo-afl \
    && cargo install honggfuzz \
    && cargo install grcov

# Clone the specified GitHub repository
RUN git clone https://github.com/rachsrl/fuzzing-polkadot-sdk.git -b fuzzing_demo

# Set working directory
WORKDIR /fuzzing-polkadot-sdk

# Default command
CMD ["/bin/bash"]
