FROM rust:1.86.0

WORKDIR /app

COPY . /app

# install
RUN cargo build

# build release
RUN cargo build --release

EXPOSE 8083

ENTRYPOINT ["/app/target/release/rust-app"]