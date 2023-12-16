FROM ubuntu

RUN apt-get update

RUN apt-get install -y \
    build-essential \
    curl \
    postgresql-client-common \
    postgresql-client-14 \
    libpq-dev

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install diesel_cli --no-default-features --features postgres

RUN diesel setup

COPY ./migrations ./migrations

# RUN diesel migration run