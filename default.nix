{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  androidComposition = androidenv.composeAndroidPackages {
    #toolsVersion = "25.2.5";
    platformToolsVersion = "33.0.3";
    buildToolsVersions = [ "30.0.3" ];
    #includeEmulator = true;
    #emulatorVersion = "27.2.0";
    platformVersions = [ "16" "28" ];
    #includeSources = false;
    #includeDocs = false;
    includeSystemImages = false;
    systemImageTypes = [ "default" ];
    abiVersions = [ "x86" "x86_64" "armeabi-v7a" "arm64-v8a" ];
    #lldbVersions = [ "2.0.2558144" ];
    #cmakeVersions = [ "3.6.4111459" ];
    includeNDK = true;
    #ndkVersion = "21.0.6113669";
    #ndkVersion = "21.3.6528147";
    ndkVersion = "21.4.7075529";
    useGoogleAPIs = false;
    useGoogleTVAddOns = false;
    includeExtras = [
      #"extras;google;gcm"
    ];
  };
  androidsdk = androidComposition.androidsdk;
  sdk_root = "${androidsdk}/libexec/android-sdk";
  ndk_root = "${sdk_root}/ndk-bundle";
  ndk_path = "${ndk_root}/toolchains/llvm/prebuilt/linux-x86_64/bin";
in mkShell rec {
  # cargo apk
  ANDROID_SDK_ROOT = "${sdk_root}";
  ANDROID_NDK_ROOT = "${ndk_root}";

  # cargo ndk 
  ANDROID_SDK_HOME = "${sdk_root}";

  # llvm-config for libclang
  ##PATH = "${ndk_path}:${builtins.getEnv "PATH"}";
  shellHook = ''
    export PATH="${ndk_path}:$PATH";
  '';

  # reduce resources usage
  DART_VM_OPTIONS = "--old_gen_heap_size=256 --observe";
  GRADLE_OPTS = "-Xmx64m -Dorg.gradle.jvmargs='-Xmx256m -XX:MaxPermSize=64m'";

  buildInputs = [ pkgconfig openssl zlib ncurses5 cmake libssh2 libgit2 ];
}
