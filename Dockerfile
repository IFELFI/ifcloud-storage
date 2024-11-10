FROM clux/muslrust:stable AS chef
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
RUN apk add protoc protobuf-dev 
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/runner \
  /usr/local/bin/

CMD ["/usr/local/bin/runner"]
