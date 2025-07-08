# Velvet Programming Language

Velvet is a modern, minimalist programming language designed to combine the simplicity of shell scripting with the clarity and structure of Rust. It features a domain-specific language (DSL) for creating graphical user interfaces (GUIs) using HTML/CSS, a robust command-line interface (CLI) inspired by npm, and a high-performance core written in Rust. Velvet aims to provide an intuitive development experience for creating both console-based and GUI applications with minimal boilerplate code.

Velvet is open-source, cross-platform, and built with a focus on developer productivity. It supports dynamic typing, a clean syntax, and an integrated development workflow with commands like `vel init`, `vel install`, `vel start`, and `vel debug`.

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
  - [CLI (Go)](#cli-go)
  - [GUI (JavaScript/HTML/CSS, Tauri)](#gui-javascript-htmlcss-tauri)
  - [Package Management](#package-management)
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


## Installtion

---

## Installation

   ```bash
   git clone https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Laguage.git

   ```
