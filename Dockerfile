FROM rust:1

WORKDIR /src

# Pre-build all dependencies
RUN USER=root cargo init --bin --name bender
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/bender*

# Build the rest of the project
COPY . .
RUN cargo build --release --target-dir /out
RUN cargo test --release --target-dir /out

FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies

COPY --from=0 /out/deps/bender /app/bender
ADD ./quotes.json /app/quotes.json

WORKDIR /app
CMD [ "/app/bender" ]