# The current directory containing the Makefile.
ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

VERSION  ?= 1

.DEFAULT: all
.PHONY: all build build-env clean push push-build-env

all: build

build:
	docker run --rm -it -v $(ROOT_DIR):/build jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build cargo build --release
	mkdir -p $(ROOT_DIR)/target/release/staging
	cp $(ROOT_DIR)/target/release/k8s-aws-nlb-proxy-protocol-operator $(ROOT_DIR)/target/release/staging/k8s-aws-nlb-proxy-protocol-operator
	docker build -t jarrednicholls/k8s-aws-nlb-proxy-protocol-operator:$(VERSION) -f $(ROOT_DIR)/Dockerfile $(ROOT_DIR)/target/release/staging
	docker tag jarrednicholls/k8s-aws-nlb-proxy-protocol-operator:$(VERSION) jarrednicholls/k8s-aws-nlb-proxy-protocol-operator

build-env:
	docker build -t jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build -f $(ROOT_DIR)/Dockerfile.build_env $(ROOT_DIR)/src

clean:
	docker run --rm -it -v $(ROOT_DIR):/build jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build cargo clean

push:
	docker push jarrednicholls/k8s-aws-nlb-proxy-protocol-operator:$(VERSION)
	docker push jarrednicholls/k8s-aws-nlb-proxy-protocol-operator:latest

push-build-env:
	docker push jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build
