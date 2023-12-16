FROM rust AS build
WORKDIR /src

RUN USER=root cargo new --bin node
WORKDIR /src/node

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/node*
RUN cargo build --release

WORKDIR /src

FROM rust:slim
WORKDIR /src

COPY --from=build /src/node/target/release/node ./node

CMD ["./node"]
