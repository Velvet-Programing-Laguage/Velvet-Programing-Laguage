import subprocess

def execute_velvet(script_path):
    """Execute a Velvet script using the Rust core."""
    try:
        result = subprocess.run(["./velvet_core", script_path], capture_output=True, text=True, check=True)
        print(result.stdout)
    except subprocess.CalledProcessError as e:
        print(f"Error executing Velvet script: {e.stderr}")
        raise
