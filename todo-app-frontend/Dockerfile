# Build Stage
FROM rust:1.88-slim-bullseye as builder

ARG HERO_IMAGE_URL=https://picsum.photos/800
ENV HERO_IMAGE_URL=${HERO_IMAGE_URL}

# Install dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY index.html ./
COPY src ./src

RUN trunk build --release

# Runtime
FROM nginx:stable-alpine AS runtime

#Remove default nginx static html
RUN rm -rf /usr/share/nginx/html/*

COPY --from=builder /app/dist  /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 3000

CMD [ "nginx", "-g", "daemon off;" ]
