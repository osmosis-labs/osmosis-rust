#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )


# get latest osmosis tag timestamp from workflow state
LATEST_OSMOSIS_TAG_TIMESTAMP_PATH="$SCRIPT_DIR/../workflow-state/LATEST_OSMOSIS_TAG_TIMESTAMP"
LATEST_OSMOSIS_TAG_TIMESTAMP=$(cat "$LATEST_OSMOSIS_TAG_TIMESTAMP_PATH" || echo 0)


git submodule update --init --recursive 
cd dependencies/osmosis

# list all branches/tags with:
# `<branch_name> <commit_hash>`
FORMAT="%(refname:short) %(committerdate:unix)"

# get all related revisions
REVS="$(git branch -r --format="$FORMAT" --list origin/main && \
    git branch -r --format="$FORMAT" --list origin/v* && \
    git tag --format="$FORMAT" --list v*)"

# filter only rev that's greater than latest tag
MATRIX=$(echo "$REVS" | awk -v latest_tag_timestamp="$LATEST_OSMOSIS_TAG_TIMESTAMP" '$2 > latest_tag_timestamp { print $1 }' | jq -RMrnc '{ "target": [inputs] }')

# update latest tag timestmap
rm -f "$LATEST_OSMOSIS_TAG_TIMESTAMP_PATH"
LATEST_OSMOSIS_TAG_TIMESTAMP="$(git tag --format="$FORMAT" | awk '{ print $2 }' | sort -nr | head -n 1)"
echo "$LATEST_OSMOSIS_TAG_TIMESTAMP" > "$LATEST_OSMOSIS_TAG_TIMESTAMP_PATH"

git add "$LATEST_OSMOSIS_TAG_TIMESTAMP_PATH"
git commit -m "Update latest osmosis tag timestamp to $LATEST_OSMOSIS_TAG_TIMESTAMP"
git push

# pass along target rev matrix
echo "matrix=$MATRIX" >> $GITHUB_OUTPUT