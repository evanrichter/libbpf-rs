# Build Stage
FROM silkeh/clang:14 as builder

USER root

## Add dependencies
RUN apt update && apt install -y libelf-dev zlib1g-dev pkg-config curl && \
    ln -s /usr/local/bin/clang /usr/local/bin/cc && \
    ln -s /usr/local/bin/clang++ /usr/local/bin/c++

## Install rust nightly and cargo fuzz
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --profile minimal -y
RUN ${HOME}/.cargo/bin/cargo install -Z sparse-registry -f cargo-fuzz

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN export RUSTFLAGS="-Clinker=clang" CC="clang" CFLAGS="-g -fsanitize=address -fsanitize-coverage=inline-8bit-counters,trace-cmp,edge,pc-table" && \
    bash -c "pushd libbpf-rs/fuzz && ${HOME}/.cargo/bin/cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv libbpf-rs/fuzz/target/x86_64-unknown-linux-gnu/release/open_object /open_object

RUN export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" CFLAGS="-g" && \
    bash -c "pushd libbpf-rs/fuzz && ${HOME}/.cargo/bin/cargo +nightly -Z sparse-registry build --release && popd" && \
    mv libbpf-rs/fuzz/target/release/open_object /open_object_no_inst

# Package Stage
FROM silkeh/clang:14

USER root

## Add dependencies
RUN apt update && apt install -y libelf-dev zlib1g-dev

COPY --from=builder /open_object /open_object_no_inst /
RUN mkdir /testsuite && cp /bin/ls /testsuite
