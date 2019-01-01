# Use a mutli-stage build pipeline to generate the executable
FROM golang:1.11-alpine

RUN apk add --no-cache build-base git

ARG VERSION="development"

ADD . /src
WORKDIR /src

RUN ["go", "test", "-v", "./..."]

ENV CGO_ENABLED=0
ENV GOOS=linux
RUN ["go", "build", "-o", "bin/bender", "-a", "-installsuffix", "cgo", "-ldflags", "-s -X main.version=$VERSION", "./internal/app/bender"]

# Build the actual container
FROM alpine:latest
LABEL maintainer="Sierra Softworks <admin@sierrasoftworks.com>"

COPY --from=0 /src/bin/bender /bin/bender
ADD configs/quotes.json /etc/quotes.json

EXPOSE 8080

LABEL VERSION=$VERSION

WORKDIR /bin
ENTRYPOINT ["/bin/bender"]
CMD ["--quotes", "/etc/quotes.json"]