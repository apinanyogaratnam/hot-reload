# Hot Reload Rust

## Had to install this for cross compiling using m1 mac

(https://github.com/messense/homebrew-macos-cross-toolchains)

brew tap messense/macos-cross-toolchains
# install x86_64-unknown-linux-gnu toolchain
brew install x86_64-unknown-linux-gnu
# install aarch64-unknown-linux-gnu toolchain
brew install aarch64-unknown-linux-gnu
# install x86_64-unknown-linux-musl toolchain
brew install x86_64-unknown-linux-musl

brew tap SergioBenitez/osxct
brew install FiloSottile/musl-cross/musl-cross
TARGET_CC=x86_64-linux-musl-gcc \
RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" \
cargo build --target=x86_64-unknown-linux-musl


rustup target list
