FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine AS runtime
RUN addgroup -S myuser \
  && adduser -S myuser -G myuser \
  && apk add protoc protobuf-dev 
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/runner \
  /app/.env \
  /usr/local/bin/

USER myuser
CMD ["/usr/local/bin/runner"]
