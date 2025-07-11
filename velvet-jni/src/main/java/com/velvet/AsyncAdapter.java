package com.velvet;

import java.util.concurrent.CompletableFuture;

public class AsyncAdapter {
    private final VelvetJNI jni;

    public AsyncAdapter() {
        this.jni = new VelvetJNI();
    }

    public CompletableFuture<String> executeAsync(String module, String args) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                return jni.asyncExec(module, args);
            } catch (Throwable e) {
                return "Error: " + e.getMessage();
            }
        });
    }
}
