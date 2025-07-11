package com.velvet;

/**
 * Adapter JNI do biblioteki natywnej velvet_core.
 * Upewnij się, że plik natywny velvet_core jest dostępny dla JVM (np. w java.library.path).
 * Każda metoda musi być zaimplementowana po stronie C/C++ (JNI).
 */
public class VelvetJNI {
    static {
        System.loadLibrary("velvet_core");
    }

    /**
     * Inicjalizuje natywny backend Velvet.
     * @param args Argumenty inicjalizacyjne
     * @return Wynik inicjalizacji
     */
    public native String init(String args);

    /**
     * Wywołuje natywny backend Python (requests).
     * @param args Argumenty
     * @return Wynik
     */
    public native String pythonRequests(String args);

    public native String cppBoost(String args);
    public native String csharpJson(String args);
    public native String rubyHttparty(String args);
    public native String jsAxios(String args);
    public native String rustFlate2(String args);
    public native String javaJython(String args);
    public native String tauriGui(String args);
    public native String waylandGui(String args);
    public native String aiTensorflow(String args);
    public native String aiPytorch(String args);
    public native String perfParallel(String args);
    public native String perfCrypto(String args);
    public native String dbSqlite(String args);
    public native String netWebsocket(String args);
    public native String gpuCuda(String args);

    /**
     * Asynchroniczne wywołanie natywnych modułów.
     * @param module Nazwa modułu
     * @param args Argumenty
     * @return Wynik
     */
    public native String asyncExec(String module, String args);
}
