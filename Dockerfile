FROM rust:1.61 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/db-test ./target/release/db-test
CMD ["/target/release/db-test"]