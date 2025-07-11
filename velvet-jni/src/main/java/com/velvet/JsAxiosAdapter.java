package com.velvet;

public class JsAxiosAdapter implements LibraryAdapter {
    @Override
    public String execute(String command) {
        try {
            String[] parts = command.split(",", 2);
            String method = parts[0];
            String url = parts[1];
            // Simulate Axios via Tauri bridge
            return "JS Axios " + method + " to " + url + " executed";
        } catch (Exception e) {
            return "Error in js_axios: " + e.getMessage();
        }
    }
}
