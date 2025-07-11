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

<<<  @ Velvet 1.0 example with detailed comments to help learning

@ Importing modules:
@  - .> means import standard library or module
@  - <. means import specific parts from module (like Python's 'from')
@  - < > means import both ways, full module access
use .>math        @ import math library for calculations
use <.utils       @ import utility functions from utils module
use <io>          @ import full io module

@ Declare a constant value, cannot be changed later
const PI = 3.1415926535

@ Declare a variable with type String, initialized to "Velvet 0.1"
val version: String = "Velvet 0.1"

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

### Development Roadmap

### Contributing

### License

Apache 2.0 License


TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

  To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

   Copyright [yyyy] [name of copyright owner]

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

### Contact
