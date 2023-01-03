FROM clux/muslrust:stable as builder

ARG PROTOC_VERSION=3.20.2

RUN apt-get update && apt-get install -y openssl unzip

# Install protoc
RUN curl -sSL -o /tmp/protoc.zip "https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip" && unzip /tmp/protoc.zip -d /usr/local && rm /tmp/protoc.zip

WORKDIR /src

# Pre-build all dependencies
RUN USER=root cargo init --bin --name bender
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release --locked && rm -rf target/x86_64-unknown-linux-musl/release/deps/bender*
RUN rm src/*.rs

# Add the source code
COPY . .

# Run the test suite
RUN cargo test --release && rm -rf target/x86_64-unknown-linux-musl/release/deps/bender*

# Build the final executable of the project
RUN cargo build --release --bin bender --locked

# Ensure that the binary is at a known location for the next stage
RUN mkdir /out && \
    rm /src/target/x86_64-unknown-linux-musl/release/deps/bender*.d && \
    cp /src/target/x86_64-unknown-linux-musl/release/deps/bender* /out/bender

FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder /out/bender /app/bender

WORKDIR /app
CMD [ "/app/bender" ]