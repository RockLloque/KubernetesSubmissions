# Build Stage
FROM rust:1.88-slim-bullseye as builder

WORKDIR /app

# Install dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# Runtime
FROM debian:bullseye-slim

COPY --from=builder /app/target/release/todo-app  /usr/local/bin/todo-app

RUN mkdir -p /user/local

CMD [ "todo-app" ]
