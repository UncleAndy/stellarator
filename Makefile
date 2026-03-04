.PHONY: cuda build

cuda:
	./compile_cuda.sh

build: cuda
	cargo build
