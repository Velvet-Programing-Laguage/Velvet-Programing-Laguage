package main

import (
    "fmt"
    "os"
    "os/exec"
    "path/filepath"
    "runtime"
    "strings"
)

func main() {
    if len(os.Args) < 2 {
        fmt.Println("Usage: vel <command> [args]")
        os.Exit(1)
    }

    command := os.Args[1]
    args := os.Args[2:]

    switch command {
    case "init":
        initProject()
    case "install":
        handleInstall(args)
    case "build":
        fmt.Println("Building project...")
    case "debug":
        fmt.Println("Debugging project...")
    case "start":
        startProject(args)
    case "errors":
        fmt.Println("Displaying errors...")
    case "update":
        handleUpdate(args)
    case "reset":
        resetEnvironments()
    case "help", "?":
        printHelp()
    default:
        fmt.Printf("Unknown command: %s\n", command)
        printHelp()
    }
}

func getLibraryPath() string {
    if runtime.GOOS == "windows" {
        return filepath.Join(os.Getenv("ProgramFiles"), ".velvet-library")
    }
    return "/usr/lib/.velvet-library"
}

func handleInstall(args []string) {
    if len(args) < 2 || args[0] != "<.>" {
        fmt.Println("Usage: vel install <.> <language> <command>")
        os.Exit(1)
    }

    language := strings.ToLower(args[1])
    command := strings.Join(args[2:], " ")
    libraryPath := getLibraryPath()
    envPath := filepath.Join(libraryPath, language, command)

    os.MkdirAll(envPath, 0755)

    var cmd *exec.Cmd
    switch language {
    case "python":
        cmd = exec.Command("python", "-m", "venv", envPath)
        cmd.Run()
        cmd = exec.Command(filepath.Join(envPath, "bin", "pip"), "install", command)
    case "ruby":
        cmd = exec.Command("bundle", "install", "--path", envPath)
    case "rust":
        cmd = exec.Command("cargo", "install", "--root", envPath, command)
    case "go":
        cmd = exec.Command("go", "get", "-d", command)
        cmd.Dir = envPath
    case "crystal":
        cmd = exec.Command("crystal", "deps", "install", command)
        cmd.Dir = envPath
    case "elixir":
        cmd = exec.Command("mix", "deps.get", command)
        cmd.Dir = envPath
    case "java":
        cmd = exec.Command("mvn", "install", "-Ddir="+envPath, command)
    case "javascript":
        cmd = exec.Command("npm", "install", "--prefix", envPath, command)
    default:
        fmt.Printf("Unsupported language: %s\n", language)
        os.Exit(1)
    }

    cmd.Stdout = os.Stdout
    cmd.Stderr = os.Stderr
    if err := cmd.Run(); err != nil {
        fmt.Printf("Error installing %s for %s: %v\n", command, language, err)
        os.Exit(1)
    }
    fmt.Printf("Installed %s for %s in %s\n", command, language, envPath)
}

func initProject() {
    os.Mkdir("src", 0755)
    os.Mkdir("src/config", 0755)
    configPath := "src/config/vel.config"
    os.WriteFile(configPath, []byte("# Velvet configuration\n"), 0644)
    fmt.Println("Initialized Velvet project")
}

func startProject(args []string) {
    if len(args) == 0 {
        fmt.Println("Starting default Velvet project...")
    } else {
        fmt.Printf("Starting Velvet file: %s\n", args[0])
    }
}

func handleUpdate(args []string) {
    fmt.Println("Updating dependencies...")
    // Similar to handleInstall, implement update logic
}

func resetEnvironments() {
    libraryPath := getLibraryPath()
    if err := os.RemoveAll(libraryPath); err != nil {
        fmt.Printf("Error resetting environments: %v\n", err)
        os.Exit(1)
    }
    fmt.Println("Reset all isolated environments")
}

func printHelp() {
    fmt.Println(`
Velvet CLI Commands:
  init                Initialize a new Velvet project
  install <.> <lang> <cmd>  Install a dependency in an isolated environment
  build               Build the Velvet project
  debug               Run project in debug mode
  start [file.vel]    Start the Velvet application
  errors              Display recent errors
  update <.> <cmd>    Update a dependency
  reset               Reset all isolated environments
  help, ?             Show this help message
`)
}
