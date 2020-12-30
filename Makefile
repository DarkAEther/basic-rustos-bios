.PHONY: build run default clean

default: build

build: src/main.rs
	export RUST_TARGET_PATH=$(pwd)
	xargo bootimage --target=x86_64-unknown-rustos-none
	mkdir -p build
	cp target/x86_64-unknown-rustos-none/debug/bootimage-rustos.bin build/bootimage-rustos.bin

run: build
	qemu-system-x86_64 -drive format=raw,file=build/bootimage-rustos.bin

part-clean:
	rm -r build

full-clean:
	rm -r build
	xargo clean
