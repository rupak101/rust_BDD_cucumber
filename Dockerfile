# 1. This tells docker to use the Rust official image
FROM rust:1.75

# 2. Copy the files in the machine to the Docker image
COPY ./ ./

# Build the program for release
RUN cargo build --release

# Set the environment variables for the container
ENV RUST_LOG=info

# Copy the .env file to the container's root
COPY .env .env

# Run the application
CMD ["cargo", "test", "--release"]
