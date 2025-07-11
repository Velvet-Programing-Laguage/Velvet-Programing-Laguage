package com.velvet;

public class PerfParallelAdapter implements ModuleAdapter {
    private final VelvetJNI jni;

    public PerfParallelAdapter() {
        this.jni = new VelvetJNI();
    }

    @Override
    public String execute(String args) {
        try {
            return jni.perfParallel(args);
        } catch (Throwable e) {
            return "Error: " + e.getMessage();
        }
    }
}
