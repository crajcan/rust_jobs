FROM rust:1.72-buster

RUN mkdir -p /usr/src/rust_jobs_api

WORKDIR /usr/src/rust_jobs_api

COPY . /usr/src/rust_jobs_api

ENV DB_HOST="host.docker.internal"
ENV SOCKET_ADDRESS=0.0.0.0:80

RUN ./usr/src/rust_jobs_api/build.sh

CMD dozer