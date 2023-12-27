FROM rust AS build
WORKDIR /src

RUN rustup override set nightly

RUN USER=root cargo new --bin node
WORKDIR /src/node

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations

RUN rm ./target/release/deps/node*
RUN cargo build --release

WORKDIR /src

FROM rust:slim

RUN apt-get update
RUN apt-get install -y libpq-dev

HEALTHCHECK CMD curl --fail http://localhost/health || exit 1

WORKDIR /src

COPY --from=build /src/node/target/release/node ./node
COPY ./config.yaml ./config.yaml

CMD ["./node"]
