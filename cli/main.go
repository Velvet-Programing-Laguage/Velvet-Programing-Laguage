package main

import (
    "encoding/json"
    "fmt"
    "github.com/go-resty/resty/v2"
    "github.com/spf13/cobra"
    "io/ioutil"
    "os"
    "os/exec"
    "path/filepath"
)

type VelConfig struct {
    Name         string            `json:"name"`
    Version      string            `json:"version"`
    Dependencies map[string]string `json:"dependencies"`
}

func main() {
    var rootCmd = &cobra.Command{Use: "vel"}

    var initCmd = &cobra.Command{
        Use:   "init",
        Short: "Initialize a new Velvet project",
        Run: func(cmd *cobra.Command, args []string) {
            config := VelConfig{
                Name:    "velvet-project",
                Version: "0.1.0",
                Dependencies: map[string]string{
                    "velvet-std": "^1.0.0",
                },
            }
            configData, _ := json.MarshalIndent(config, "", "  ")
            ioutil.WriteFile("vel.json", configData, 0644)
            ioutil.WriteFile("main.vel", []byte(`say "Hello World!"`), 0644)
            fmt.Println("Project initialized")
        },
    }

    var startCmd = &cobra.Command{
        Use:   "start",
        Short: "Run Velvet project",
        Run: func(cmd *cobra.Command, args []string) {
            go func() {
                tauriCmd := exec.Command("npm", "run", "tauri", "dev")
                tauriCmd.Dir = "gui"
                tauriCmd.Stdout = os.Stdout
                tauriCmd.Stderr = os.Stderr
                tauriCmd.Run()
            }()
            coreCmd := exec.Command("./target/release/velvet-core", "main.vel")
            coreCmd.Dir = "core"
            coreCmd.Stdout = os.Stdout
            coreCmd.Stderr = os.Stderr
            coreCmd.Run()
        },
    }

    var debugCmd = &cobra.Command{
        Use:   "debug",
        Short: "Run Velvet project in debug mode",
        Run: func(cmd *cobra.Command, args []string) {
            fmt.Println("Debug mode: Logging enabled")
            coreCmd := exec.Command("./target/debug/velvet-core", "main.vel")
            coreCmd.Dir = "core"
            coreCmd.Stdout = os.Stdout
            coreCmd.Stderr = os.Stderr
            coreCmd.Run()
        },
    }

    var installCmd = &cobra.Command{
        Use:   "install",
        Short: "Install Velvet dependencies",
        Run: func(cmd *cobra.Command, args []string) {
            configData, err := ioutil.ReadFile("vel.json")
            if err != nil {
                fmt.Println("Error reading vel.json:", err)
                return
            }
            var config VelConfig
            json.Unmarshal(configData, &config)
            client := resty.New()
            for name, version := range config.Dependencies {
                fmt.Printf("Installing %s@%s...\n", name, version)
                // Mocked download (replace with real repo URL)
                resp, err := client.R().Get(fmt.Sprintf("https://mock-repo/%s/%s", name, version))
                if err != nil {
                    fmt.Println("Error downloading:", err)
                    continue
                }
                os.MkdirAll("vel_modules", 0755)
                ioutil.WriteFile(filepath.Join("vel_modules", name+".vel"), resp.Body(), 0644)
            }
            fmt.Println("Dependencies installed")
        },
    }

    rootCmd.AddCommand(initCmd, startCmd, debugCmd, installCmd)
    rootCmd.Execute()
}
