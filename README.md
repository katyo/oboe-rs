# Safe bindings for **oboe** library

[![github](https://img.shields.io/badge/github-katyo/oboe--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/oboe-rs)
[![Crates.io Package](https://img.shields.io/crates/v/oboe.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/oboe)
[![Docs.rs API Docs](https://img.shields.io/badge/docs.rs-oboe-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/oboe)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/Apache-2.0)
[![CI Status](https://img.shields.io/github/workflow/status/katyo/oboe-rs/Rust?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/oboe-rs/actions?query=workflow%3ARust)

[Oboe](https://github.com/google/oboe) is a C++ library which makes it easy to build high-performance audio apps on Android. It was created primarily to allow developers to target a simplified API that works across multiple API levels back to API level 16 (Jelly Bean).

## Crates structure

* _oboe-sys_ raw unsafe bindings to library which generated using [bindgen](https://crates.io/crates/bindgen).
* _oboe_ safe rust wrapper for library which intended to use for android applications
* _oboe-demo_ simple usage example for library (incomplete)

## Crate features

The *oboe* and *oboe-sys* already have pregenerated bindings
and precompiled library for the following Android targets:

* _armv7_
* _aarch64_
* _i686_
* _x86_64_

In case when you want to generate bindings and/or compile library
youself you can use features below:

* _generate-bindings_ which runs bindgen to generate bindings
* _compile-library_ which runs cmake to compile _oboe_ C++ library

## Build issues

The **[clang-sys](https://crates.io/crates/clang-sys)** crate uses
**[llvm-config](http://llvm.org/docs/CommandGuide/llvm-config.html)**
for searching [libclang](https://clang.llvm.org/docs/Tooling.html)
library and preparing _C_/_C++_ compiler configuration.
In order to get proper setup you should add *llvm-config* to your
executables search path.

In case of using tools with libclang under the hood like __bindgen__
you must be sure in proper your setup. Otherwise you get an errors
related to missing headers or definitions.

To build applications you need recent version of __cargo-apk__ from git,
which supports latest Android [SDK](https://developer.android.com/studio#command-tools) (28+) and [NDK](https://developer.android.com/ndk) (20+).
Don't forget to set ANDROID_HOME and NDK_HOME environment variables with paths to installed SDK and NDK.

For building host crates which requires _C_-compiler you usually should
also set __HOST_CC__ environment variable with path to your _C_-compiler.

## Usage example

Playing sine wave in asynchronous (callback-driven) mode:

```rust
use oboe::{
    AudioOutputCallback,
    AudioOutputStream,
    AudioStreamBuilder,
    DataCallbackResult,
    PerformanceMode,
    SharingMode,
    Mono,
};

// Structure for sound generator
pub struct SineWave {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: Option<f32>,
}

// Default constructor for sound generator
impl Default for SineWave {
    fn default() -> Self {
        Self {
            frequency: 440.0,
            gain: 0.5,
            phase: 0.0,
            delta: None,
        }
    }
}

// Audio output callback trait implementation
impl AudioOutputCallback for SineWave {
    // Define type for frames which we would like to process
    type FrameType = (f32, Mono);

    // Implement sound data output callback
    fn on_audio_ready(&mut self, stream: &mut dyn AudioOutputStream, frames: &mut [f32]) -> DataCallbackResult {
        // Configure out wave generator
        if self.delta.is_none() {
            let sample_rate = stream.get_sample_rate() as f32;
            self.delta = (self.frequency2.0PI / sample_rate).into();
            println!("Prepare sine wave generator: samplerate={}, time delta={}", sample_rate, self.delta.unwrap());
        }

        let delta = self.delta.unwrap();

        // Generate audio frames to fill the output buffer
        for frame in frames {
            *frame = self.gainself.phase.sin();
            self.phase += delta;
            while self.phase > 2.0PI {
                self.phase -= 2.0PI;
            }
        }

        // Notify the oboe that stream is continued
        DataCallbackResult::Continue
    }
}

// ...

// Create playback stream
let mut sine = AudioStreamBuilder::default()
    // select desired performance mode
    .set_performance_mode(PerformanceMode::LowLatency)
    // select desired sharing mode
    .set_sharing_mode(SharingMode::Shared)
    // select sound sample format
    .set_format::<f32>()
    // select channels configuration
    .set_channel_count::<Mono>()
    // set our generator as callback
    .set_callback(SineWave::default())
    // open the output stream
    .open_stream()
    .unwrap();

// Start playback
sine.start().unwrap();

// ...
```
