FROM rust:bullseye as builder

# Set environment variables
ENV USER=rustuser \
    UID=1000 \
    GID=1000

# Create a new user and group in the builder stage
RUN groupadd -g $GID $USER && \
    useradd -m -u $UID -g $GID -s /bin/bash $USER

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() {}' > src/main.rs

RUN chown -R $USER:$USER /app

RUN cargo build --release 

COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-gnu 

FROM debian:bullseye 

LABEL org.opencontainers.image.source = "https://github.com/locmai/promguard"

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/promguard promguard

EXPOSE 8080

CMD ["/app/promguard"]
