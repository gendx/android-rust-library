package com.example.myrustapplication;

public class NativeLibrary {

    static {
        System.loadLibrary("simd");
    }

    public String run() {
        return nativeRun();
    }

    private static native String nativeRun();
}
