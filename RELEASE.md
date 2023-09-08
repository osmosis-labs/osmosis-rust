# Release Process

# Release Process

This repository contains `osmosis-std` and `osmosis-std-derive` crates. 

`osmosis-std-derive` is a procedural macro crate intended for use by the `osmosis-std` crate. 
If there have been any changes to `osmosis-std-derive` since the last release, it should be released first before releasing `osmosis-std`.

There is no specific tag for the `osmosis-std-derive` version as it is intended for internal use by `osmosis-std` (it needs to be a separate crate since it is a procedural macro and requires a specific crate type).

## Pre-requisite
- Make sure that the current change you are going to release match the intended osmosis version by running
```bash
SKIP_GIT_UPDATE=1 ./scripts/update-and-rebuild.sh <osmosis-revision>
```

For example, v19 release:
```bash
SKIP_GIT_UPDATE=1 ./scripts/update-and-rebuild.sh v19.0.0
```

The `SKIP_GIT_UPDATE=1` is there for not creating new autobuild branch.

The script 

- Make sure that you are one of [crate publisher](https://github.com/orgs/osmosis-labs/teams/crate-publishers)


## Release `osmosis-std`
- Check if there is any change to `osmosis-std-derive` since last release of `osmosis-std`
- If there is a change
    - [Release `osmosis-std-derive`](#release-osmosis-std-derive)
    - Bump `osmosis-std-derive` *dependency* version in `packages/osmosis-std/Cargo.toml`
    - Bump `osmosis-std` version in `packages/osmosis-std/Cargo.toml`
- Release new version of `osmosis-std` by running `cargo publish -p osmosis-std`
- Create github release with `osmosis-std` version as a tag name and release title


## Release `osmosis-std-derive`
This one is optional if there is no change to `osmosis-std-derive` since last release of `osmosis-std`
- Bump `osmosis-std-derive` version in `packages/osmosis-std-derive/Cargo.toml`
- Release new version of `osmosis-std-derive` by running `cargo publish -p osmosis-std-derive`