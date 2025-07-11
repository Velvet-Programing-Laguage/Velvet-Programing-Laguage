package com.velvet;

public class DbSqliteAdapter implements ModuleAdapter {
    private final VelvetJNI jni;

    public DbSqliteAdapter() {
        this.jni = new VelvetJNI();
    }

    @Override
    public String execute(String args) {
        try {
            return jni.dbSqlite(args);
        } catch (Throwable e) {
            return "Error: " + e.getMessage();
        }
    }
}
