{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let toolchain_path = "toolchains/llvm/prebuilt/linux-x86_64";
in stdenv.mkDerivation rec {
  name = "oboe";

  ANDROID_HOME = "${builtins.getEnv "HOME"}/.androidenv";
  NDK_HOME = "${ANDROID_HOME}/ndk/20.1.5948944";
  ANDROID_NDK = NDK_HOME;

  LD_LIBRARY_PATH = "${zlib}/lib:${ncurses5}/lib";

  PATH = "${NDK_HOME}/${toolchain_path}/bin:${builtins.getEnv "PATH"}";

  HOST_CC = "${clang}/bin/clang";

  buildInputs = [
    pkgconfig
    zlib
    ncurses5
    openssl
    cmake
  ];
}
