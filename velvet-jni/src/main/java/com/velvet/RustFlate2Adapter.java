package com.velvet;

public class RustFlate2Adapter implements LibraryAdapter {
    @Override
    public String execute(String command) {
        try {
            String[] parts = command.split(",", 2);
            String method = parts[0];
            String data = parts[1];
            // Simulate Rust flate2 via JNI
            return "Rust flate2 " + method + " on " + data + " executed";
        } catch (Exception e) {
            return "Error in rust_flate2: " + e.getMessage();
        }
    }
}
