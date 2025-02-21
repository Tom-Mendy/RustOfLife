FROM rust AS dependencies

WORKDIR /app

# install sdl2
RUN apt-get update && apt-get install -y libsdl2-dev libsdl2-ttf-dev

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

COPY . .

# Build the dependencies
RUN cargo build --release

FROM alpine:3.15 AS run

WORKDIR /app

COPY --from=dependencies /app/target/release/rust_of_life /rust_of_life

CMD [ "mv", "/rust_of_life", "/app/rust_of_life" ]
