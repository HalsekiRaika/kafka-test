FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    openssl \
    libssl-dev \
    pkg-config \
    valgrind \
    zlib1g-dev \
    cmake

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable


ENV PATH /root/.cargo/bin/:$PATH

RUN mkdir kafka-test

WORKDIR kafka-test

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release

ENTRYPOINT ["/bin/bash"]