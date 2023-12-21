# Android Rust example application with SIMD instructions

This repository is a demonstration of an Android app integrating a native Rust library.
Additionally, this Rust library demonstrates detecting support for SIMD instructions.

You can find more information in the following blog posts:

- [Compiling Rust libraries for Android apps: a deep dive](https://gendignoux.com/blog/2022/10/24/rust-library-android.html),
- [Detecting SIMD support on ARM with Android (and patching the Rust compiler for it)](https://gendignoux.com/blog/2022/11/09/rust-simd-detect-arm-android.html),
- [Testing SIMD instructions on ARM with Rust on Android](https://gendignoux.com/blog/2023/01/05/rust-arm-simd-android.html).

## Usage

To build a Docker container containing all the build tools and the demo application, run:

```bash
$ sudo ./docker-build.sh
```

### Building and running the demo Android application

You can then launch this Docker container to build the application.
This comes in various configurations:

- `docker-run-basic.sh`: minimal setup to compile the application within the container.
- `docker-run-emulator.sh`: besides compiling the application, this setup allows to spawn an Android emulator within the container to test this application.
- `docker-run-usb.sh`: besides compiling the application, this setup gives the container access to the USB devices, so that you can install the application on a real device via ADB.
- `docker-run-all.sh`: setup with everything available in the container.

```bash
$ sudo ./docker-run-basic.sh
$ ./script-rust.sh && ./script-java.sh
```

You can build various flavors of the Rust library.

- `script-rust-nightly.sh`: basic script using the nightly Rust toolchain, with stripping of debug symbols enabled.
- `script-rust-nightly-nostrip.sh`: same, but without stripping debug symbols.
- `script-rust-stage1.sh` and `script-rust-stage1-nostrip.sh`: using a locally built `stage1` Rust compiler (see below).
- `script-rust-default.sh` and `script-rust-default-nostrip.sh`: using the default Rust toolchain (stable).
- `script-relinked.sh`: more advanced library, which bundles another library linked twice (see the corresponding [blog post](https://gendignoux.com/blog/2023/01/05/rust-arm-simd-android.html#mixing-dynamic-and-static-detection-re-linking-a-dependency)).

Then, multiple ways are provided to build the Java part of the Android app.

- `script-java.sh`: normal build of the Android app.
- `script-java-incorrect-arm64.sh`: Android app built with the ARM-64 library in the wrong folder.
- `script-java-incorrect-x86_64.sh`: Android app built with the x86_64 library in the wrong folder.

You can then spawn an Android emulator with `emulator.sh`, and use the `launch-app-debug.sh` or `launch-app-release.sh` scripts to install+launch the application via ADB to either the emulator or a real device connected via USB.

### Building and using a patched Rust compiler

This repository also shows how to patch and build the Rust compiler.
You can launch the Docker container in various scenarios:

- `docker-run-rustc-1cpu.sh`: with 1 CPU, 2 GB of RAM,
- `docker-run-rustc-2cpu.sh`: with 2 CPUs, 2 GB of RAM,
- `docker-run-rustc-4cpu.sh`: with 4 CPUs, 3 GB of RAM,
- `docker-run-rustc-8cpu.sh`: with 8 CPUs, 5 GB of RAM,
- `docker-run-all-rustc.sh`: to compile `rustc` + build the app.

Once launched, you can clone and compile `rustc` for Android with:

- `scripts/clone-rustlang-head.sh`: clone the latest commit on the Rust repository,
- `scripts/stage0.sh`: build a stage 0 compiler,
- `scripts/stage1.sh`: build a stage 1 compiler.

You'll also find the following tool to generate a flame graph of the disk space used by a build of `rustc`:

- `tools/flamedisk`: small Rust tool to generate an input suitable for `flamegraph.pl`,
- `scripts/flamedisk.sh`: driver script to generate the flame graph.

### Running unit tests and benchmarks on an Android device

This repository shows how to run Rust unit tests and benchmarks directly on an attached Android device (physical device via USB or emulator), without using a full Android application.
The `android-runner.sh` script tells Cargo how to do that.

Within the Docker container, you can run the following benchmarks:

- `bench.sh`: demo application, located in `src/android-simd`,
- `bench-haraka.sh`: my implementation of the Haraka hash function (https://github.com/gendx/haraka-rs),
- `bench-horcrux.sh`: my implementation of Shamir's Secret Sharing (https://github.com/gendx/horcrux).
