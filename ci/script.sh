set -euxo pipefail

main() {
    if [[ ! $string =~ .*linux.* ]]; then
        sed -i "s/linux-embedded-hal/#linux-embedded-hal/g" Cargo.toml
        sed -i "s/embedded-hal-mock/#embedded-hal-mock/g" Cargo.toml
    fi

    cargo check --target $TARGET
    cargo build --target $TARGET --release
    if [[ $string =~ .*linux.* ]]; then
        cargo build --target $TARGET --examples
    fi

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        if [[ $string =~ .*linux.* ]]; then
            cargo test --target $TARGET
        fi
    fi

    if [[ ! $string =~ .*linux.* ]]; then
        sed -i "s/#linux-embedded-hal/linux-embedded-hal/g" Cargo.toml
        sed -i "s/#embedded-hal-mock/embedded-hal-mock/g" Cargo.toml
    fi
}

main
