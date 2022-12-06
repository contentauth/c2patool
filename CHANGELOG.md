# Changelog

All changes to this project are documented in this file.

This project adheres to [Semantic Versioning](https://semver.org), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

Do not manually edit this file. It will be automatically updated when a new release is published.

## 0.3.9
_06 December 2022_

* update to c2pa-rs 0.16.0
* allows clients to output manifest report to specified directory ([#91](https://github.com/contentauth/c2pa-rs/pull/91))

## 0.3.8
_09 November 2022_

* Bump c2pa from 0.13.2 to 0.15.0 ([#87](https://github.com/contentauth/c2pa-rs/pull/87))
* Build infrastructure improvements ([#85](https://github.com/contentauth/c2pa-rs/pull/85))
* Fix new Clippy warning in Rust 1.65 ([#84](https://github.com/contentauth/c2pa-rs/pull/84))
* Readme updates ([#62](https://github.com/contentauth/c2pa-rs/pull/62))

## 0.3.7
_22 September 2022_

* Treat a source asset with a manifest store as a default parent ([#76](https://github.com/contentauth/c2pa-rs/pull/76))
* Fetch remote manifests for --info ([#75](https://github.com/contentauth/c2pa-rs/pull/75))

## 0.3.6
_16 September 2022_

* Update Cargo.lock when publishing crate ([#71](https://github.com/contentauth/c2pa-rs/pull/71))
* [IGNORE] update readme --info ([#70](https://github.com/contentauth/c2pa-rs/pull/70))
* Update Cargo.lock to 0.3.5

## 0.3.5
_15 September 2022_

* Upgrade cpufeatures to non-yanked version ([#68](https://github.com/contentauth/c2pa-rs/pull/68))
* Add --info option  ([#65](https://github.com/contentauth/c2pa-rs/pull/65))
* Updated publish workflow to upload binaries to GitHub ([#58](https://github.com/contentauth/c2pa-rs/pull/58))
* Fix Make release script & update readme ([#55](https://github.com/contentauth/c2pa-rs/pull/55))
* (Some version history omitted as we worked on some release process issues)

## 0.3.0
_18 August 2022_

* (MINOR) Rework c2patool parameters ([#53](https://github.com/contentauth/c2pa-rs/pull/53))
* Update to 0.11.0 c2pa-rs ([#38](https://github.com/contentauth/c2pa-rs/pull/38))
* Remove Homebrew, Git installation methods, and add "update" wording ([#33](https://github.com/contentauth/c2pa-rs/pull/33))

## 0.2.1
_29 June 2022_

* Add BMFF support for video & etc ([#25](https://github.com/contentauth/c2pa-rs/pull/25))

## 0.2.0
_28 June 2022_

* (MINOR) Upgrade to c2pa Rust SDK version 0.6.0 ([#24](https://github.com/contentauth/c2pa-rs/pull/24))
* Fix an error in the README documentation ([#23](https://github.com/contentauth/c2pa-rs/pull/23))
* Display help if there are no arguments on the command line ([#21](https://github.com/contentauth/c2pa-rs/pull/21))
* Bump anyhow from 1.0.57 to 1.0.58 ([#17](https://github.com/contentauth/c2pa-rs/pull/17))
* Updates examples to use ta_url instead of ta ([#15](https://github.com/contentauth/c2pa-rs/pull/15))

## 0.1.3
_17 June 2022_

* Update to latest c2pa Rust SDK ([#12](https://github.com/contentauth/c2pa-rs/pull/12))
* Add built-in default certs to make getting started easier ([#9](https://github.com/contentauth/c2pa-rs/pull/9))

## 0.1.2
_10 June 2022_

* Update crate's description field

## 0.1.1
_10 June 2022_

* Initial public release
