"""Clodo command-line interface."""

import os
import subprocess
import sys


def run():
    """
    Entrypoint for running the clodo command-line interface.

    This function is responsible for running the compiled Rust binary with the
    provided command-line arguments. It handles some basic error cases, such as
    the binary not being found after installation.
    """
    # Path to the compiled Rust binary (adjusted during installation)
    binary_path = os.path.join(os.path.dirname(__file__), "clodo")
    if sys.platform.startswith("win"):
        binary_path += ".exe"

    # Run the Rust binary with the provided arguments
    try:
        result = subprocess.run([binary_path] + sys.argv[1:], check=True)
        sys.exit(result.returncode)
    except subprocess.CalledProcessError as e:
        sys.exit(e.returncode)
    except FileNotFoundError:
        print("Error: clodo binary not found. Ensure it was built correctly.")
        sys.exit(1)
