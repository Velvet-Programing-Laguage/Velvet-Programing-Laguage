package com.velvet;

public class VelvetJNI {
    static {
        System.loadLibrary("velvet_core");
    }

    public native void initJNI();

    public native String runPygame(String args);

    public native String runTk(String args);

    public native String captureCamera(String args);

    public native String createPDF(String args);
}
