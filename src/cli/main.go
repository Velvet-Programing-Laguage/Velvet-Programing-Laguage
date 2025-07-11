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
        buildProject()
    case "debug":
        debugProject()
    case "start":
        startProject(args)
    case "errors":
        displayErrors()
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
        fmt.Println("Usage: vel install <.> <language> <dependency>")
        os.Exit(1)
    }

    language := strings.ToLower(args[1])
    depName := strings.ReplaceAll(args[2], "/", "_") // Sanitize for directory name
    command := strings.Join(args[2:], " ")
    libraryPath := getLibraryPath()
    envPath := filepath.Join(libraryPath, language, depName)

    os.MkdirAll(envPath, 0755)

    var cmd *exec.Cmd
    switch language {
    case "python":
        cmd = exec.Command("python", "-m", "venv", envPath)
        if err := cmd.Run(); err != nil {
            fmt.Printf("Error creating Python venv: %v\n", err)
            os.Exit(1)
        }
        if runtime.GOOS == "windows" {
            cmd = exec.Command(filepath.Join(envPath, "Scripts", "pip"), "install", command)
        } else {
            cmd = exec.Command(filepath.Join(envPath, "bin", "pip"), "install", command)
        }
    case "ruby":
        gemfile := filepath.Join(envPath, "Gemfile")
        os.WriteFile(gemfile, []byte(fmt.Sprintf("source 'https://rubygems.org'\ngem '%s'", command)), 0644)
        cmd = exec.Command("bundle", "install", "--path", envPath, "--gemfile", gemfile)
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
    case "javascript", "typescript":
        cmd = exec.Command("npm", "install", "--prefix", envPath, command)
    case "kotlin":
        cmd = exec.Command("gradle", "install", "-Ddir="+envPath, command)
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
    os.MkdirAll("src/config", 0755)
    configPath := "src/config/vel.config"
    os.WriteFile(configPath, []byte("# Velvet configuration\n"), 0644)
    fmt.Println("Initialized Velvet project")
}

func buildProject() {
    fmt.Println("Building Velvet project...")
    cmd := exec.Command("cargo", "build", "--release")
    cmd.Stdout = os.Stdout
    cmd.Stderr = os.Stderr
    if err := cmd.Run(); err != nil {
        fmt.Printf("Error building project: %v\n", err)
        os.Exit(1)
    }
}

func debugProject() {
    fmt.Println("Debugging Velvet project...")
    cmd := exec.Command("cargo", "run")
    cmd.Stdout = os.Stdout
    cmd.Stderr = os.Stderr
    if err := cmd.Run(); err != nil {
        fmt.Printf("Error debugging project: %v\n", err)
        os.Exit(1)
    }
}

func startProject(args []string) {
    script := "tests/test_velvet.vel"
    if len(args) > 0 {
        script = args[0]
    }
    cmd := exec.Command("./velvet_core", script)
    cmd.Stdout = os.Stdout
    cmd.Stderr = os.Stderr
    if err := cmd.Run(); err != nil {
        fmt.Printf("Error starting project: %v\n", err)
        os.Exit(1)
    }
}

func displayErrors() {
    fmt.Println("Displaying recent errors...")
    // Placeholder: Read error logs
}

func handleUpdate(args []string) {
    if len(args) < 2 || args[0] != "<.>" {
        fmt.Println("Usage: vel update <.> <language> <dependency>")
        os.Exit(1)
    }

    language := strings.ToLower(args[1])
    depName := strings.ReplaceAll(args[2], "/", "_")
    command := strings.Join(args[2:], " ")
    libraryPath := getLibraryPath()
    envPath := filepath.Join(libraryPath, language, depName)

    var cmd *exec.Cmd
    switch language {
    case "python":
        if runtime.GOOS == "windows" {
            cmd = exec.Command(filepath.Join(envPath, "Scripts", "pip"), "install", "--upgrade", command)
        } else {
            cmd = exec.Command(filepath.Join(envPath, "bin", "pip"), "install", "--upgrade", command)
        }
    case "ruby":
        gemfile := filepath.Join(envPath, "Gemfile")
        cmd = exec.Command("bundle", "update", "--gemfile", gemfile)
    case "rust":
        cmd = exec.Command("cargo", "install", "--root", envPath, "--force", command)
    case "go":
        cmd = exec.Command("go", "get", "-u", command)
        cmd.Dir = envPath
    case "crystal":
        cmd = exec.Command("crystal", "deps", "update", command)
        cmd.Dir = envPath
    case "elixir":
        cmd = exec.Command("mix", "deps.update", command)
        cmd.Dir = envPath
    case "javascript", "typescript":
        cmd = exec.Command("npm", "update", "--prefix", envPath, command)
    case "java", "kotlin":
        cmd = exec.Command("mvn", "install", "-Ddir="+envPath, "-U", command)
    default:
        fmt.Printf("Unsupported language: %s\n", language)
        os.Exit(1)
    }

    cmd.Stdout = os.Stdout
    cmd.Stderr = os.Stderr
    if err := cmd.Run(); err != nil {
        fmt.Printf("Error updating %s for %s: %v\n", command, language, err)
        os.Exit(1)
    }
    fmt.Printf("Updated %s for %s in %s\n", command, language, envPath)
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
  install <.> <lang> <dep>  Install a dependency in an isolated environment
  build               Build the Velvet project
  debug               Run project in debug mode
  start [file.vel]    Start the Velvet application
  errors              Display recent errors
  update <.> <cmd>    Update a dependency
  reset               Reset all isolated environments
  help, ?             Show this help message
`)
}
