package com.example.esp_server_app

import io.flutter.embedding.android.FlutterActivity

class MainActivity: FlutterActivity() {
    init {
        System.loadLibrary("rust_lib");
    }
}
