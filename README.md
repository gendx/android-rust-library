# Android Rust example application

This repository is a demonstration of an Android app integrating a native Rust library.

You can find more information in the following blog post: [Compiling Rust libraries for Android apps: a deep dive](https://gendignoux.com/blog/2022/10/24/rust-library-android.html).

## Usage

To build a Docker container containing all the build tools and the demo application, run:

```bash
$ sudo ./docker-build.sh
```

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
- `script-rust-default.sh` and `script-rust-default-nostrip.sh`: using the default Rust toolchain (stable).

Then, multiple ways are provided to build the Java part of the Android app.

- `script-java.sh`: normal build of the Android app.
- `script-java-incorrect-arm64.sh`: Android app built with the ARM-64 library in the wrong folder.
- `script-java-incorrect-x86_64.sh`: Android app built with the x86_64 library in the wrong folder.

You can then spawn an Android emulator with `emulator.sh`, and use the `launch-app-debug.sh` or `launch-app-release.sh` scripts to install+launch the application via ADB to either the emulator or a real device connected via USB.
