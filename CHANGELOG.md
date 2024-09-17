# Changelog

All changes to this project are documented in this file.

This project adheres to [Semantic Versioning](https://semver.org), except that – as is typical in the Rust community – the minimum supported Rust version may be increased without a major version increase.

Do not manually edit this file. It will be automatically updated when a new release is published.

## 0.9.9
_17 September 2024_

* Pull in latest bug fixes ([#237](https://github.com/contentauth/c2pa-rs/pull/237))
* Document fragment subcommand ([#236](https://github.com/contentauth/c2pa-rs/pull/236))
* Switch back to using `pull_request` instead of `pull_request_target` trigger
* Bump actions/checkout from 3 to 4 ([#243](https://github.com/contentauth/c2pa-rs/pull/243))
* Remove no-longer-maintained clippy-check action ([#238](https://github.com/contentauth/c2pa-rs/pull/238))
## 0.9.8
_30 August 2024_

* Initial fragment support ([#230](https://github.com/contentauth/c2pa-rs/pull/230))
* Add warning about accessing a private key directly ([#218](https://github.com/contentauth/c2pa-rs/pull/218))
## 0.9.7
_15 August 2024_

* Update to latest c2pa SDK ([#222](https://github.com/contentauth/c2pa-rs/pull/222))
* Remove rust toolchain version lock ([#221](https://github.com/contentauth/c2pa-rs/pull/221))
* Update security guidance to link to SECURITY.md ([#217](https://github.com/contentauth/c2pa-rs/pull/217))
## 0.9.6
_30 July 2024_

* Pull latest c2pa-rs bug fixes into c2patool ([#212](https://github.com/contentauth/c2pa-rs/pull/212))
* only run tests/clippy if labeled ([#205](https://github.com/contentauth/c2pa-rs/pull/205))
* Bump env_logger from 0.10.2 to 0.11.4 ([#204](https://github.com/contentauth/c2pa-rs/pull/204))
* Updates cargo packages and cargo.deny file. ([#200](https://github.com/contentauth/c2pa-rs/pull/200))
## 0.9.5
_18 July 2024_

* Update to lastest c2pa-rs ([#197](https://github.com/contentauth/c2pa-rs/pull/197))
* added security.md ([#196](https://github.com/contentauth/c2pa-rs/pull/196))
## 0.9.4
_25 June 2024_

* Update c2patool ([#190](https://github.com/contentauth/c2pa-rs/pull/190))
* Match c2pa-rs minimum toolchain version and test in CI ([#188](https://github.com/contentauth/c2pa-rs/pull/188))
* Document how to specify an icon ([#182](https://github.com/contentauth/c2pa-rs/pull/182))
## 0.9.3
_29 May 2024_

* Remove binary modules ([#179](https://github.com/contentauth/c2pa-rs/pull/179))
## 0.9.2
_24 May 2024_

* Remove integration tests for now due to extraneous binaries ([#178](https://github.com/contentauth/c2pa-rs/pull/178))
## 0.9.1
_22 May 2024_

* Add better support for cargo-binstall ([#177](https://github.com/contentauth/c2pa-rs/pull/177))
## 0.9.0
_07 May 2024_

* Integrate with c2pa-rs 0.32.0, various test case fixes. ([#175](https://github.com/contentauth/c2pa-rs/pull/175))
* (MINOR) Add HTTP source option for trust config ([#174](https://github.com/contentauth/c2pa-rs/pull/174))
## 0.8.2
_28 March 2024_

* fixed c2patool asset name ([#171](https://github.com/contentauth/c2pa-rs/pull/171))
## 0.8.1
_25 March 2024_

* use c2pa-rs 0.31.1 for actions.changes support ([#170](https://github.com/contentauth/c2pa-rs/pull/170))
## 0.8.0
_20 March 2024_

* (MINOR) allow clients to sign with a process outside of c2patool ([#169](https://github.com/contentauth/c2pa-rs/pull/169))
* Add trust and verification options to c2pa_tool ([#168](https://github.com/contentauth/c2pa-rs/pull/168))
* adds version to c2patool artifact names ([#158](https://github.com/contentauth/c2pa-rs/pull/158))
## 0.7.0
_22 November 2023_

* (MINOR) updates to c2pa-rs v0.28.2 ([#153](https://github.com/contentauth/c2pa-rs/pull/153))
* Update to c2pa-rs 0.28.1
## 0.6.2
_05 October 2023_

* update to c2pa 0.27.1 ([#146](https://github.com/contentauth/c2pa-rs/pull/146))
* Merge branch 'main' of https://github.com/contentauth/c2patool
* Add Do not train example
* Upgrade to c2pa-rs 0.26.0 ([#143](https://github.com/contentauth/c2pa-rs/pull/143))
* Fix issue with docusaurus styling and fix broken links ([#138](https://github.com/contentauth/c2pa-rs/pull/138))
* Updates to c2pa-rs 0.25.1 ([#128](https://github.com/contentauth/c2pa-rs/pull/128))
* Fix windows release ([#132](https://github.com/contentauth/c2pa-rs/pull/132))
## 0.6.1
_24 July 2023_

* use compress-archive instead of tar ([#130](https://github.com/contentauth/c2pa-rs/pull/130))

## 0.6.0
_22 June 2023_

* (MINOR) update to c2pa-rs 0.24.0 ([#127](https://github.com/contentauth/c2pa-rs/pull/127))

## 0.5.4
_13 June 2023_

* integrate c2pa 23.0 bump version ([#126](https://github.com/contentauth/c2pa-rs/pull/126))
* Merge branch 'main' of https://github.com/contentauth/c2patool
* c2pa-rs 23.0 + updated test
* Update README.md ([#124](https://github.com/contentauth/c2pa-rs/pull/124))

## 0.5.3
_04 May 2023_

* Parent Ingredient JSON ([#123](https://github.com/contentauth/c2pa-rs/pull/123))

## 0.5.2
_19 April 2023_

* Ingredient thumbnails, extension cleanup, toolkit update ([#120](https://github.com/contentauth/c2pa-rs/pull/120))

## 0.5.1
_10 April 2023_

* Update README.md ([#118](https://github.com/contentauth/c2pa-rs/pull/118))
* Update expired sample certs ([#113](https://github.com/contentauth/c2pa-rs/pull/113))

## 0.5.0
_28 March 2023_

* (MINOR) New ingredient support and c2pa file formats ([#111](https://github.com/contentauth/c2pa-rs/pull/111))
* Leverage new Manifest & Ingredient, add Ingredient creation. ([#107](https://github.com/contentauth/c2pa-rs/pull/107))

## 0.4.0
_01 March 2023_

* (MINOR) Add --certs and --tree options ([#106](https://github.com/contentauth/c2pa-rs/pull/106))
* update to cp2pa 0.17.0 ([#105](https://github.com/contentauth/c2pa-rs/pull/105))
* Update for Clippy in Rust 1.67 ([#101](https://github.com/contentauth/c2pa-rs/pull/101))

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
