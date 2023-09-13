FROM debian:bookworm

RUN apt-get -y update \
  && apt-get clean \
  && apt-get -y install \
    curl \
    unzip \
    dpkg

RUN mkdir -p /usr/src/rust_jobs_api

WORKDIR /usr/src/rust_jobs_api

COPY . /usr/src/rust_jobs_api

ENV DB_HOST="host.docker.internal"
ENV SOCKET_ADDRESS=0.0.0.0:80

RUN ./build.sh

RUN dozer --version

CMD dozer run