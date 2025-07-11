package main

import (
    "encoding/json"
    "os"
)

type Config struct {
    Dependencies map[string]string
}

func loadConfig() (*Config, error) {
    file, err := os.Open("vel.json")
    if err != nil {
        return nil, err
    }
    defer file.Close()
    var config Config
    decoder := json.NewDecoder(file)
    err = decoder.Decode(&config)
    return &config, err
}
