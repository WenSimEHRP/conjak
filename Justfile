# for dummies
default:
    just --list

# build the wasm binary
build_wasm:
    cargo build --release --target wasm32-unknown-unknown
    cp ./target/wasm32-unknown-unknown/release/*.wasm ./src

# build the package's documentation
build_package_docs:
    typst c docs.typ \
        --input version="$(git describe --tags || echo "UNKNOWN")" \
        --input date="$(git log --max-count=1 --pretty='%cd' --date=iso-strict)"
