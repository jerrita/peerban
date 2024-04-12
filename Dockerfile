FROM rust as builder-amd64
ENV TARGET x86_64-unknown-linux-musl

FROM rust as builder-arm64
ENV TARGET aarch64-unknown-linux-musl

FROM builder-${TARGETARCH} as builder
WORKDIR /usr/src/peerban
COPY . .
RUN rustup target add ${TARGET}
RUN cargo build --profile opt --target ${TARGET}

FROM scratch
COPY --from=builder /usr/src/peerban/target/*/opt/peerban /
CMD ["/peerban"]
