.PHONY: build run default clean

default: build

build: src/
	cargo bootimage
	mkdir -p build
	cp target/x86_64-unknown-rustos-none/debug/bootimage-rustos.bin build/bootimage-rustos.bin

run: build
	cargo run

part-clean:
	rm -r build

full-clean:
	rm -r build
	cargo clean
