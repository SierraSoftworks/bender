# Use a mutli-stage build pipeline to generate the executable
FROM golang:1.9

ARG SENTRY_DSN=""
ARG VERSION="development"

ENV GO_PATH="/go"

ADD . $GO_PATH/src/github.com/SierraSoftworks/bender
WORKDIR $GO_PATH/src/github.com/SierraSoftworks/bender

RUN go get -t ./...
RUN go test -v ./...

ENV CGO_ENABLED=0
ENV GOOS=linux
RUN go build -o bin/bender -a -installsuffix cgo -ldflags "-s -X main.version=$VERSION -X main.sentryDSN=$SENTRY_DSN"

# Build the actual container
FROM alpine:latest
LABEL maintainer="Sierra Softworks <admin@sierrasoftworks.com>"

RUN apk add --update tini
ENTRYPOINT ["/sbin/tini", "--"]

COPY --from=0 /go/src/github.com/SierraSoftworks/bender/bin/bender /bin/bender
ADD quotes.json /etc/quotes.json

EXPOSE 8080

LABEL VERSION=$VERSION

WORKDIR /bin
ENTRYPOINT ["/bin/bender"]
CMD ["--quotes", "/etc/quotes.json"]