package com.example.myrustapplication;

import android.os.Build;
import android.os.Bundle;
import android.util.Log;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;

import java.util.Arrays;

public class MainActivity extends AppCompatActivity {

    private static final String TAG = "MyRustSimdApplication";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        Log.i(TAG, "SUPPORTED_ABIS: " + Arrays.toString(Build.SUPPORTED_ABIS));
        Log.i(TAG, "SUPPORTED_32_BIT_ABIS: " + Arrays.toString(Build.SUPPORTED_32_BIT_ABIS));
        Log.i(TAG, "SUPPORTED_64_BIT_ABIS: " + Arrays.toString(Build.SUPPORTED_64_BIT_ABIS));

        Log.i(TAG, "CPU_ABI [deprecated]: " + Build.CPU_ABI);
        Log.i(TAG, "CPU_ABI2 [deprecated]: " + Build.CPU_ABI2);
        Log.i(TAG, "OS.ARCH: " + System.getProperty("os.arch"));

        NativeLibrary lib = new NativeLibrary();
        String message = lib.run();
        ((TextView) findViewById(R.id.greetingField)).setText(message);
    }
}
