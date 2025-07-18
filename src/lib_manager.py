import subprocess
import os
import sys
import json
import hashlib
import logging
import shutil

LIB_DIR = os.path.join("lib", ".velvet_library")
CONFIG_FILE = os.path.join(LIB_DIR, "config.json")
LOG_FILE = os.path.join(LIB_DIR, "lib_manager.log")

logging.basicConfig(filename=LOG_FILE, level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s")

def ensure_lib_dir():
    os.makedirs(LIB_DIR, exist_ok=True, mode=0o700)
    if not os.path.exists(CONFIG_FILE):
        with open(CONFIG_FILE, 'w') as f:
            json.dump({"libraries": {}, "version": "1.0"}, f)
    os.chmod(LIB_DIR, 0o700)
    os.chmod(CONFIG_FILE, 0o600)

def get_isolated_dir(manager: str, command: str) -> str:
    return os.path.join(LIB_DIR, f"{manager}_{hashlib.sha256((manager + command).encode()).hexdigest()}")

def install_library(manager: str, command: str):
    ensure_lib_dir()
    isolated_dir = get_isolated_dir(manager, command)
    if os.path.exists(isolated_dir):
        shutil.rmtree(isolated_dir)
    os.makedirs(isolated_dir, mode=0o700)
    
    managers = {
        "pip": lambda: subprocess.run(f"{command} --target={isolated_dir} --no-user", shell=True, capture_output=True),
        "gem": lambda: subprocess.run(f"{command} --install-dir={isolated_dir}", shell=True, capture_output=True),
        "mix": lambda: subprocess.run(f"{command}", shell=True, cwd=isolated_dir, capture_output=True),
        "cargo": lambda: subprocess.run(f"{command}", shell=True, cwd=isolated_dir, capture_output=True),
        "npm": lambda: subprocess.run(f"{command} --prefix={isolated_dir}", shell=True, capture_output=True),
        "yarn": lambda: subprocess.run(f"{command} --prefix={isolated_dir}", shell=True, capture_output=True),
        "composer": lambda: subprocess.run(f"{command} --install-dir={isolated_dir}", shell=True, capture_output=True),
        "mvn": lambda: subprocess.run(f"{command} --file={isolated_dir}/pom.xml", shell=True, capture_output=True),
        "gradle": lambda: subprocess.run(f"{command} --project-dir={isolated_dir}", shell=True, capture_output=True),
        "go": lambda: subprocess.run(f"{command}", shell=True, cwd=isolated_dir, capture_output=True),
        "nuget": lambda: subprocess.run(f"{command} -OutputDirectory {isolated_dir}", shell=True, capture_output=True),
    }
    if manager not in managers:
        logging.error(f"Unsupported manager: {manager}")
        print(f"Error: Unsupported manager: {manager}")
        sys.exit(1)
    
    logging.info(f"Installing {command} via {manager} in {isolated_dir}")
    result = managers[manager]()
    if result.returncode == 0:
        with open(CONFIG_FILE, 'r+') as f:
            config = json.load(f)
            config["libraries"].setdefault(manager, []).append({"command": command, "dir": isolated_dir})
            f.seek(0)
            json.dump(config, f, indent=2)
        logging.info(f"Installed {command} via {manager}")
        print(f"Installed {command} via {manager}")
    else:
        logging.error(f"Failed to install {command}: {result.stderr.decode()}")
        print(f"Error installing {command}: {result.stderr.decode()}")
        sys.exit(1)

def update_libraries():
    ensure_lib_dir()
    with open(CONFIG_FILE, 'r') as f:
        config = json.load(f)
    for manager, libraries in config["libraries"].items():
        for lib in libraries:
            print(f"Updating {lib['command']} via {manager}...")
            install_library(manager, lib["command"])

def list_libraries():
    ensure_lib_dir()
    with open(CONFIG_FILE, 'r') as f:
        config = json.load(f)
    for manager, libraries in config["libraries"].items():
        print(f"\n{manager}:")
        for lib in libraries:
            print(f"  - {lib['command']} ({lib['dir']})")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python lib_manager.py <install|update|list> [manager] [command]")
        sys.exit(1)
    action = sys.argv[1]
    if action == "install":
        if len(sys.argv) < 4:
            print("Usage: python lib_manager.py install <manager> <command>")
            sys.exit(1)
        install_library(sys.argv[2], " ".join(sys.argv[3:]))
    elif action == "update":
        update_libraries()
    elif action == "list":
        list_libraries()
    else:
        print(f"Unknown action: {action}")
        sys.exit(1)
