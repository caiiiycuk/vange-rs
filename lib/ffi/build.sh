set -xe

cargo build --release --features android_logger --target aarch64-linux-android
cargo build --release --features android_logger --target arm-linux-androideabi
cargo build --release --features android_logger --target x86_64-linux-android
cargo build --release --features android_logger --target i686-linux-android

cp -v ../../target/aarch64-linux-android/release/librusty_vangers.a ~/gamepix/games/vangers-android/native/librusty/aarch64-linux-android/release/
cp -v ../../target/arm-linux-androideabi/release/librusty_vangers.a ~/gamepix/games/vangers-android/native/librusty/arm-linux-androideabi/release/
cp -v ../../target/x86_64-linux-android/release/librusty_vangers.a ~/gamepix/games/vangers-android/native/librusty/x86_64-linux-android/release/
cp -v ../../target/i686-linux-android/release/librusty_vangers.a ~/gamepix/games/vangers-android/native/librusty/i686-linux-android/release/
