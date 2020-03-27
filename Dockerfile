FROM debian:jessie AS builder

RUN apt-get update && apt-get install -y curl build-essential libpq-dev openssl libssl-dev pkg-config

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"

# Install rust WASM
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

ADD . ./

RUN wasm-pack build --target web --no-typescript

FROM nginx:alpine

COPY --from=builder /index.html /usr/share/nginx/html/
COPY --from=builder /pkg/front_bg.wasm /usr/share/nginx/html/
COPY --from=builder /pkg/front.js /usr/share/nginx/html/
COPY --from=builder /pkg/package.json /usr/share/nginx/html/