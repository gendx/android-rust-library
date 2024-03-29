FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y \
        wget \
        unzip \
        build-essential \
        python3 \
        curl \
        cmake \
        git \
        openjdk-11-jdk-headless \
        libgl1-mesa-dri \
        libgl1-mesa-glx \
        libpulse0 \
        libasound2 \
        libxcomposite1 \
        libxcursor1 \
        libxdamage1 \
        libxi6 \
        libxtst6 \
        --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --uid 1000 --create-home --shell /bin/bash dev
# For some reason the host adds the USB device to the 46(plugdev) group. Being
# member of this group is enough to have rw access to the USB device and run
# adb.
RUN groupadd --gid 106 --system kvm \
    && usermod --groups 46,106 dev

USER dev
WORKDIR "/home/dev"

# Install Gradle manually. The Debian package is too outdated.
ENV GRADLE_ROOT=/home/dev/opt/gradle

RUN mkdir -p ${GRADLE_ROOT}
RUN wget -nv https://services.gradle.org/distributions/gradle-7.5.1-bin.zip -O gradle-7.5.1-bin.zip \
    && sha256sum gradle-7.5.1-bin.zip \
    && echo "f6b8596b10cce501591e92f229816aa4046424f3b24d771751b06779d58c8ec4  gradle-7.5.1-bin.zip" | sha256sum -c - \
    && unzip gradle-7.5.1-bin.zip -d ${GRADLE_ROOT} \
    && rm gradle-7.5.1-bin.zip

ENV PATH=${PATH}:${GRADLE_ROOT}/gradle-7.5.1/bin

# Set the ${ANDROID_HOME} variable, so that the tools can find our installation.
# See https://developer.android.com/studio/command-line/variables#envar.
ENV ANDROID_HOME=/home/dev/opt/android-sdk

RUN mkdir -p ${ANDROID_HOME}
RUN wget -nv https://dl.google.com/android/repository/commandlinetools-linux-8512546_latest.zip \
        -O ${HOME}/commandlinetools-linux-8512546_latest.zip \
    && sha256sum commandlinetools-linux-8512546_latest.zip \
    && echo "2ccbda4302db862a28ada25aa7425d99dce9462046003c1714b059b5c47970d8 commandlinetools-linux-8512546_latest.zip" | sha256sum -c - \
    && unzip commandlinetools-linux-8512546_latest.zip -d ${ANDROID_HOME}/cmdline-tools \
    && rm commandlinetools-linux-8512546_latest.zip

ENV PATH=${PATH}:${ANDROID_HOME}/cmdline-tools/cmdline-tools/bin:${ANDROID_HOME}/platform-tools

# Given the following configuration in `build.gradle`:
#   classpath 'com.android.tools.build:gradle:7.3.0'
# the build tools version to use is 30.0.3.
# See https://mvnrepository.com/artifact/com.android.tools.build/gradle/7.3.0.
RUN yes | sdkmanager --licenses \
    && sdkmanager --list \
    && sdkmanager --verbose \
        "build-tools;30.0.3" \
        "ndk;25.1.8937393" \
        "platforms;android-33" \
        "system-images;android-29;default;x86_64" \
    && rm -R ${HOME}/.android/
RUN sdkmanager --list_installed

RUN cd ${HOME}/opt/android-sdk/ndk/25.1.8937393/toolchains/llvm/prebuilt/linux-x86_64/bin/ \
    && ln -s aarch64-linux-android30-clang aarch64-linux-android-clang \
    && ln -s armv7a-linux-androideabi30-clang arm-linux-androideabi-clang \
    && ln -s i686-linux-android30-clang i686-linux-android-clang \
    && ln -s x86_64-linux-android30-clang x86_64-linux-android-clang

# Replace the Android emulator by a custom one (version 31.3.11 from the
# archive at https://developer.android.com/studio/emulator_archive). Indeed,
# from version 31.3.12 on, the disk size is too big and doesn't fit in a
# reasonably-sized tmpfs.
#
# See https://developer.android.com/studio/releases/emulator#31-3-12.
RUN rm -R ${ANDROID_HOME}/emulator \
    && wget -nv https://dl.google.com/android/repository/emulator-linux_x64-9058569.zip \
        -O ${HOME}/emulator-linux_x64-9058569.zip \
    && sha256sum emulator-linux_x64-9058569.zip \
    && echo "5b06dae2b8c79b0a39456c3b4d31cf1895571bbf9763cc8ba84c8fdae15673e8 emulator-linux_x64-9058569.zip" | sha256sum -c - \
    && yes | unzip emulator-linux_x64-9058569.zip -d ${ANDROID_HOME} \
    && rm emulator-linux_x64-9058569.zip

USER root
COPY emulator-package.xml /home/dev/opt/android-sdk/emulator/package.xml
USER dev

# Create an Android virtual device.
RUN avdmanager create avd \
    -n test_avd \
    -d pixel \
    -k "system-images;android-29;default;x86_64" \
    && mv ${HOME}/.android ${HOME}/android

# Install Rust toolchain.
ENV NDK_HOME=${ANDROID_HOME}/ndk/25.1.8937393

RUN wget -nv https://sh.rustup.rs -O rustup.sh \
    && sha256sum rustup.sh \
    && echo "be3535b3033ff5e0ecc4d589a35d3656f681332f860c5fd6684859970165ddcc  rustup.sh" | sha256sum -c - \
    && sh rustup.sh -y \
    && rm rustup.sh \
    && rm .profile \
    && ${HOME}/.cargo/bin/rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        i686-linux-android \
        x86_64-linux-android \
    && ${HOME}/.cargo/bin/rustup toolchain install nightly \
    && ${HOME}/.cargo/bin/rustup target add --toolchain nightly \
        aarch64-linux-android \
        armv7-linux-androideabi \
        i686-linux-android \
        x86_64-linux-android \
    && mkdir -p ${HOME}/rustup \
    && mv ${HOME}/.rustup/toolchains ${HOME}/rustup/toolchains

ENV PATH=${PATH}:/home/dev/.cargo/bin

USER root

COPY \
    scripts/build-application.sh \
    scripts/build-application-relinked.sh \
    scripts/build-application-stage1.sh \
    scripts/setup.sh \
    scripts/clone-rustlang-head.sh \
    scripts/clone-rustlang-1.67.0-1286ee23e.sh \
    scripts/stage0.sh \
    scripts/stage1.sh \
    scripts/strip-rust.sh \
    scripts/script-rust-default.sh \
    scripts/script-rust-default-nostrip.sh \
    scripts/script-rust-nightly.sh \
    scripts/script-rust-nightly-nostrip.sh \
    scripts/script-rust-stage1.sh \
    scripts/script-rust-stage1-nostrip.sh \
    scripts/script-gradle.sh \
    scripts/script-java.sh \
    scripts/script-java-incorrect-arm64.sh \
    scripts/script-java-incorrect-x86_64.sh \
    scripts/emulator.sh \
    scripts/launch-app-debug.sh \
    scripts/launch-app-release.sh \
    scripts/flamedisk.sh \
    scripts/android-runner.sh \
    scripts/build-relinked.sh \
    scripts/script-relinked.sh \
    scripts/bench.sh \
    scripts/bench-haraka.sh \
    scripts/bench-horcrux.sh \
    /home/dev/
RUN chmod 555 \
    /home/dev/build-application.sh \
    /home/dev/build-application-relinked.sh \
    /home/dev/build-application-stage1.sh \
    /home/dev/setup.sh \
    /home/dev/clone-rustlang-head.sh \
    /home/dev/clone-rustlang-1.67.0-1286ee23e.sh \
    /home/dev/stage0.sh \
    /home/dev/stage1.sh \
    /home/dev/strip-rust.sh \
    /home/dev/script-rust-default.sh \
    /home/dev/script-rust-default-nostrip.sh \
    /home/dev/script-rust-nightly.sh \
    /home/dev/script-rust-nightly-nostrip.sh \
    /home/dev/script-rust-stage1.sh \
    /home/dev/script-rust-stage1-nostrip.sh \
    /home/dev/script-gradle.sh \
    /home/dev/script-java.sh \
    /home/dev/script-java-incorrect-arm64.sh \
    /home/dev/script-java-incorrect-x86_64.sh \
    /home/dev/emulator.sh \
    /home/dev/launch-app-debug.sh \
    /home/dev/launch-app-release.sh \
    /home/dev/flamedisk.sh \
    /home/dev/android-runner.sh \
    /home/dev/build-relinked.sh \
    /home/dev/script-relinked.sh \
    /home/dev/bench.sh \
    /home/dev/bench-haraka.sh \
    /home/dev/bench-horcrux.sh

COPY --chown=1000:1000 cargo-config.toml /home/dev/.cargo/config
COPY --chown=1000:1000 \
    rustbuild-config.toml \
    stdarch.patch \
    horcrux.dynamic_detect.patch \
    /home/dev/

COPY --chown=1000:1000 src /home/dev/src
COPY --chown=1000:1000 tools /home/dev/tools

USER dev

ENTRYPOINT [ "/bin/bash" ]

