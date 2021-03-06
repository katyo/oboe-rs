/*!
# Raw (unsafe) bindings for Oboe library

[![github](https://img.shields.io/badge/github-katyo/oboe--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/oboe-rs)
[![Crates.io Package](https://img.shields.io/crates/v/oboe-sys.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/oboe-sys)
[![Docs.rs API Docs](https://img.shields.io/badge/docs.rs-oboe--sys-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/oboe-sys)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/Apache-2.0)
[![CI Status](https://img.shields.io/github/workflow/status/katyo/oboe-rs/Rust?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/oboe-rs/actions?query=workflow%3ARust)

[Oboe](https://github.com/google/oboe) is a C++ library which makes it easy to build high-performance audio apps on Android. It was created primarily to allow developers to target a simplified API that works across multiple API levels back to API level 16 (Jelly Bean).

Usually you shouldn't use this crate directly, instead use [oboe](https://crates.io/crates/oboe) crate which provides safe interface.
*/

#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    deref_nullptr, // TODO: Remove after closing https://github.com/rust-lang/rust-bindgen/issues/1651
    clippy::redundant_static_lifetimes, // TODO: Remove after that this issue will be fixed in bindgen
    clippy::missing_safety_doc
)]

#[cfg(all(not(target_os = "android"), not(feature = "test")))]
compile_error!("Currently oboe-sys only supports Android platform");

#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(
    not(feature = "generate-bindings"),
    any(target_os = "android", test),
    target_arch = "arm"
))]
include!("bindings_armv7.rs");

#[cfg(all(
    not(feature = "generate-bindings"),
    any(target_os = "android", test),
    target_arch = "aarch64"
))]
include!("bindings_aarch64.rs");

#[cfg(all(
    not(feature = "generate-bindings"),
    any(target_os = "android", test),
    target_arch = "x86"
))]
include!("bindings_i686.rs");

#[cfg(all(
    not(feature = "generate-bindings"),
    any(target_os = "android", test),
    target_arch = "x86_64"
))]
include!("bindings_x86_64.rs");
