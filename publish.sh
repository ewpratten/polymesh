#! /bin/bash

set -e

echo "Publishing core library"
cargo publish --manifest-path libpolymesh/Cargo.toml

echo "Publishing all tools"
cargo publish --manifest-path tools/pmfcube/Cargo.toml
cargo publish --manifest-path tools/pmfextract/Cargo.toml
cargo publish --manifest-path tools/pmfpack/Cargo.toml
cargo publish --manifest-path tools/pmftree/Cargo.toml
cargo publish --manifest-path tools/pmfview/Cargo.toml
cargo publish --manifest-path tools/vox2pmf/Cargo.toml