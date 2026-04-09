#!/usr/bin/env bash
set -euo pipefail

pushd crates/kreuzcrawl-node >/dev/null
node_tgz="$(npm pack | tail -1 | tr -d '\r')"
mv "$node_tgz" "$GITHUB_WORKSPACE/$node_tgz"
popd >/dev/null

echo "tarball=$GITHUB_WORKSPACE/$node_tgz" >>"$GITHUB_OUTPUT"
