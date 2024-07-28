#! /bin/bash

# Where this script is located
# https://stackoverflow.com/a/246128/3837223
this_name="$(basename "$0")"
this_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

# Make a temporary directory
td=$(mktemp -d)

# Where the destination directory is located
tests_dest="${this_dir}/../tests/"
mkdir -p "${tests_dest}"

# Download the example project to the temporary directory (sparsely)
git clone --sparse --depth=1 --filter=blob:none \
  https://github.com/kyclark/command-line-rust.git \
  "${td}"

# Copy necessary code in a subshell
(
  cd "${td}" &&
    git sparse-checkout add 03_catr &&
    cd 03_catr &&
    ./mk-outs.sh &&
    mv -iv ./tests/* "${tests_dest}"
)

# Clean up the temporary directory
rm -rf "${td}"

# Install necessary crates
cargo add anyhow
cargo add clap --features derive
cargo add --dev assert_cmd predicates pretty_assertions rand

# Run test
cargo test --quiet
