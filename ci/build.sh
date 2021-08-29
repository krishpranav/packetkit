#!/bin/sh
set -ex

case "$TARGET" in
    aarch64-unknown-linux-gnu)
        export RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc-6"
        ;;
esac

case "$BUILD_MODE" in
    release)
        cargo build --verbose --release --target="$TARGET"
        ls -lah "target/$TARGET/release/packetkit"
        file "target/$TARGET/release/packetkit"
        ;;
    boxxy)
        cargo build --verbose --examples
        ;;
    reprotest)
        docker build -t reprotest-packetkit -f docs/Dockerfile.reprotest .
        ;;
    cross)
        docker build --build-arg TARGET="$TARGET" -t "packetkit-test-$TARGET" -f ci/Dockerfile .
        # restart this script but inside the container and without BUILD_MODE=cross
        docker run -e TARGET="$TARGET" "packetkit-test-$TARGET" ci/build.sh
        ;;
esac
