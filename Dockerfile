FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY ./target/release/upl_microservice /usr/local/bin/upl_microservice
RUN apt-get update && apt-get install -y
RUN apt-get install curl -y
STOPSIGNAL SIGINT
ENTRYPOINT ["upl_microservice"]