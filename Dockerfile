FROM fedora:33
RUN dnf update -y && dnf clean all -y
WORKDIR /usr/local/bin
COPY ./target/release/upl_microservice /usr/local/bin/upl_microservice
STOPSIGNAL SIGINT
ENTRYPOINT ["upl_microservice"]
