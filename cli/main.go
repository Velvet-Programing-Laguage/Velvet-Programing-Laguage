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
                Version: "0.4.0",
                Dependencies: map[string]string{
                    "fs": "^1.0.0",
                    "http": "^1.0.0",
                    "time": "^1.0.0",
                    "crypto": "^1.0.0",
                    "math": "^1.0.0",
                    "os": "^1.0.0",
                    "random": "^1.0.0",
                    "string": "^1.0.0"
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

    var testCmd = &cobra.Command{
        Use:   "test",
        Short: "Run Velvet tests",
        Run: func(cmd *cobra.Command, args []string) {
            testFiles, _ := filepath.Glob("examples/test*.vel")
            for _, file := range testFiles {
                fmt.Printf("Running test: %s\n", file)
                coreCmd := exec.Command("./target/release/velvet-core", file)
                coreCmd.Dir = "core"
                coreCmd.Stdout = os.Stdout
                coreCmd.Stderr = os.Stderr
                if err := coreCmd.Run(); err != nil {
                    fmt.Printf("Test %s failed: %v\n", file, err)
                } else {
                    fmt.Printf("Test %s passed\n", file)
                }
            }
        },
    }

    var runCmd = &cobra.Command{
        Use:   "run [file]",
        Short: "Run a specific Velvet file",
        Args:  cobra.ExactArgs(1),
        Run: func(cmd *cobra.Command, args []string) {
            file := args[0]
            if _, err := os.Stat(file); os.IsNotExist(err) {
                fmt.Printf("File %s does not exist\n", file)
                return
            }
            coreCmd := exec.Command("./target/release/velvet-core", file)
            coreCmd.Dir = "core"
            coreCmd.Stdout = os.Stdout
            coreCmd.Stderr = os.Stderr
            coreCmd.Run()
        },
    }

    rootCmd.AddCommand(initCmd, startCmd, debugCmd, installCmd, testCmd, runCmd)
    rootCmd.Execute()
}
