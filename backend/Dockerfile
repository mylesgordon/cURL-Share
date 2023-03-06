FROM rust:1-slim-buster
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/backend"]