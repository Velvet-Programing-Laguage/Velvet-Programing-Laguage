import subprocess
import os
import platform

def get_library_path():
    """Determine the library path based on the OS."""
    if platform.system() == "Windows":
        return os.path.join(os.environ.get("ProgramFiles", "C:\\Program Files"), ".velvet-library")
    return "/usr/lib/.velvet-library"

def install_dependency(language, command, dep_name):
    """Install a dependency in an isolated environment."""
    library_path = get_library_path()
    env_path = os.path.join(library_path, language, dep_name.replace("/", "_"))
    os.makedirs(env_path, exist_ok=True)

    try:
        if language == "python":
            subprocess.run(f"python -m venv {env_path}", shell=True, check=True)
            pip_path = f"{env_path}/Scripts/pip" if platform.system() == "Windows" else f"{env_path}/bin/pip"
            subprocess.run(f"{pip_path} install {command}", shell=True, check=True)
        elif language == "ruby":
            gemfile = os.path.join(env_path, "Gemfile")
            with open(gemfile, "w") as f:
                f.write(f"source 'https://rubygems.org'\ngem '{command}'")
            subprocess.run(f"bundle install --path {env_path} --gemfile={gemfile}", shell=True, check=True)
        elif language == "rust":
            subprocess.run(f"cargo install --root {env_path} {command}", shell=True, check=True)
        elif language == "go":
            subprocess.run(f"go get -d {command}", shell=True, cwd=env_path, check=True)
        elif language == "crystal":
            subprocess.run(f"crystal deps install {command}", shell=True, cwd=env_path, check=True)
        elif language == "elixir":
            subprocess.run(f"mix deps.get {command}", shell=True, cwd=env_path, check=True)
        elif language == "java":
            subprocess.run(f"mvn install -Ddir={env_path} {command}", shell=True, check=True)
        elif language in ["javascript", "typescript"]:
            subprocess.run(f"npm install --prefix {env_path} {command}", shell=True, check=True)
        elif language == "kotlin":
            subprocess.run(f"gradle install -Ddir={env_path} {command}", shell=True, check=True)
        else:
            raise ValueError(f"Unsupported language: {language}")
        print(f"Installed {command} for {language} in {env_path}")
    except subprocess.CalledProcessError as e:
        print(f"Error installing {command} for {language}: {e}")
        raise
