.PHONY: build

CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER := x86_64-linux-gnu-gcc
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER

CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER := aarch64-linux-gnu-gcc
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER

CC_x86_64_unknown_linux_gnu := x86_64-linux-gnu-gcc
export CC_x86_64_unknown_linux_gnu

CXX_x86_64_unknown_linux_gnu := x86_64-linux-gnu-g++
export CXX_x86_64_unknown_linux_gnu

AR_x86_64_unknown_linux_gnu := x86_64-linux-gnu-ar
export AR_x86_64_unknown_linux_gnu

CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER := x86_64-linux-musl-gcc
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER

build:
	rustup target add x86_64-unknown-linux-gnu
	cargo build --target x86_64-unknown-linux-gnu --release

	rustup target add aarch64-unknown-linux-gnu
	cargo build --target aarch64-unknown-linux-gnu --release

	rustup target add x86_64-unknown-linux-musl
	cargo build --target x86_64-unknown-linux-musl
