package com.velvet;

public class GpuCudaAdapter implements ModuleAdapter {
    private final VelvetJNI jni;

    public GpuCudaAdapter() {
        this.jni = new VelvetJNI();
    }

    @Override
    public String execute(String args) {
        try {
            return jni.gpuCuda(args);
        } catch (Throwable e) {
            return "Error: " + e.getMessage();
        }
    }
}
