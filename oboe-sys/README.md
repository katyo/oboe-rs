# Raw (unsafe) bindings for **oboe** library

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-brightgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Package](https://img.shields.io/crates/v/oboe-sys.svg?style=popout)](https://crates.io/crates/oboe-sys)
[![Docs.rs API Docs](https://docs.rs/oboe-sys/badge.svg)](https://docs.rs/oboe-sys)

[Oboe](https://github.com/google/oboe) is a C++ library which makes it easy to build high-performance audio apps on Android. It was created primarily to allow developers to target a simplified API that works across multiple API levels back to API level 16 (Jelly Bean).

Usually you shouldn't use this crate directly, instead use [oboe](https://crates.io/crates/oboe) crate which provides safe interface.
