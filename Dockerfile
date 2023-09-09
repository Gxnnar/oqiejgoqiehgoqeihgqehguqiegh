FROM rust:1.70.0 as build

RUN cargo new --bin school_proxy
WORKDIR /school_proxy

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y sqlite3 libsqlite3-dev

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build -r
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/school_proxy*
RUN cargo build -r

FROM debian:latest

WORKDIR /school_proxy
COPY --from=build /school_proxy/target/release/school_proxy .
COPY ./web ./web

EXPOSE 8080
CMD ["./school_proxy"]