#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

TEMP="$SCRIPT_DIR/../tmp"

mkdir -p "$TEMP/tmp"

# list all branches with:
# `<branch_name> <commit_hash>`
git branch -r --format="%(refname:short) %(objectname)" --list origin/main > "$TEMP/branches"


# diff file / input stream
# if there is a diff, change 
