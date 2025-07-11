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

    public native String base64Encode(String args);

    public native String compressData(String args);

    public native String hashMD5(String args);

    public native String httpPost(String args);

    public native String imageCrop(String args);

    public native String jsonValidate(String args);

    public native String xmlParse(String args);

    public native String yamlValidate(String args);

    public native String dbMongoConnect(String args);

    public native String dbRedisConnect(String args);
}
