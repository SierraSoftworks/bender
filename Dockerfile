# NOTE: This Dockerfile depends on you building the bender binary first.
# It will then package that binary into the image, and use that as the entrypoint.
# This means that running `docker build` is not a repeatable way to build the same
# image, but the benefit is much faster cross-platform builds; a net win.
#
# The binary is expected to be a statically linked (musl) executable placed at
# ./bender in the build context (see .github/workflows/release.yml).
FROM cgr.dev/chainguard/static:latest

LABEL org.opencontainers.image.source=https://github.com/SierraSoftworks/bender
LABEL org.opencontainers.image.description="Your unfriendly source of Futurama quotes"

COPY ./bender /app/bender
COPY ./quotes.json /app/quotes.json

WORKDIR /app
ENTRYPOINT [ "/app/bender" ]
