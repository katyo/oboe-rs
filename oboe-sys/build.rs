#[cfg(feature = "generate-bindings")]
use bindgen;

#[cfg(feature = "compile-library")]
use cmake;

use std::env;

#[cfg(feature = "generate-bindings")]
use std::path::PathBuf;

fn main() {
    if !env::var("CARGO_FEATURE_RUSTDOC").is_ok() {
        #[cfg(feature = "compile-library")]
        {
            let library = cmake::Config::new("oboe-ext")
                .always_configure(true)
                .build();
            let library_out = library.display();

            println!("cargo:rustc-link-search=native={}/build", library_out);
            println!("cargo:rustc-link-search=native={}/build/oboe", library_out);
            println!("cargo:rustc-link-lib=static=oboe");
            println!("cargo:rustc-link-lib=static=oboe-ext");
        }

        #[cfg(not(feature = "compile-library"))]
        {
            let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
                .expect("CARGO_CFG_TARGET_ARCH is set by cargo.");

            let library_path = env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR is set by cargo.");

            let library_arch = rustc_target(&target_arch);

            println!("cargo:rustc-link-search=native={}/lib", library_path);
            println!("cargo:rustc-link-lib=static=oboe_{}", library_arch);
        }

        #[cfg(feature = "generate-bindings")]
        generate_bindings();
    }
}

#[cfg(not(feature = "compile-library"))]
fn rustc_target<S: AsRef<str>>(target_arch: &S) -> &'static str {
    match target_arch.as_ref() {
        "arm" => "armv7",
        "aarch64" => "aarch64",
        "x86" => "i686",
        "x86_64" => "x86_64",
        arch => panic!("Unsupported architecture {}", arch),
    }
}

#[cfg(feature = "generate-bindings")]
fn android_target<S: AsRef<str>>(target_arch: &S) -> &'static str {
    match target_arch.as_ref() {
        "arm" => "arm-linux-androideabi",
        "aarch64" => "aarch64-linux-android",
        "x86" => "i686-linux-android",
        "x86_64" => "x86_64-linux-android",
        arch => panic!("Unsupported architecture {}", arch),
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .expect("CARGO_CFG_TARGET_OS is set by cargo.");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH is set by cargo.");

    let mut clang_args = Vec::new();

    if target_os == "android" {
        let ndk_target = android_target(&target_arch);

        clang_args.push(format!("--target={}", ndk_target));
    }

    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .clang_args(&clang_args)
        .clang_args(&[
            "-xc++",
            "-std=c++14",
            "-Ioboe-ext/include",
            "-Ioboe/include",
        ])
        .header("oboe-ext/include/wrapper.h")
        .opaque_type("std::*")
        .whitelist_type("oboe::AudioStreamBase")
        .whitelist_type("oboe::AudioStream")
        .whitelist_type("oboe::AudioStreamBuilder")
        .whitelist_type("oboe::LatencyTuner")
        .whitelist_type("oboe::AudioStreamCallbackWrapper")
        .whitelist_type("oboe::StabilizedCallback")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
