# Use a Rust base image with Cargo installed
FROM rust:1.78.0 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Now copy the source code
COPY ./src ./src

# Build your application
RUN cargo build --release

# Start a new stage to create a smaller image without unnecessary build dependencies
FROM apt-get update && apt-get install -y curl && apt-get clean

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/app/target/release/basic_rust_app ./

# Expose any necessary ports
EXPOSE 8000

# Command to run the application
CMD ["./basic_rust_app"]
