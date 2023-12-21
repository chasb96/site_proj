FROM rust

WORKDIR /src

RUN apt update
RUN apt install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./.env ./
COPY . ./migrations

CMD ["diesel", "migration", "run"]