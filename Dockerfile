FROM rust:1.49.0 AS builder

WORKDIR /app

RUN cargo install cargo-strip

ADD . /app

RUN cargo build --release --all-features && cargo strip

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/pigeon /

CMD ["./pigeon"]

EXPOSE 8080
