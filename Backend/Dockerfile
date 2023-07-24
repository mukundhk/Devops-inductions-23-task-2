FROM rust:1.71-slim-bullseye

RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get -y install libpq-dev

WORKDIR /backend
COPY . /backend

RUN cargo install diesel_cli --no-default-features --features postgres

RUN chmod +x init.sh

ENTRYPOINT ["./init.sh"]
