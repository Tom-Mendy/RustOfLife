FROM rust:alpine3.22 AS dependencies

WORKDIR /app

# install build and SDL2 dependencies
RUN apk add --no-cache \
		build-base \
		pkgconfig \
		cmake \
		sdl2-dev \
		sdl2_ttf-dev

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

COPY . .

# Build the dependencies
RUN cargo build --release

FROM alpine:3.22 AS run

WORKDIR /app

RUN apk add --no-cache \
		sdl2 \
		sdl2_ttf

COPY --from=dependencies /app/target/release/rust_of_life /app/rust_of_life

ENTRYPOINT ["/app/rust_of_life"]
