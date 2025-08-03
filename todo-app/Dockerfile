# Build Stage
FROM rust:1.88 as builder


WORKDIR /app

COPY . .

RUN cargo build --release


# Runtime
FROM debian:bullseye-slim

COPY --from=builder /app/target/release/todo-app  /usr/local/bin/todo-app

CMD [ "todo-app" ]
