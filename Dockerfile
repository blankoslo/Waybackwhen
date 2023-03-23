FROM rust:1.68.0-buster
# Install dependencies
RUN apt-get update && \
    apt-get install -y chromium && \
    rm -rf /var/lib/apt/lists/*

# set working directory
WORKDIR /app

COPY . .

RUN cargo build --release


CMD ["./target/release/waybackwhen"]




