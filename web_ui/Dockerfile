FROM rust AS build
WORKDIR /src

RUN USER=root cargo new --bin web_ui
WORKDIR /src/web_ui

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/web_ui*
RUN cargo build --release

WORKDIR /src

FROM rust:slim
WORKDIR /src

COPY --from=build /src/web_ui/src/assets ./src/assets
COPY --from=build /src/web_ui/src/templates ./src/templates
COPY --from=build /src/web_ui/target/release/web_ui ./web_ui

CMD ["./web_ui"]
