# Bindings for **oboe** library

[Oboe](https://github.com/google/oboe) is a C++ library which makes it easy to build high-performance audio apps on Android. It was created primarily to allow developers to target a simplified API that works across multiple API levels back to API level 16 (Jelly Bean).

## Crates structure

* _oboe-sys_ raw unsafe bindings to library which generated using [bindgen](https://crates.io/crates/bindgen).
* _oboe_ safe rust wrapper for library which intended to use for android applications
* _oboe-demo_ simple usage example for library (incomplete)

## Crate features

The *oboe* and *oboe-sys* already have pregenerated bindings
and precompiled static libraries for four android binary
architectures:

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

In case of using tools with libclang under the hood like bindgen
you must be sure in proper your setup. Otherwise you get an errors
related to missing headers or definitions.
