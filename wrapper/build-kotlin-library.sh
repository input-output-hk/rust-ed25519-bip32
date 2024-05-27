#!/bin/bash

set -eo pipefail

cd `dirname $0`
trap "cd -" EXIT

OS_NAME=$(uname -s)
NAME="ed25519_bip32_wrapper"
LIBRARY_NAME="lib$NAME.a"
OUT_PATH="build"


if [[ "$OS_NAME" == "Linux" ]]; then
  NDKOSVariant="linux-x86_64"
  # Linux target

  # Cross build "x86_64-unknown-linux-gnu"
  if [[ -d "./target/x86_64-unknown-linux-gnu/release" ]]; then
    echo "Skipping x86_64-unknown-linux-gnu: already built"
  else
    echo "Building x86_64-unknown-linux-gnu: [cross build --release --target x86_64-unknown-linux-gnu]..."
    cargo install cross --git https://github.com/cross-rs/cross
    cross build --release --target x86_64-unknown-linux-gnu
  fi
  # Cargo build "aarch64-unknown-linux-gnu"
  if [[ -d "./target/aarch64-unknown-linux-gnu/release" ]]; then
    echo "Skipping aarch64-unknown-linux-gnu: already built"
  else
    echo "Building aarch64-unknown-linux-gnu: [cross build --release --target aarch64-unknown-linux-gnu]..."
    cargo install cross --git https://github.com/cross-rs/cross
    cross build --release --target aarch64-unknown-linux-gnu
  fi

elif [[ "$OS_NAME" == "Darwin" ]]; then
  NDKOSVariant="darwin-x86_64"

  # Linux target
  # Cross build "x86_64-unknown-linux-gnu"
  if [[ -d "./target/x86_64-unknown-linux-gnu/release" ]]; then
    echo "Skipping x86_64-unknown-linux-gnu: already built"
  else
    echo "Building x86_64-unknown-linux-gnu: [cargo build --release --target x86_64-unknown-linux-gnu]..."
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc
    cargo build --release --target x86_64-unknown-linux-gnu
  fi
  # Cargo build "aarch64-unknown-linux-gnu"
  if [[ -d "./target/aarch64-unknown-linux-gnu/release" ]]; then
    echo "Skipping aarch64-unknown-linux-gnu: already built"
  else
    echo "Building aarch64-unknown-linux-gnu: [cargo build --release --target aarch64-unknown-linux-gnu]..."
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
    cargo build --release --target aarch64-unknown-linux-gnu
  fi
fi


# Linux target

# Cross build "x86_64-unknown-linux-gnu"
if [[ -d "./target/x86_64-unknown-linux-gnu/release" ]]; then
  echo "Skipping x86_64-unknown-linux-gnu: already built"
else
  echo "Building x86_64-unknown-linux-gnu: [cross build --release --target x86_64-unknown-linux-gnu]..."
  cargo install cross --git https://github.com/cross-rs/cross
  cross build --release --target x86_64-unknown-linux-gnu
fi
# Cargo build "aarch64-unknown-linux-gnu"
if [[ -d "./target/aarch64-unknown-linux-gnu/release" ]]; then
  echo "Skipping aarch64-unknown-linux-gnu: already built"
else
  echo "Building aarch64-unknown-linux-gnu: [cross build --release --target aarch64-unknown-linux-gnu]..."
  cargo install cross --git https://github.com/cross-rs/cross
  cross build --release --target aarch64-unknown-linux-gnu
fi

# Apple
if [[ "$OS_NAME" == "Darwin" ]]; then
  AARCH64_APPLE_DARWIN_PATH="./target/aarch64-apple-darwin/release"
  X86_64_APPLE_DARWIN_PATH="./target/x86_64-apple-darwin/release"

  apple_targets=("aarch64-apple-ios" "aarch64-apple-ios-sim" "x86_64-apple-ios" "aarch64-apple-darwin" "x86_64-apple-darwin")
  # apple_targets=()

  # Build for apple targets
  for target in "${apple_targets[@]}"; do
    echo "Building for $target..."
    rustup target add $target
    cargo build --release --target $target
  done

  # Merge mac libraries with lipo
  mkdir -p $OUT_PATH/macos-native/static
  mkdir -p $OUT_PATH/macos-native/dynamic

  lipo -create $AARCH64_APPLE_DARWIN_PATH/lib$NAME.dylib \
                $X86_64_APPLE_DARWIN_PATH/lib$NAME.dylib \
        -output $OUT_PATH/macos-native/dynamic/lib$NAME.dylib


  lipo -create $AARCH64_APPLE_DARWIN_PATH/lib$NAME.a \
                $X86_64_APPLE_DARWIN_PATH/lib$NAME.a \
        -output $OUT_PATH/macos-native/static/lib$NAME.a
fi

# Android
android_targets=("aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android")
android_jni=("arm64-v8a" "armeabi-v7a" "x86" "x86_64")

# Cross build
# cargo install cross --git https://github.com/cross-rs/cross

# # Build for android targets
# for target in "${android_targets[@]}"; do
#   echo "Building for $target..."
#   cross build --release --target $target
# done


# Cargo NDK build
# Environment Variables
export ANDROID_SDK=$(echo $ANDROID_HOME)
export NDK=$(echo $ANDROID_NDK_HOME)
export TOOLCHAIN="$NDK/toolchains/llvm/prebuilt/$NDKOSVariant"
export AR="$TOOLCHAIN/bin/llvm-ar"
export LD="$TOOLCHAIN/bin/ld"
export RANLIB="$TOOLCHAIN/bin/llvm-ranlib"
export STRIP="$TOOLCHAIN/bin/llvm-strip"
export PATH="$PATH:$TOOLCHAIN:$AR:$LD:$RANLIB:$STRIP:$TOOLCHAIN/bin/"


# Build command
cargo install cargo-ndk

rustup target add ${android_targets[@]}

# Build for android targets
for target in "${android_targets[@]}"; do
  export CC="$TOOLCHAIN/bin/${target}21-clang"
  export CXX="$TOOLCHAIN/bin/${target}21-clang++"
  export PATH="$PATH:$CC:$CXX"

  if [[ -d "./target/$target/release" ]]; then
    echo "Skipping $target: already built"
  else
    echo "Building $target: [cargo ndk build --release --target $target]..."
    cargo ndk build --release --target $target
  fi
done

# Create JNI Libs folder
for key in "${!android_targets[@]}"; do
  mkdir -p $OUT_PATH/jniLibs/${android_jni[$key]}
  cp ./target/${android_targets[$key]}/release/lib$NAME.so $OUT_PATH/jniLibs/${android_jni[$key]}/lib$NAME.so || echo ""
  echo "${android_targets[$key]}: ${android_jni[$key]}"
done

# Generate wrapper
echo "Generating wrapper..."
mkdir -p $OUT_PATH
cargo install --bin uniffi-bindgen-kotlin-multiplatform uniffi_bindgen_kotlin_multiplatform@0.1.0
CURRENT_ARCH=$(rustc --version --verbose | grep host | cut -f2 -d' ')

uniffi-bindgen-kotlin-multiplatform --lib-file ./target/$CURRENT_ARCH/release/$LIBRARY_NAME --out-dir $OUT_PATH/generated ./ed25519_bip32.udl

echo "Completed"
