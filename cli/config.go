package main

import (
    "encoding/json"
    "os"
)

type Config struct {
    Debug        bool              `json:"debug"`
    Dependencies map[string]string `json:"dependencies"`
}

func loadConfig() Config {
    file, err := os.Open("vel.json")
    if err != nil {
        logger.Println("Error loading config:", err)
        return Config{}
    }
    defer file.Close()
    var config Config
    decoder := json.NewDecoder(file)
    if err := decoder.Decode(&config); err != nil {
        logger.Println("Error decoding config:", err)
        return Config{}
    }
    return config
}
