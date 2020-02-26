# CentOS 7 has a build of OpenSSL that includes the FIPS 140-2 Object Module.
# This is the only reason we are using centos:7 as a base image :)
FROM centos:7

RUN yum install -y openssl && \
    yum clean all && \
    rm -rf /var/cache/yum

COPY k8s-aws-nlb-proxy-protocol-operator /usr/bin

ENTRYPOINT ["k8s-aws-nlb-proxy-protocol-operator"]
