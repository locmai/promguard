FROM rust:bullseye as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target "$(uname -m)-unknown-linux-gnu"

RUN cp "/app/target/$(uname -m)-unknown-linux-gnu/release/promguard" /usr/local/bin/promguard

FROM scratch

LABEL org.opencontainers.image.source = "https://github.com/locmai/promguard"

WORKDIR /app

COPY --from=builder /usr/local/bin/promguard promguard

EXPOSE 8080

CMD ["/app/promguard"]