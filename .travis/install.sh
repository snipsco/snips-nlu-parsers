#!/bin/bash
set -ev

source .travis/common.sh

perl -p -i -e \
    "s/^snips-nlu-parsers-ffi-macros = .*\$/snips-nlu-parsers-ffi-macros = { path = \"..\/..\/ffi\/ffi-macros\" \}/g" \
    python/ffi/Cargo.toml
