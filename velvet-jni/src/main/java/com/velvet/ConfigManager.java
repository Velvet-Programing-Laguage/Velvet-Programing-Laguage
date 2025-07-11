package com.velvet;

import java.io.File;
import java.io.FileReader;
import com.fasterxml.jackson.databind.ObjectMapper;

public class ConfigManager {
    private ObjectMapper mapper = new ObjectMapper();
    private Map<String, String> config;

    public ConfigManager() {
        config = new HashMap<>();
    }

    public void loadConfig(String path) {
        try {
            File file = new File(path);
            if (file.exists()) {
                config = mapper.readValue(new FileReader(file), Map.class);
            }
        } catch (Exception e) {
            System.err.println("Error loading config: " + e.getMessage());
        }
    }

    public String getConfig(String key) {
        return config.getOrDefault(key, "");
    }
}
