FROM rust:latest

# TODO multi stage build to minimize image.

WORKDIR /usr/src/app
# FOrbidden outside path build context
#COPY ../Cargo.lock .
COPY ./Cargo.toml .
COPY ./src ./src

RUN cargo install --path . --debug

EXPOSE 4001

CMD ["todo"]

