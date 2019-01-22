#!/bin/bash
set -ev

source .travis/common.sh

echo "Running rust tests..."
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --all
cargo test --all

echo "Running python tests..."
cd python
python -m pip install tox
tox
cd ../..
