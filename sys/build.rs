#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    // Skip build on docs.rs and CI
    if matches!(env::var("DOCS_RS"), Ok(s) if s == "1") {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo."));
    let src_dir = Path::new("oboe");
    let ext_dir = Path::new("oboe-ext");

    let target = env::var("TARGET").expect("TARGET is set by cargo.");
    let profile = env::var("PROFILE").expect("PROFILE is set by cargo.");

    let builder = Builder::new(
        "oboe",
        env!("CARGO_PKG_VERSION"),
        target,
        profile,
        "https://github.com/katyo/{package}-rs/releases/download/{version}/lib{package}-ext_{target}_{profile}.tar.gz",
        &out_dir,
        src_dir,
        ext_dir,
    );

    builder.bindings();
    builder.library();

    add_libdir(builder.lib_dir);

    /*if cfg!(feature = "shared-stdcxx") {
        add_lib("c++_shared", false);
    } else {
        add_lib("c++_static", false);
    }*/

    add_lib("oboe-ext", !cfg!(feature = "shared-link"));

    add_lib("log", false);
    add_lib("OpenSLES", false);
}

struct Builder {
    pub src_dir: PathBuf,

    pub lib_url: String,
    pub lib_dir: PathBuf,

    pub ext_dir: PathBuf,
    pub bind_file: PathBuf,

    pub target: String,
    pub profile: String,
}

impl Builder {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        package: impl AsRef<str>,
        version: impl AsRef<str>,

        target: impl AsRef<str>,
        profile: impl AsRef<str>,

        lib_url: impl AsRef<str>,

        out_dir: impl AsRef<Path>,
        src_dir: impl AsRef<Path>,
        ext_dir: impl AsRef<Path>,
    ) -> Self {
        let package = package.as_ref();
        let version = version.as_ref();
        let profile = profile.as_ref();
        let target = target.as_ref();

        let lib_url = lib_url
            .as_ref()
            .replace("{package}", package)
            .replace("{version}", version)
            .replace("{target}", target)
            .replace("{profile}", profile);

        let out_dir = out_dir.as_ref();
        let lib_dir = out_dir.join("library");

        let src_dir = src_dir.as_ref().into();
        let ext_dir = ext_dir.as_ref().into();

        let bind_file = out_dir.join("bindings.rs");

        let target = target.into();
        let profile = profile.into();

        Self {
            src_dir,

            lib_url,
            lib_dir,

            ext_dir,
            bind_file,

            target,
            profile,
        }
    }

    #[cfg(not(feature = "generate-bindings"))]
    pub fn bindings(&self) {}

    #[cfg(feature = "generate-bindings")]
    pub fn bindings(&self) {
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
            .allowlist_type("oboe::ChannelCount")
            .allowlist_type("oboe::AudioStreamBase")
            .allowlist_type("oboe::AudioStream")
            .allowlist_type("oboe::AudioStreamBuilder")
            .allowlist_type("oboe::LatencyTuner")
            .allowlist_type("oboe::AudioStreamCallbackWrapper")
            .allowlist_type("oboe::StabilizedCallback")
            .allowlist_type("oboe::DefaultStreamValues")
            .allowlist_type("oboe::Version")
            .allowlist_function("oboe::AudioStreamBuilder_.+")
            .allowlist_function("oboe::AudioStream_.+")
            .allowlist_function("oboe::AudioStreamCallbackWrapper_.+")
            .allowlist_function("oboe::getSdkVersion")
            .blocklist_type("std::.*_ptr.*")
            .blocklist_type("oboe::ManagedStream")
            .blocklist_function("oboe::AudioStreamBuilder_openStream")
            .blocklist_function("oboe::AudioStreamBuilder_openStream1")
            .blocklist_function("oboe::AudioStreamBuilder_openManagedStream")
            .blocklist_function("oboe::AudioStreamBuilder_setPackageName")
            .blocklist_function("oboe::AudioStreamBuilder_setAttributionTag")
            .opaque_type("std::.*")
            .opaque_type("oboe::AudioStream")
            .opaque_type("oboe::AudioStreamBuilder")
            .opaque_type("oboe::LatencyTuner")
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(&self.bind_file)
            .expect("Couldn't write bindings!");
    }

    #[cfg(feature = "test")]
    pub fn library(&self) {}

    #[cfg(all(not(feature = "test"), feature = "fetch-prebuilt"))]
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

    #[cfg(all(not(feature = "test"), not(feature = "fetch-prebuilt")))]
    pub fn library(&self) {
        let src_files = &[
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
            "common/SourceI24Caller.cpp",
            "common/SourceI32Caller.cpp",
            "common/Utilities.cpp",
            "common/QuirksManager.cpp",
            "fifo/FifoBuffer.cpp",
            "fifo/FifoController.cpp",
            "fifo/FifoControllerBase.cpp",
            "fifo/FifoControllerIndirect.cpp",
            "flowgraph/FlowGraphNode.cpp",
            "flowgraph/ChannelCountConverter.cpp",
            "flowgraph/ClipToRange.cpp",
            "flowgraph/ManyToMultiConverter.cpp",
            "flowgraph/MonoToMultiConverter.cpp",
            "flowgraph/MultiToMonoConverter.cpp",
            "flowgraph/RampLinear.cpp",
            "flowgraph/SampleRateConverter.cpp",
            "flowgraph/SinkFloat.cpp",
            "flowgraph/SinkI16.cpp",
            "flowgraph/SinkI24.cpp",
            "flowgraph/SinkI32.cpp",
            "flowgraph/SourceFloat.cpp",
            "flowgraph/SourceI16.cpp",
            "flowgraph/SourceI24.cpp",
            "flowgraph/SourceI32.cpp",
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

        let ext_files = &[
            "AudioStreamWrapper.cpp",
            "AudioStreamBuilderWrapper.cpp",
            "AudioStreamCallbackWrapper.cpp",
        ];

        if env::var(format!("CXX_{}", self.target)).is_err() {
            if let Ok(cc) = env::var(format!("CC_{}", self.target)) {
                env::set_var(
                    format!("CXX_{}", self.target),
                    cc.replace("clang", "clang++"),
                );
            }
        }

        let mut library = cc::Build::new();

        library.cpp(true);
        library.cpp_link_stdlib(if cfg!(feature = "shared-stdcxx") {
            "c++_shared"
        } else {
            "c++_static"
        });
        for flag in &[
            "-std=c++17",
            "-Wall",
            "-Wextra-semi",
            "-Wshadow",
            "-Wshadow-field",
            "-fno-rtti",
            "-fno-exceptions",
            //"-Ofast",
        ] {
            library.flag(flag);
        }
        if self.profile == "debug" {
            //library.flag("-Werror");
            library.define("OBOE_ENABLE_LOGGING", "1");
        }

        library.static_flag(!cfg!(feature = "shared-link"));
        library.shared_flag(cfg!(feature = "shared-link"));

        library.include(self.src_dir.join("include"));
        library.include(self.src_dir.join("src"));
        library.include(self.ext_dir.join("include"));
        library.include(self.ext_dir.join("src"));

        for file in src_files {
            library.file(self.src_dir.join("src").join(file));
        }
        for file in ext_files {
            library.file(self.ext_dir.join("src").join(file));
        }

        library.out_dir(&self.lib_dir);
        library.compile("oboe-ext");
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
