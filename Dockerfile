FROM rust:lastest AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/db-test ./target/release/db-test
ENV PORT = 4000
EXPOSE 4000
CMD ["/target/release/db-test"]