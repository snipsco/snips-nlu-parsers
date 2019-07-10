#!/bin/bash

# Install Rust
if [[ -z ${TRAVIS_RUST_VERSION+w} ]]; then
  curl https://sh.rustup.rs -sSf | bash -s -- -y
fi
