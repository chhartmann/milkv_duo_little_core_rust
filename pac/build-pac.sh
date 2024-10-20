#! /bin/bash

set -euxo pipefail

cd -- "$(dirname "$0")"

svd2rust --target riscv -i SG2000.svd
form -i lib.rs -o src/
rm lib.rs
cargo fmt
