FROM rust:1

WORKDIR /src

# Pre-build all dependencies
RUN USER=root cargo init --bin --name bender
COPY ./Cargo.lock .
COPY ./Cargo.toml .
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/bender*

# Add the source code
COPY . .

# Run the test suite
RUN cargo test --release
RUN rm /src/target/release/deps/bender*


# Build the rest of the project
RUN cargo build --release --bin bender

# Ensure that the binary is at a known location for the next stage
RUN rm /src/target/release/deps/bender*.d
RUN cp /src/target/release/deps/bender* /src/target/release/deps/bender

FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies

RUN apt-get update && apt-get install -y libssl1.1

COPY --from=0 /src/target/release/deps/bender /app/bender
ADD ./quotes.json /app/quotes.json

WORKDIR /app
CMD [ "/app/bender" ]