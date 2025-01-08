FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libpq-dev && cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/project .
COPY .env .
CMD ["./project"]
