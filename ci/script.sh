set -euxo pipefail

main() {
    if [[ ! $TARGET =~ .*linux.* ]]; then
        sed -i "s/linux-embedded-hal/#linux-embedded-hal/g" Cargo.toml
        sed -i "s/embedded-hal-mock/#embedded-hal-mock/g" Cargo.toml
    fi

    cargo check --target $TARGET
    cargo build --target $TARGET --release
    if [[ $TARGET =~ .*linux.* ]]; then
        cargo build --target $TARGET --examples
    fi

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        if [[ $TARGET =~ .*linux.* ]]; then
            cargo test --target $TARGET
        fi
    fi

    if [[ ! $TARGET =~ .*linux.* ]]; then
        sed -i "s/#linux-embedded-hal/linux-embedded-hal/g" Cargo.toml
        sed -i "s/#embedded-hal-mock/embedded-hal-mock/g" Cargo.toml
    fi
}

main
