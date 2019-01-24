#!/usr/bin/env bash

NEW_VERSION=${1?"usage $0 <new version>"}

echo "Updating ontology version to ${NEW_VERSION}"
find . -name "Cargo.toml" -exec perl -p -i -e "s/snipsco\/snips-nlu-ontology\".*\$/snipsco\/snips-nlu-ontology\", tag = \"$NEW_VERSION\" }/g" {} \;
