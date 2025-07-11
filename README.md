# Velvet Programming Language

Velvet is a modern, minimalist programming language designed to combine the simplicity of shell scripting with the clarity and structure of Rust. It features a domain-specific language (DSL) for creating graphical user interfaces (GUIs) using HTML/CSS, a robust command-line interface (CLI) inspired by npm, and a high-performance core written in Rust. Velvet aims to provide an intuitive development experience for creating both console-based and GUI applications with minimal boilerplate code.

Velvet is open-source, cross-platform, and built with a focus on developer productivity. It supports dynamic typing, a clean syntax, and an integrated development workflow with commands like `vel init`, `vel install`, `vel start`, `vel build`, `vel build -> release (package name)`, and `vel debug`.

---

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Project Structure](#project-structure)
- [Installation](#installation)
- [Usage](#usage)
  - [Writing Velvet Code](#writing-velvet-code)
  - [CLI Commands](#cli-commands)
- [Example](#example)
- [Technical Architecture](#technical-architecture)
  - [Core (Rust)](#core-rust)
  - [CLI (Rust)](#cli-rust)
  - [GUI (JavaScript/HTML/CSS, Tauri)](#gui-javascript-htmlcss-tauri)
  - [Package Management Java](#package-management)
- [Development Roadmap](#development-roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## Overview

Velvet is designed to make programming accessible yet powerful. It draws inspiration from the simplicity of shell scripting, the type safety and clarity of Rust, and the ease of package management from npm. Velvet's key goals are:

- **Simplicity**: A minimal syntax with keywords like `say` (instead of `print`) and `do` for blocks, reducing cognitive load for developers.
- **GUI Development**: A built-in DSL for creating GUI applications using HTML/CSS, rendered via a Webview (powered by Tauri or Electron).
- **Cross-Platform CLI**: A command-line interface written in Go, providing commands like `vel init`, `vel start`, and `vel debug` for a seamless developer experience.
- **Performance**: A core written in Rust for fast parsing and execution.
- **Extensibility**: A package management system similar to npm, using `vel.json` to define dependencies.

Velvet is ideal for rapid prototyping, small-scale applications, and developers who want to create GUI or console applications without complex setup.

---

## Key Features

| **Feature**            | **Description**                                                                 |
|------------------------|--------------------------------------------------------------------------------|
| **Minimal Syntax**     | Uses simple keywords like `say` and `do`, with optional indentation-based blocks (Python-like). No semicolons required. |
| **Dynamic Typing**     | Type system inspired by Python/Shell for flexibility and ease of use.           |
| **GUI DSL**            | A domain-specific language to define GUI elements (e.g., `window`, `button`) that generate HTML/CSS. |
| **CLI Tools**          | A Go-based CLI (`vel`) for project initialization, dependency management, and debugging. |
| **Package Management** | Dependency management via `vel.json`, similar to `package.json` in npm.         |
| **Debugger**           | Built-in debugging mode accessible via `vel debug`.                            |
| **Runtime**            | Rust-based interpreter with a JavaScript interface for GUI rendering.           |
| **GUI Rendering**      | Webview-based GUI using Tauri (lightweight) or Electron, rendering DSL-generated HTML/CSS. |
| **Future Web IDE**     | Planned support for a browser-based IDE using React or Svelte.                  |

---

## Project Structure

The Velvet project is organized into modular components, each implemented in a language best suited for its purpose.

velvet/
├── cli/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── core/
│   ├── Cargo.toml
│   └── src/
│       ├── builder.rs
│       ├── config.rs
│       ├── error.rs
│       ├── ffi.rs
│       ├── interpreter.rs
│       ├── logger.rs
│       ├── module_registry.rs
│       ├── modules.rs
│       ├── parser.rs
│       ├── plugin_system.rs
│       ├── repl.rs
│       ├── runtime.rs
│       ├── stdlib_1.rs
│       ├── stdlib_2.rs
│       └── types.rs
├── examples/
│   ├── ai_prediction.vel
│   ├── crypto_ops.vel
│   ├── http_workflow.vel
│   ├── jython_gui.vel
│   ├── modern_gui.vel
│   ├── parallel_processing.vel
│   └── wayland_app.vel
├── gui/
│   ├── package.json
│   └── src/
│       ├── app.js
│       ├── events.js
│       ├── gui.js
│       ├── index.html
│       └── styles.css
├── vel_modules/
│   ├── ai_pytorch.vel
│   ├── ai_tensorflow.vel
│   ├── cpp_boost.vel
│   ├── csharp_json.vel
│   ├── db_sqlite.vel
│   ├── gpu_cuda.vel
│   ├── java_jython.vel
│   ├── js_axios.vel
│   ├── net_websocket.vel
│   ├── perf_crypto.vel
│   ├── perf_parallel.vel
│   ├── python_requests.vel
│   ├── ruby_httparty.vel
│   ├── rust_flate2.vel
│   ├── tauri_gui.vel
│   └── wayland_gui.vel
├── velvet-jni/
│   ├── pom.xml
│   └── src/
│       └── main/
│           └── java/
│               └── com/
│                   └── velvet/
│                       ├── AiPytorchAdapter.java
│                       ├── AiTensorflowAdapter.java
│                       ├── AsyncAdapter.java
│                       ├── CppBoostAdapter.java
│                       ├── CsharpJsonAdapter.java
│                       ├── DbSqliteAdapter.java
│                       ├── GpuCudaAdapter.java
│                       ├── JavaJythonAdapter.java
│                       ├── JsAxiosAdapter.java
│                       ├── LibraryManager.java
│                       ├── ModuleAdapter.java
│                       ├── NetWebsocketAdapter.java
│                       ├── PerfCryptoAdapter.java
│                       ├── PerfParallelAdapter.java
│                       ├── PythonRequestsAdapter.java
│                       ├── RubyHttpartyAdapter.java
│                       ├── RustFlate2Adapter.java
│                       ├── TauriGuiAdapter.java
│                       ├── VelvetJNI.java
│                       └── WaylandGuiAdapter.java
└── vel.json

## Installation

### 1) Installation From Source


Install Repo

```bash
git clone https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Laguage.git
```

Go to file

```bash
cd Velvet-Programing-Laguage
```

#### FOR LINUX AND MACOS


Add Sudo Permission for script

```bash
chmod +x install.sh
```

Run Install.sh

```bash
./install.sh
```


#### FOR WINDOWS

Add Sudo Permission For Script

```bash
Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
```

Run Install.ps1

```bash
.\install.ps1
```

### 2) Fast Installation

#### For Linux
```bash
curl -OL https://raw.githubusercontent.com/Velvet-Programing-Laguage/Velvet-Programing-Laguage/main/install.sh
chmod +x install.sh
./install.sh
```

#### For Windows
```bash
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm https://raw.githubusercontent.com/Velvet-Programing-Laguage/Velvet-Programing-Laguage/main/install.ps1 | iex
```

## Usage

Writing your first Velvet program

1. Create a new file with the .v extension, for example:



# hello_world.vel

say("Hello, Velvet World!")


---

Running your Velvet program (CLI)

After installing Velvet using the install.sh script:

vel run hello_world.vel

This will compile and run your Velvet code, displaying:

Hello, Velvet World!


---

Building a Velvet project

To build (compile) your project without running it immediately:

vel build hello_world.vel

The compiled output will be stored in the default build directory.


---

Initializing a new Velvet project

You can initialize a new Velvet project structure using:

vel init my_project
cd my_project

This creates a folder with the recommended Velvet project layout.


---


 Command overview

Command	Description

vel init <project>	Initialize a new Velvet project
vel build <file.vel>	Build a Velvet file/project
vel run <file.vel>	Run a Velvet file directly
vel gui	Launch the Velvet GUI
vel --help	Show available CLI options



---

 Example workflow

git clone https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Language.git
cd Velvet-Programing-Language
sudo ./install.sh

# Create and run your first program
echo 'say("Hello from Velvet!")' > test.vel
vel run test.vel

### Writing Velvet Code

### CLI Commands

## Example

## Technical Architecture

### Core Rust

### CLI Rust

### 
