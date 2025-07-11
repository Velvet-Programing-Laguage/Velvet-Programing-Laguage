# Velvet Programming Language

Velvet is a modern, minimalist programming language designed to combine the simplicity of shell scripting with the clarity and structure of Rust. It features a domain-specific language (DSL) for creating graphical user interfaces (GUIs) using HTML/CSS, a robust command-line interface (CLI) inspired by npm, and a high-performance core written in Rust. Velvet aims to provide an intuitive development experience for creating both console-based and GUI applications with minimal boilerplate code.

Velvet is open-source, cross-platform, and built with a focus on developer productivity. It supports dynamic typing, a clean syntax, and an integrated development workflow with commands like `vel init`, `vel install`, `vel start`, `vel build`, `vel build -> release (package name)`, and `vel debug`.

---

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Installation](#installation)
- [Usage](#usage)
  - [Writing Velvet Code](#writing-velvet-code)
  - [CLI Commands](#cli-commands)
- [Example](#example)
- [Technical Architecture](#technical-architecture)
  - [Core (Rust)](#core-rust)
  - [CLI (Rust)](#cli-rust)
  - [Python Interpreter](#python-interpreter)
- [Development Roadmap](#development-roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## Overview

Velvet is designed to make programming accessible yet powerful. It draws inspiration from the simplicity of shell scripting, the type safety and clarity of Rust, and the ease of package management from npm. Velvet's key goals are:

- **Simplicity**: A minimal syntax with keywords like `say` (instead of `print`) and `do` for blocks, reducing cognitive load for developers.
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
| **Runtime**            | Rust-based interpreter with a python        |

---


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


# Create and run your first program
echo 'say("Hello from Velvet!")' > test.vel
vel run test.vel

### Writing Velvet Code

###### Random Code in Velvet

<<< @ Velvet 1.0 example with detailed comments to help learning

@ Importing modules:
@  - .> means import standard library or module
@  - <. means import specific parts from module (like Python's 'from')
@  - < > means import both ways, full module access
use .>math        @ import math library for calculations
use <.utils       @ import utility functions from utils module
use <io>          @ import full io module

@ Declare a constant value, cannot be changed later
const PI = 3.1415926535

@ Declare a variable with type String, initialized to "Velvet 1.0"
val version: String = "Velvet 1.0"

@ Define a function named circle_area that calculates the area of a circle
@ It takes one argument: radius (type Number)
fun circle_area(radius: Number) -> 
    PI * radius * radius   @ formula for circle area: πr²

@ Define a lambda (anonymous function) to double a number
let double = (x) => x * 2

@ Define a function to filter numbers greater than a threshold
fun filter_greater_than(nums: List, threshold: Number) -> 
    nums |> filter(x => x > threshold) 
    @ Use pipeline operator to apply filter: keep x where x > threshold

@ Define a struct (like class) for User
type User:
    name: String     @ User's name
    age: Number      @ User's age
    active: Bool     @ Is user active?

@ Create a list of users (instances of User)
let users = [
    User(name="Michal", age=21, active=true),    @ active user
    User(name="Anna", age=30, active=false),     @ inactive user
    User(name="Tom", age=18, active=true)        @ active user
]

@ Define the main function - program entry point
fun main() -> 
    say "Language version: " + version   @ Print the version string

    let r = 5                           @ Define radius variable
    let area = circle_area(r)           @ Calculate area using our function
    say "Circle area with radius " + r + ": " + area

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]  @ Define a list of numbers
    let doubled_numbers = numbers |> map(double)    @ Double each number using map and lambda
    say "Doubled numbers: " + doubled_numbers

    let filtered = filter_greater_than(doubled_numbers, 10)  @ Filter doubled numbers greater than 10
    say "Numbers > 10: " + filtered

    @ Loop through each user in users list
    for user in users:
        if user.active:                      @ Check if user is active
            say "Active user: " + user.name + ", age: " + user.age
        else:
            say "Inactive user: " + user.name

    @ Use match statement to handle different cases for user's name
    match users[0].name:
        "Michal" | say "Hello Michal! Nice to see you!"  @ If name is Michal
        _ | say "Unknown user"                           @ Default case for others

    @ Example of error handling using try-catch
    try:
        let x = 10 / 0                  @ This will cause an error (division by zero)
        say "Division result: " + x
    catch e:
        say "An error occurred: " + e   @ Catch and print the error message

@ Call main function to run the program
main()  >>>

### CLI Commands

vel restart < The command restarts and clears the isolated environment of all dependencies. >
vel init <project>	< Initialize a new Velvet project >
vel build <location to project> < Compile Project >
vel run <file.vel>	< Run a Velvet file directly >
vel build .> release deb < Packs all code written in velvet into a deb file. >
vel ? <	Show every commands >
vel install <.> cargo install clap 
< Install libraries for each language in an isolated environment. >
vel install < Go to the directory with your code, save the vel.json file, run vel install. >

## Example

## Technical Architecture

### Core Rust

### CLI Go

### Python Interpreter
