FROM rust AS build
WORKDIR /src

RUN USER=root cargo new --bin host
WORKDIR /src/host

COPY ./host/Cargo.lock ./Cargo.lock
COPY ./host/Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./host/src ./src

RUN rm ./target/release/deps/host*
RUN cargo build --release

WORKDIR /src

FROM rust:slim
WORKDIR /src

COPY --from=build /src/host/target/release/host ./host

CMD ["./host"]
