#!/bin/bash

set -ex

rustup target add aarch64-linux-android
rustup target add arm-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

pushd vange-rs/lib/ffi

cargo build --release --features android_logger --target aarch64-linux-android
cargo build --release --features android_logger --target arm-linux-androideabi
cargo build --release --features android_logger --target x86_64-linux-android
cargo build --release --features android_logger --target i686-linux-android

# native platform
cargo build --release

popd