package com.velvet;

public class NetWebsocketAdapter implements ModuleAdapter {
    private final VelvetJNI jni;

    public NetWebsocketAdapter() {
        this.jni = new VelvetJNI();
    }

    @Override
    public String execute(String args) {
        try {
            return jni.netWebsocket(args);
        } catch (Throwable e) {
            return "Error: " + e.getMessage();
        }
    }
}
