# The current directory containing the Makefile.
ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

.DEFAULT: all
.PHONY: all build build-env build-fips clean push push-build-env

all: build

build:
	docker run --rm -it -v $(ROOT_DIR):/build jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build cargo build --release
	mkdir -p $(ROOT_DIR)/target/release/staging
	cp $(ROOT_DIR)/target/release/k8s-aws-nlb-proxy-protocol-operator $(ROOT_DIR)/target/release/staging/k8s-aws-nlb-proxy-protocol-operator
	docker build -t jarrednicholls/k8s-aws-nlb-proxy-protocol-operator -f $(ROOT_DIR)/Dockerfile $(ROOT_DIR)/target/release/staging

build-env:
	docker build -t jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build -f $(ROOT_DIR)/build/Dockerfile $(ROOT_DIR)/build
	docker build -t jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build:fips -f $(ROOT_DIR)/build/Dockerfile.fips $(ROOT_DIR)/build

build-fips:
	docker run --rm -it -v $(ROOT_DIR):/build jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build:fips cargo build --release

clean:
	docker run --rm -it -v $(ROOT_DIR):/build jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build cargo clean

push:
	docker push jarrednicholls/k8s-aws-nlb-proxy-protocol-operator

push-build-env:
	docker push jarrednicholls/k8s-aws-nlb-proxy-protocol-operator-build
