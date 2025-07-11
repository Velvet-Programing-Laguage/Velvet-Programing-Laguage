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

    public native String pythonRequests(String args);

    public native String cppBoost(String args);

    public native String csharpJson(String args);

    public native String rubyHttparty(String args);

    public native String jsAxios(String args);

    public native String rustFlate2(String args);

    public native String javaJython(String args);
}
