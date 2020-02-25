FROM debian:buster-slim

RUN apt-get update -y && \
    apt-get install -y openssl && \
    apt-get clean

COPY k8s-aws-nlb-proxy-protocol-operator /usr/bin

ENTRYPOINT ["/usr/bin/k8s-aws-nlb-proxy-protocol-operator"]
