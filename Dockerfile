FROM rust:1.83 AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release
FROM debian:bookworm-slim
COPY --from=build /app/target/release/actix-starter /app
EXPOSE 8080
CMD ["/app"]
