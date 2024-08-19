FROM rust:1.80.1 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM ubuntu:24.04
RUN apt-get update && \
  apt-get install --no-install-recommends -y \
  libssl-dev=3.0.13-0ubuntu3.2 \
  ca-certificates=20240203 && \
  rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/VE_Server .
EXPOSE 8080

# Run the application
CMD ["./VE_Server"]
