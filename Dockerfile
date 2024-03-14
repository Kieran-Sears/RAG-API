FROM rust:1.76 AS builder

WORKDIR /app

COPY . .

# Compile the Rust code
RUN cargo build --release

# Final stage: create a minimal runtime image
FROM debian:buster-slim

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rustfomers /app/rustformers

# Set the entry point
CMD ["./rustformers"]
