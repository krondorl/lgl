FROM rust:1.92 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/lgl /usr/local/bin/lgl
ENTRYPOINT ["lgl"]