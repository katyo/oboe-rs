# Safe bindings for **oboe** library

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-brightgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Package](https://img.shields.io/crates/v/oboe.svg?style=popout)](https://crates.io/crates/oboe)
[![Docs.rs API Documentation](https://docs.rs/oboe/badge.svg)](https://docs.rs/oboe)

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
