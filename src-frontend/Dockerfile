# --- STAGE 1: Builder ---
FROM rust:slim-bookworm AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the pre-compiled binary from your local project folder
# Make sure you ran 'scp' to put the 'cargo-leptos' binary here first!
COPY cargo-leptos /usr/local/bin/cargo-leptos
RUN chmod +x /usr/local/bin/cargo-leptos

WORKDIR /app
COPY . .

# Compile the production-ready binary and style assets
# Since we injected the binary, this command will now run instantly
# Debugging: List the directory contents and print the Cargo.toml to ensure it's there
RUN ls -R /app && cat /app/Cargo.toml

# Run the build with the environment variable
ENV LEPTOS_CONFIG_PATH=/app/Cargo.toml
RUN cargo-leptos build --release

# Enable verbose cargo output via environment variable
ENV CARGO_TERM_VERBOSE=true
ENV LEPTOS_CONFIG_PATH=/app/Cargo.toml

# --- STAGE 2: Lightweight Runner ---
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Pull the compiled binary and web assets from the builder platform
COPY --from=builder /app/target/release/subgrid /app/subgrid
COPY --from=builder /app/target/site /app/site

# Expose the standard app engine port
EXPOSE 3000

ENV LEPTOS_OUTPUT_NAME="subgrid"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"

CMD ["/app/subgrid"]
