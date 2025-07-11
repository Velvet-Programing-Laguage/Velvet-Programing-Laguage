package com.velvet;

public class TauriGuiAdapter implements ModuleAdapter {
    private final VelvetJNI jni;

    public TauriGuiAdapter() {
        this.jni = new VelvetJNI();
    }

    @Override
    public String execute(String args) {
        try {
            return jni.tauriGui(args);
        } catch (Throwable e) {
            return "Error: " + e.getMessage();
        }
    }
}
