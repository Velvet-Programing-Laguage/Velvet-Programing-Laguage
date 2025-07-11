package com.velvet;

public class VelvetJNI {
    static {
        System.loadLibrary("velvet_core");
    }

    public native String init(String args);
    public native String pythonRequests(String args);
    public native String cppBoost(String args);
    public native String csharpJson(String args);
    public native String rubyHttparty(String args);
    public native String jsAxios(String args);
    public native String rustFlate2(String args);
    public native String javaJython(String args);
}
