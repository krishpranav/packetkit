#!/bin/sh
set -ex

case "$BUILD_MODE" in
    release)
        RUST_LOG=packetkit target/$TARGET/release/packetkit -r pcaps/SkypeIRC.pcap > /dev/null
        ;;
    boxxy)
        if ! cat ci/boxxy_stage0.txt | RUST_LOG=boxxy cargo run --example boxxy | grep -q 'cargo run --example boxxy'; then
            echo SANDOX ERROR: expected match
            exit 1
        fi

        if ! cat ci/boxxy_stage1.txt | RUST_LOG=boxxy cargo run --example boxxy | grep -q 'cargo run --example boxxy'; then
            echo SANDOX ERROR: expected match
            exit 1
        fi

        if cat ci/boxxy_stage2.txt | RUST_LOG=boxxy cargo run --example boxxy | grep -q 'cargo run --example boxxy'; then
            echo SANDOX ERROR: expected NO match
            exit 1
        fi
        ;;
    reprotest)
        docker run --privileged reprotest-packetkit ci/reprotest.sh || true
        ;;
    cross)
        # do not execute tests when cross compiling
        ;;
esac
