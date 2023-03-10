FROM rust:1-slim-buster
WORKDIR /backend

COPY ./ ./

ENV SQLX_OFFLINE true
RUN cargo build --release

CMD BASE_URL=0.0.0.0 ./target/release/backend
