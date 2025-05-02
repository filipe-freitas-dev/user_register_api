FROM rust:1.86-slim-bullseye

ARG APP_NAME=user_register_api

RUN apt-get update && apt-get install -y \
    build-essential \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . /app

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/bin/server"]