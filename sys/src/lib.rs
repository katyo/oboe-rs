#![doc = include_str!("../README.md")]
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
