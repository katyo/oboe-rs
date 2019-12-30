#[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
use git2::build::RepoBuilder;

#[cfg(feature = "generate-bindings")]
use bindgen;

#[cfg(all(feature = "compile-library", feature = "cmake"))]
use cmake;

#[cfg(all(feature = "compile-library", feature = "cc"))]
use cc;

use std::env;

#[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
use std::{
    path::{Path, PathBuf},
    fs::metadata,
};

fn main() {
    if !env::var("CARGO_FEATURE_RUSTDOC").is_ok() {
        #[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
        let out_dir = PathBuf::from(
            env::var("OUT_DIR")
                .expect("OUT_DIR is set by cargo.")
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

        #[cfg(feature = "compile-library")]
        let (lib_dirs, libs) = { // compiling oboe library and binding extensions
            compile_library(&oboe_src, &oboe_ext)
        };

        #[cfg(not(feature = "compile-library"))]
        let (lib_dirs, libs) = {
            let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
                .expect("CARGO_CFG_TARGET_ARCH is set by cargo.");

            let lib_path = env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR is set by cargo.");

            let lib_arch = rustc_target(&target_arch);

            (
                &[format!("{}/lib", lib_path)],
                &[format!("oboe_{}", lib_arch), format!("oboe-ext_{}", lib_arch)]
            )
        };

        for lib_dir in lib_dirs {
            println!("cargo:rustc-link-search=native={}", lib_dir);
        }

        for lib in libs {
            println!("cargo:rustc-link-lib=static={}", lib);
        }

        let dylibs = &[
            "log",
            "OpenSLES",
        ];

        for dylib in dylibs {
            println!("cargo:rustc-link-lib={}", dylib);
        }

        #[cfg(feature = "generate-bindings")]
        {
            let out_file = out_dir.join("bindings.rs");

            generate_bindings(&oboe_src, &oboe_ext, &out_file);
        }
    }
}

#[cfg(all(feature = "compile-library", feature = "cmake"))]
fn compile_library(oboe_src: &Path, oboe_ext: &Path) -> (Vec<String>, Vec<String>) {
    let library = cmake::Config::new(oboe_ext)
        .define("OBOE_DIR", oboe_src)
        //.cflag("-fPIC")
        //.cxxflag("-fno-use-cxa-atexit")
        .always_configure(true)
        .very_verbose(true)
        .build();
    let lib_out = library.display();
    (
        vec![format!("{}/build", lib_out), format!("{}/build/oboe", lib_out)],
        vec!["oboe".into(), "oboe-ext".into()],
    )
}

#[cfg(all(feature = "compile-library", feature = "cc"))]
fn compile_library(oboe_src: &Path, oboe_ext: &Path) -> (Vec<String>, Vec<String>) {
    let oboe_include_dir = oboe_src.join("include");
    let oboe_ext_include_dir = oboe_ext.join("include");

    let oboe_source_dir = oboe_src.join("src");
    let oboe_ext_source_dir = oboe_ext.join("src");

    let oboe_sources = &[
        "aaudio/AAudioLoader.cpp",
        "aaudio/AudioStreamAAudio.cpp",
        "common/AudioSourceCaller.cpp",
        "common/AudioStream.cpp",
        "common/AudioStreamBuilder.cpp",
        "common/DataConversionFlowGraph.cpp",
        "common/FilterAudioStream.cpp",
        "common/FixedBlockAdapter.cpp",
        "common/FixedBlockReader.cpp",
        "common/FixedBlockWriter.cpp",
        "common/LatencyTuner.cpp",
        "common/SourceFloatCaller.cpp",
        "common/SourceI16Caller.cpp",
        "common/Utilities.cpp",
        "common/QuirksManager.cpp",
        "fifo/FifoBuffer.cpp",
        "fifo/FifoController.cpp",
        "fifo/FifoControllerBase.cpp",
        "fifo/FifoControllerIndirect.cpp",
        "flowgraph/FlowGraphNode.cpp",
        "flowgraph/ClipToRange.cpp",
        "flowgraph/ManyToMultiConverter.cpp",
        "flowgraph/MonoToMultiConverter.cpp",
        "flowgraph/RampLinear.cpp",
        "flowgraph/SampleRateConverter.cpp",
        "flowgraph/SinkFloat.cpp",
        "flowgraph/SinkI16.cpp",
        "flowgraph/SinkI24.cpp",
        "flowgraph/SourceFloat.cpp",
        "flowgraph/SourceI16.cpp",
        "flowgraph/SourceI24.cpp",
        "flowgraph/resampler/IntegerRatio.cpp",
        "flowgraph/resampler/LinearResampler.cpp",
        "flowgraph/resampler/MultiChannelResampler.cpp",
        "flowgraph/resampler/PolyphaseResampler.cpp",
        "flowgraph/resampler/PolyphaseResamplerMono.cpp",
        "flowgraph/resampler/PolyphaseResamplerStereo.cpp",
        "flowgraph/resampler/SincResampler.cpp",
        "flowgraph/resampler/SincResamplerStereo.cpp",
        "opensles/AudioInputStreamOpenSLES.cpp",
        "opensles/AudioOutputStreamOpenSLES.cpp",
        "opensles/AudioStreamBuffered.cpp",
        "opensles/AudioStreamOpenSLES.cpp",
        "opensles/EngineOpenSLES.cpp",
        "opensles/OpenSLESUtilities.cpp",
        "opensles/OutputMixerOpenSLES.cpp",
        "common/StabilizedCallback.cpp",
        "common/Trace.cpp",
        "common/Version.cpp",
    ];

    let oboe_ext_sources = &[
        "AudioStreamBuilderWrapper.cpp",
        "AudioStreamCallbackWrapper.cpp",
        "AudioStreamWrapper.cpp",
    ];

    let library = cc::Build::new()
        .cpp(true)
        .cpp_set_stdlib("c++")
        .cpp_link_stdlib("c++")
        .flag("-std=c++14")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .include(&oboe_ext_include_dir)
        .include(&oboe_ext_source_dir)
        .include(&oboe_include_dir)
        .include(&oboe_source_dir)
        .define("OBOE_ENABLE_LOGGING",
                if env::var("DEBUG").is_ok() {
                    "1"
                } else {
                    "0"
                })
        .warnings(true)
        //.extra_warnings(true)
        .flag("-Wextra-semi")
        .flag("-Wshadow")
        .flag("-Wshadow-field")
        .files(oboe_sources.iter().map(|file| {
            oboe_source_dir.join(file)
        }))
        .files(oboe_ext_sources.iter().map(|file| {
            oboe_ext_source_dir.join(file)
        }))
        .compile("oboe-ext");

    (
        vec![],
        vec![],
    )
}

#[cfg(any(feature = "generate-bindings", feature = "compile-library"))]
fn fetch_oboe(out_dir: &Path) { // clonning oboe git repo
    //let repo = "https://github.com/google/oboe";
    let repo = "https://github.com/katyo/oboe";
    //let version = "1.3-stable";
    let version = "avoid-global-constant-vector";

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
        //.opaque_type("std::*")
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
        .whitelist_function("oboe::getSdkVersion")
        //.blacklist_type("std_.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
