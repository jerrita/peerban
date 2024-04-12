FROM --platform=linux/amd64 messense/rust-musl-cross:x86_64-musl as builder-amd64
FROM --platform=linux/amd64 messense/rust-musl-cross:aarch64-musl as builder-arm64

FROM --platform=linux/amd64 builder-${TARGETARCH} as builder
WORKDIR /usr/src/peerban
COPY . .
RUN cargo build --profile opt \
    && musl-strip target/*/opt/peerban

FROM scratch
COPY --from=builder /usr/src/peerban/target/*/opt/peerban /
CMD ["/peerban"]
