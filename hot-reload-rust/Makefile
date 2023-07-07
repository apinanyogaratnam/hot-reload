.PHONY: build

CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER := x86_64-linux-gnu-gcc
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER

CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER := aarch64-linux-gnu-gcc
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER

build:
	rustup target add x86_64-unknown-linux-gnu
	rustup target add aarch64-unknown-linux-gnu
	cargo build --target x86_64-unknown-linux-gnu --release
	cargo build --target aarch64-unknown-linux-gnu --release
