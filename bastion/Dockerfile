FROM rust

WORKDIR /src

RUN apt update
RUN apt install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
RUN echo DATABASE_URL=postgresql://postgres:development@postgres_db:5432/3d2a > .env

COPY . ./migrations

CMD ["diesel", "migration", "run"]