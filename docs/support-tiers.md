# Support tiers for c2patool

This document defines the levels of support that this project provides for various build configurations.
It's adapted from [c2pa-rs's support tiers document](https://github.com/contentauth/c2pa-rs/blob/main/docs/support-tiers.md) – this repo's tier structure is deliberately simpler, since c2patool is a single binary shipped for a small, fixed set of platforms rather than a library with many optional build configurations.
These levels of support are inspired by the Rust language project's [Target Tier Policy](https://doc.rust-lang.org/rustc/target-tier-policy.html) and use similar language.

The CAI team will determine, at its discretion and with input from various internal and external client teams, which configurations are supported at which tier.
The CAI team will always announce a change in tier support with at least a minor semantic version bump.

## Definition of a build configuration

A _build configuration_ will specify:

* A Rust build tuple (e.g. `x86_64-pc-windows-msvc` for 64-bit Windows).
  * Unless otherwise specified this is executed on the `(platform)-latest` runner image [provided by GitHub](https://github.com/actions/runner-images).
* A Rust version specifier, which will be one of:
  * `stable` ([the most recent "stable" release](https://blog.rust-lang.org/releases/latest))
  * `MSRV` (the oldest release supported by this project, currently `1.88.0`, tracked in `Cargo.toml`'s `rust-version` field)
* A feature flag set. c2patool currently ships one meaningful choice here: the default (`networking`) feature, which pulls in `c2pa`'s remote-manifest-fetching support.
* A crypto library, which is not independently selectable the way it is in c2pa-rs: c2patool's `Cargo.toml` pins `openssl` on every native target and `rust_native_crypto` on `wasm32-wasip2` (openssl doesn't build for wasm).
* On platforms where relevant, a C library identifier (i.e. `glibc` or `musl`). c2patool only tests against `glibc`.

## How the tiers gate merges and releases

The tiers map onto the branching model in [c2patool's own release process](release-process.md#validation-gating).
Unlike c2pa-rs, which splits its per-PR merge gate (Tier 1A) from a slower cross-platform-and-MSRV gate (Tier 1B) that only runs against release-targeting branches, c2patool folds both into a single required workflow, [`ci.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/ci.yml):

* c2patool ships a macOS/Linux/Windows binary as its core product, and has real per-OS code (e.g. the `cfg(windows)` `windows-sys` dependency), so cross-platform coverage isn't a lower-priority concern the way it is for a library crate.
* There is no separate slower tier to catch a cross-platform or MSRV regression if it were left out of the fast gate, so both run in the same required workflow on every pull request.

As a result, **Tier 1A is the merge gate for `main`, and is also required for any pull request targeting a release-line (`stable`, `v0.x`) or release-candidate (`*-rc*`) branch** – including backport PRs and the `release-plz` release PR.
A daily scheduled run of the same workflow also validates `main` on a nightly cadence.

See [validation gating](release-process.md#validation-gating) in the release process for how this fits the overall flow.

## Tier 1A

Tier 1A configurations are the most actively supported.
A Tier 1A configuration will:

* Have continuous integration tests that build and pass for this build configuration on every commit to `main`, as well as to the release-line (`stable`, `v0.x`) and release-candidate (`*-rc*`) branches described in the [release process](release-process.md).
Failing tests block the pull request.
* This test suite is the most complete set of tests available for this component.
* Tier 1A configurations _may_ also have built artifacts generated for each versioned release.
The location where these artifacts are published will be documented.

The [`ci.yml` workflow](https://github.com/contentauth/c2patool/blob/main/.github/workflows/ci.yml) enforces these requirements.

### Tier 1A for c2patool

* **Ubuntu:** `x86_64-unknown-linux-gnu`, Rust `stable` | `MSRV`, default (`networking`) feature, `openssl`, `glibc`
* **macOS:** `aarch64-apple-darwin` (GitHub's `macos-latest` runner), Rust `stable` | `MSRV`, default feature, `openssl`
* **Windows:** `x86_64-pc-windows-msvc`, Rust `stable` | `MSRV`, default feature, `openssl`
* **WASI:** `wasm32-wasip2`, Rust `nightly-2026-01-16`, `rust_native_crypto`, run under `wasmtime`

The same workflow also runs Clippy, the `cargo +nightly fmt` check, a default-features `cargo check`, a `cargo-deny` license/vulnerability audit, and an internal-docs build (`ubuntu-latest`, `stable` only).
These don't vary by platform, so they aren't separate build configurations, but they gate merges the same way.

**Build artifacts:** [`c2patool-release.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/c2patool-release.yml) builds and publishes a binary (plus an SBOM) for every versioned release and every release-candidate build:

* **macOS:** universal binary (`aarch64-apple-darwin` and `x86_64-apple-darwin` combined via `lipo`)
* **Linux:** `x86_64-unknown-linux-gnu`
* **Windows:** `x86_64-pc-windows-msvc`

These are published to the [releases page](https://github.com/contentauth/c2patool/releases); the macOS build is additionally distributed via [Homebrew](https://brew.sh/).

## Tier 1B

A Tier 1B configuration will:

* Have continuous integration tests that build and pass for every versioned release.
Failing tests block the release.
* This test suite should be the same as for Tier 1A.
* Tier 1B configurations _may_ also have built artifacts generated for each versioned release.
The location where these artifacts are published will be documented.

A decision to place a configuration in Tier 1B is typically made because the CI test suite for this configuration adds significantly to the time required to complete a PR validation and the likelihood of finding issues that are specific to this configuration is deemed low.

There are no current Tier 1B configurations for c2patool: as explained under [how the tiers gate merges and releases](#how-the-tiers-gate-merges-and-releases), the cross-platform and MSRV coverage that c2pa-rs places in Tier 1B runs directly in c2patool's Tier 1A gate instead.

## Tier 2

A Tier 2 configuration will:

* Have continuous integration tests that _build_ for this build configuration for each versioned release.
A failing build blocks the release.
* A test suite that is a subset of the Tier 1 test suite may be defined for this build configuration.
If it exists, a failing test suite blocks the release.
* Generally avoid Tier 2, but it may be necessary when a fully-native execution environment is not available to us.
* If built artifacts are generated for this build configuration, they should be built for every versioned release and the location should be documented.

There are no current Tier 2 configurations for c2patool.

## Tier 3

A Tier 3 configuration is experimental and minimally supported.
It has been shown to work at one time, but no special effort is made to ensure that such a configuration can be built on an ongoing basis.

There are no current Tier 3 configurations for c2patool.
