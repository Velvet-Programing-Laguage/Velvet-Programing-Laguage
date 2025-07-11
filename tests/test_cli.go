package main

import (
    "testing"
    "os"
)

func TestInitProject(t *testing.T) {
    initProject()
    if _, err := os.Stat("src/config/vel.config"); os.IsNotExist(err) {
        t.Error("vel.config was not created")
    }
}

func TestInstallPython(t *testing.T) {
    handleInstall([]string{"<.>", "python", "requests"})
    libraryPath := getLibraryPath()
    if _, err := os.Stat(filepath.Join(libraryPath, "python", "requests")); os.IsNotExist(err) {
        t.Error("Python requests environment was not created")
    }
}
