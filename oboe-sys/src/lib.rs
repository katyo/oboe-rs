#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(all(not(target_os = "android"), not(test), not(feature = "rustdoc")))]
compile_error!("Currently oboe-sys only supports Android platform");

#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(not(feature = "generate-bindings"), any(target_os = "android", test), target_arch = "arm"))]
include!("bindings_armv7.rs");

#[cfg(all(not(feature = "generate-bindings"), any(target_os = "android", test), target_arch = "aarch64"))]
include!("bindings_aarch64.rs");

#[cfg(all(not(feature = "generate-bindings"), any(target_os = "android", test), target_arch = "x86"))]
include!("bindings_i686.rs");

#[cfg(any(feature = "rustdoc", all(not(feature = "generate-bindings"), any(target_os = "android", test), target_arch = "x86_64")))]
include!("bindings_x86_64.rs");
