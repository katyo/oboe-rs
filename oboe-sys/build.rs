#[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
use git2::build::RepoBuilder;

#[cfg(feature = "generate-bindings")]
use bindgen;

#[cfg(all(feature = "compile-library", feature = "cmake"))]
use cmake;

use std::env;

#[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
use std::{
    path::{Path, PathBuf},
    fs::metadata,
};

enum LinkArg {
    SearchPath(String),
    StaticLib(String),
    SharedLib(String),
}

use self::LinkArg::*;

fn main() {
    if !env::var("CARGO_FEATURE_RUSTDOC").is_ok() {
        #[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
        let out_dir = PathBuf::from(
            env::var("OUT_DIR").expect("OUT_DIR is set by cargo.")
        );

        #[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
        let oboe_src = {
            let oboe_src = out_dir.join("oboe-src");

            if !metadata(oboe_src.join(".git"))
                .map(|meta| meta.is_dir())
                .unwrap_or(false) {
                    fetch_oboe(&oboe_src);
                }

            oboe_src
        };

        #[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
        let oboe_ext = Path::new("oboe-ext");

        // compiling oboe library and binding extensions
        #[cfg(feature = "compile-library")]
        let link_args = compile_library(&oboe_src, &oboe_ext);

        // select precompiled oboe library for specified target
        #[cfg(not(feature = "compile-library"))]
        let link_args = select_library();

        for link_arg in link_args {
            match link_arg {
                SearchPath(path) => println!("cargo:rustc-link-search=native={}", path),
                StaticLib(name) => println!("cargo:rustc-link-lib=static={}", name),
                SharedLib(name) => println!("cargo:rustc-link-lib={}", name),
            }
        }

        #[cfg(feature = "generate-bindings")]
        {
            let out_file = out_dir.join("bindings.rs");
            generate_bindings(&oboe_src, &oboe_ext, &out_file);
        }
    }
}

#[cfg(not(feature = "compile-library"))]
fn select_library() -> Vec<LinkArg> {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH is set by cargo.");

    let lib_path = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is set by cargo.");

    let lib_arch = rustc_target(&target_arch);

    let lib_conf = env::var("PROFILE")
        .expect("PROFILE is set by cargo.");

    let lib_name = "oboe-ext".into();

    vec![
        SearchPath(format!("{}/lib/{}/{}", lib_path, lib_conf, lib_arch)),
        if cfg!(feature = "static-link") { StaticLib(lib_name) } else { SharedLib(lib_name) },
    ]
}

#[cfg(feature = "compile-library")]
fn compile_library(oboe_src: &Path, oboe_ext: &Path) -> Vec<LinkArg> {
    let library = cmake::Config::new(oboe_ext)
        .define("OBOE_DIR", oboe_src)
        .define("BUILD_SHARED_LIBS", if cfg!(feature = "static-link") { "0" } else { "1" })
        .always_configure(true)
        .very_verbose(true)
        .build_target("all")
        .build();
    let lib_out = library.display();
    let lib_name = "oboe-ext".into();

    vec![
        SearchPath(format!("{}/build", lib_out)),
        if cfg!(feature = "static-link") { StaticLib(lib_name) } else { SharedLib(lib_name) },
    ]
}

#[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
fn fetch_oboe(out_dir: &Path) { // clonning oboe git repo
    let repo = "https://github.com/google/oboe";
    let version = "master";

    let url = env::var("OBOE_GIT_URL")
        .unwrap_or_else(|_| repo.into());
    let tag = env::var("OBOE_GIT_TAG")
        .unwrap_or_else(|_| version.into());

    eprintln!("git clone to: {}", &out_dir.display());

    let _repo = match RepoBuilder::new()
        .branch(&tag)
        .clone(&url, out_dir) {
            Ok(repo) => repo,
            Err(error) => panic!("Unable to fetch oboe library from git due to {}. url={} tag={}", error, url, tag),
        };
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
fn generate_bindings(oboe_src: &Path, oboe_ext: &Path, out_file: &Path) {
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .expect("CARGO_CFG_TARGET_OS is set by cargo.");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH is set by cargo.");

    let mut clang_args = Vec::new();

    if target_os == "android" {
        let ndk_target = android_target(&target_arch);

        clang_args.push(format!("--target={}", ndk_target));
    }

    let oboe_src_include = oboe_src.join("include");
    let oboe_ext_include = oboe_ext.join("include");

    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        .clang_args(&clang_args)
        .clang_args(&[
            "-xc++",
            "-std=c++14",
        ])
        .clang_args(&[
            format!("-I{}", oboe_ext_include.display()),
            format!("-I{}", oboe_src_include.display()),
        ])
        .header(oboe_ext_include.join("oboe").join("OboeExt.h").display().to_string())
        .whitelist_type("oboe::ChannelCount")
        .whitelist_type("oboe::AudioStreamBase")
        .whitelist_type("oboe::AudioStream")
        .whitelist_type("oboe::AudioStreamBuilder")
        .whitelist_type("oboe::LatencyTuner")
        .whitelist_type("oboe::AudioStreamCallbackWrapper")
        .whitelist_type("oboe::StabilizedCallback")
        .whitelist_type("oboe::DefaultStreamValues")
        .whitelist_type("oboe::Version")
        .whitelist_function("oboe::AudioStreamBuilder_.+")
        .whitelist_function("oboe::AudioStream_.+")
        .whitelist_function("oboe::AudioStreamCallbackWrapper_.+")
        .whitelist_function("oboe::getSdkVersion")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
