package com.velvet;

public class AiTensorflowAdapter implements ModuleAdapter {
    private final VelvetJNI jni;

    public AiTensorflowAdapter() {
        this.jni = new VelvetJNI();
    }

    @Override
    public String execute(String args) {
        try {
            return jni.aiTensorflow(args);
        } catch (Throwable e) {
            return "Error: " + e.getMessage();
        }
    }
}
