#!/bin/bash
set -ev

export PATH="$HOME/.cargo/bin:$PATH"

if [[ "${RUST_TESTS}" == "true" ]]; then
    echo "Running rust tests..."
    cargo build --all
    cargo test --all
fi

if [[ "${PYTHON_TESTS}" == "true" ]]; then
  echo "Running python tests..."
  cd python
  python -m pip install tox
  tox
  cd ../..
fi
