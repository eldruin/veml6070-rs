set -euxo pipefail

main() {
    rustup component add rust-src
    SYSROOT=$(rustc --print sysroot)
    if [[ ! "$SYSROOT" =~ "$TARGET" ]]; then
        rustup target add $TARGET
    else
        echo "Target $TARGET is already installed"
    fi
    source ~/.cargo/env || true
    mv .cargo ._cargo
    cargo install cargo-travis || true
    mv ._cargo .cargo
}

main
