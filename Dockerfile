FROM rust:1.54-buster

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update \
    && apt-get install -y valgrind redis \
    && cargo install --path .

CMD ["redis-clone", "--host", "0.0.0.0", "--port", "6379"]