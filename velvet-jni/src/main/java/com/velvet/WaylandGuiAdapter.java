package com.velvet;

public class WaylandGuiAdapter implements ModuleAdapter {
    private final VelvetJNI jni;

    public WaylandGuiAdapter() {
        this.jni = new VelvetJNI();
    }

    @Override
    public String execute(String args) {
        try {
            return jni.waylandGui(args);
        } catch (Throwable e) {
            return "Error: " + e.getMessage();
        }
    }
}
