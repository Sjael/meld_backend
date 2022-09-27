FROM rust:1.61 AS builder
COPY . .
RUN cargo build --release

CMD ["/target/release/db-test"]