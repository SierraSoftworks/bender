FROM rust:1

ADD ./ /src
WORKDIR /src

RUN cargo test
RUN cargo build --release --target-dir /out

FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies

COPY --from=0 /out/* /app/
ADD ./quotes.json /app/quotes.json

WORKDIR /app
CMD [ "/app/bender" ]