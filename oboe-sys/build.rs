#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    #[cfg(not(feature = "rustdoc"))]
    {
        let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo."));
        let ext_dir = Path::new("oboe-ext");

        let target = env::var("TARGET").expect("The TARGET is set by cargo.");
        let profile = env::var("PROFILE").expect("PROFILE is set by cargo.");

        let builder = Builder::new(
            "oboe",
            "10bb6fa",
            target,
            profile,
            "https://github.com/google/{package}/archive/{version}.tar.gz",
            "https://github.com/katyo/{package}-rs/releases/download/lib{package}-ext_0.1.2/{target}_{profile}.tar.gz",
            &out_dir,
            ext_dir,
        );

        builder.bindings();
        builder.library();

        add_libdir(out_dir);

        #[cfg(feature = "static-link")]
        {
            add_lib("log", true);
            add_lib("liboboe", true);
        }

        #[cfg(not(feature = "static-link"))]
        {
            add_lib("liboboe-ext", false);
        }

        add_lib("OpenSLES", false);
    }
}

struct Builder {
    pub src_url: String,
    pub src_dir: PathBuf,

    pub lib_url: String,
    pub lib_dir: PathBuf,

    pub ext_dir: PathBuf,
    pub bind_file: PathBuf,

    pub target: String,
    pub profile: String,
}

impl Builder {
    pub fn new(
        package: impl AsRef<str>,
        version: impl AsRef<str>,

        target: impl AsRef<str>,
        profile: impl AsRef<str>,

        src_url: impl AsRef<str>,
        lib_url: impl AsRef<str>,

        out_dir: impl AsRef<Path>,
        ext_dir: impl AsRef<Path>,
    ) -> Self {
        let package = package.as_ref();
        let version = version.as_ref();
        let profile = profile.as_ref();
        let target = target.as_ref();

        let src_url = src_url
            .as_ref()
            .replace("{package}", package)
            .replace("{version}", version);
        let lib_url = lib_url
            .as_ref()
            .replace("{package}", package)
            .replace("{version}", version)
            .replace("{target}", target)
            .replace("{profile}", profile);

        let out_dir = out_dir.as_ref();

        let src_dir = out_dir.join(format!("{}-{}", package, version));
        let lib_dir = out_dir.into();

        let ext_dir = ext_dir.as_ref().into();

        let bind_file = out_dir.join("bindings.rs").into();

        let target = target.into();
        let profile = profile.into();

        Self {
            src_url,
            src_dir,

            lib_url,
            lib_dir,

            ext_dir,
            bind_file,

            target,
            profile,
        }
    }

    pub fn fetch(&self) {
        if self.src_dir.is_dir() {
            eprintln!(
                "Sources {} already fetched to {}",
                self.src_url,
                self.src_dir.display()
            );
        } else {
            eprintln!(
                "Fetching sources {} to {}",
                self.src_url,
                self.src_dir.display()
            );

            fetch_unroll::Fetch::from(&self.src_url)
                .unroll()
                .strip_components(1)
                .to(&self.src_dir)
                .expect("Sources should be fetched.");
        }
    }

    #[cfg(not(feature = "generate-bindings"))]
    pub fn bindings(&self) {}

    #[cfg(feature = "generate-bindings")]
    pub fn bindings(&self) {
        self.fetch();

        let target_os =
            env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS is set by cargo.");

        let target_arch =
            env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH is set by cargo.");

        let mut clang_args = Vec::new();

        if target_os == "android" {
            let ndk_target = android_target(&target_arch);

            clang_args.push(format!("--target={}", ndk_target));
        }

        let src_include = self.src_dir.join("include");
        let ext_include = self.ext_dir.join("include");

        let bindings = bindgen::Builder::default()
            .detect_include_paths(true)
            .clang_args(&clang_args)
            .clang_args(&["-xc++", "-std=c++14"])
            .clang_args(&[
                format!("-I{}", ext_include.display()),
                format!("-I{}", src_include.display()),
            ])
            .header(
                ext_include
                    .join("oboe")
                    .join("OboeExt.h")
                    .display()
                    .to_string(),
            )
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
            .blacklist_type("std.*")
            .blacklist_type("oboe::ManagedStream")
            .blacklist_function("std.*")
            .blacklist_function("oboe::AudioStreamBuilder_openStream1")
            .blacklist_function("oboe::AudioStreamBuilder_openManagedStream")
            .opaque_type("oboe::AudioStream")
            .opaque_type("oboe::AudioStreamBuilder")
            .opaque_type("oboe::LatencyTuner")
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(&self.bind_file)
            .expect("Couldn't write bindings!");
    }

    #[cfg(not(feature = "compile-library"))]
    pub fn library(&self) {
        if self.lib_dir.is_dir() {
            eprintln!(
                "Prebuilt library {} already fetched to {}",
                self.lib_url,
                self.lib_dir.display()
            );
        } else {
            eprintln!(
                "Fetching prebuilt library {} to {}",
                self.lib_url,
                self.lib_dir.display()
            );

            fetch_unroll::Fetch::from(&self.lib_url)
                .unroll()
                .to(&self.lib_dir)
                .expect("Prebuilt library should be fetched.");
        }
    }

    #[cfg(feature = "compile-library")]
    pub fn library(&self) {
        use std::fs::copy;

        self.fetch();

        if let Err(_) = env::var(format!("CXX_{}", self.target)) {
            if let Ok(cc) = env::var(format!("CC_{}", self.target)) {
                env::set_var(
                    format!("CXX_{}", self.target),
                    cc.replace("clang", "clang++"),
                );
            }
        }

        let library = cmake::Config::new(&self.ext_dir)
            .define("OBOE_DIR", &self.src_dir)
            .define(
                "BUILD_SHARED_LIBS",
                if cfg!(feature = "static-link") {
                    "0"
                } else {
                    "1"
                },
            )
            .define("CMAKE_C_COMPILER_WORKS", "1")
            .define("CMAKE_CXX_COMPILER_WORKS", "1")
            // TODO: Remove after merging https://github.com/google/oboe/pull/1020
            .cxxflag("-D__ANDROID_API_R__=30")
            .always_configure(true)
            .very_verbose(true)
            .build_target("all")
            .build();

        for lib in &["liboboe-ext.a", "liboboe-ext.so"] {
            let src = library.join("build").join(lib);
            let dst = self.lib_dir.join(lib);

            if src.is_file() {
                copy(src, dst).unwrap();
            }
        }
    }
}

fn android_target(target_arch: impl AsRef<str>) -> &'static str {
    match target_arch.as_ref() {
        "arm" => "arm-linux-androideabi",
        "aarch64" => "aarch64-linux-android",
        "x86" => "i686-linux-android",
        "x86_64" => "x86_64-linux-android",
        arch => panic!("Unsupported architecture {}", arch),
    }
}

fn rustc_target(target_arch: impl AsRef<str>) -> &'static str {
    match target_arch.as_ref() {
        "arm" => "armv7",
        "aarch64" => "aarch64",
        "x86" => "i686",
        "x86_64" => "x86_64",
        arch => panic!("Unsupported architecture {}", arch),
    }
}

fn add_lib(_name: impl AsRef<str>, _static: bool) {
    #[cfg(not(feature = "test"))]
    println!(
        "cargo:rustc-link-lib={}{}",
        if _static { "static=" } else { "" },
        _name.as_ref()
    );
}

fn add_libdir(_path: impl AsRef<Path>) {
    #[cfg(not(feature = "test"))]
    println!(
        "cargo:rustc-link-search=native={}",
        _path.as_ref().display()
    );
}
