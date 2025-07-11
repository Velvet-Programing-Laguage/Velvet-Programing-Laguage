package com.velvet;

public class PerfCryptoAdapter implements ModuleAdapter {
    private final VelvetJNI jni;

    public PerfCryptoAdapter() {
        this.jni = new VelvetJNI();
    }

    @Override
    public String execute(String args) {
        try {
            return jni.perfCrypto(args);
        } catch (Throwable e) {
            return "Error: " + e.getMessage();
        }
    }
}
