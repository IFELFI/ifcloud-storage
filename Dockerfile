# Build the application
FROM clux/muslrust:stable AS builder
# Set the working directory
WORKDIR /app
# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create runtime image
FROM alpine:latest AS runtime
# Set the working directory
WORKDIR /app
# Install the protobuf compiler
RUN apk add protoc protobuf-dev
# Copy the built application into the container
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/runner \
  ./bin/runner

CMD ["./bin/runner"]
