#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
OSMOSIS_REV=${1:-main}



git add "$SCRIPT_DIR/.."
git commit -m "build"

BRANCH="autobuild-$OSMOSIS_REV"
git checkout -b "$BRANCH"
git push -uf origin "$BRANCH"
