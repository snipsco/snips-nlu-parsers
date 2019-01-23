#!/bin/bash
set -ev

source .travis/common.sh

echo "Running rust tests..."
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --all
cargo test --all

if [ "$TRAVIS_BRANCH" == "master" ]; then
  echo "Running python tests..."
  cd python
  python -m pip install tox
  tox
  cd ../..
fi
