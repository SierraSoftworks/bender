FROM clux/muslrust:stable as builder

RUN apt-get update && apt-get install -y openssl unzip protobuf-compiler gcc

WORKDIR /src

# Pre-build all dependencies
RUN USER=root cargo init --bin --name bender
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release --locked && rm -rf target/*/release/deps/bender*
RUN rm src/*.rs

# Add the source code
COPY . .

# Run the test suite
RUN cargo test --release && rm -rf target/*/release/deps/bender*

# Build the final executable of the project
RUN cargo build --release --bin bender --locked

# Ensure that the binary is at a known location for the next stage
RUN mkdir /out && \
    rm /src/target/*/release/deps/bender*.d && \
    cp /src/target/*/release/deps/bender* /out/bender

FROM cgr.dev/chainguard/static:latest

COPY --from=builder /out/bender /app/bender
ADD ./quotes.json /app/quotes.json

WORKDIR /app
ENTRYPOINT [ "/app/bender" ]