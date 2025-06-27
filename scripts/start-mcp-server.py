#!/usr/bin/env python3
import argparse
import os
import shutil
import subprocess
import sys
import tempfile
import re
from pathlib import Path

def run_command(cmd, cwd=None, capture_output=True):
    """Run a command and return the result."""
    try:
        result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=capture_output, 
                              text=True, check=False)
        return result.returncode == 0, result.stdout.strip() if capture_output else ""
    except Exception as e:
        print(f"Error running command '{cmd}': {e}")
        return False, ""

def get_remote_commit(repo_url, tag):
    """Get remote commit hash for tag without cloning."""
    success, output = run_command(f'git ls-remote --tags "{repo_url}" "refs/tags/{tag}"')
    if success and output:
        try:
            commit_hash = output.split()[0][:7]
            return commit_hash
        except (IndexError, AttributeError):
            pass
    return None

def get_binary_version(binary_path):
    """Get existing binary version."""
    if not os.path.exists(binary_path):
        return None
    
    success, output = run_command(f'"{binary_path}" --version')
    return output if success else None

def main():
    parser = argparse.ArgumentParser(description='Install and start rust-mcp-server')
    parser.add_argument('--install-folder', default='./rust-mcp-server', 
                       help='Folder to install the executable (default: ./rust-mcp-server)')
    parser.add_argument('--tag', default='stable', 
                       help='Git tag to checkout (default: stable)')
    parser.add_argument('--keep-temp', action='store_true',
                       help='Keep temp directory for faster subsequent builds')
    parser.add_argument('server_args', nargs='*', 
                       help='Arguments to pass to the server')
    
    args = parser.parse_args()
    
    repo_url = "https://github.com/Vaiz/rust-mcp-server.git"
    install_folder = Path(args.install_folder).resolve()
    
    # Platform-specific executable name
    exe_name = "rust-mcp-server.exe" if os.name == 'nt' else "rust-mcp-server"
    binary_path = install_folder / exe_name
    
    # Use platform-specific temp directory
    temp_base = Path(tempfile.gettempdir()) / "rust-mcp-server-build"
    print(f"Using temp directory: {temp_base}")
    
    # Check if rebuild is needed
    remote_commit = get_remote_commit(repo_url, args.tag)
    print(f"Remote commit for tag '{args.tag}': {remote_commit}")
    
    existing_version = get_binary_version(binary_path)
    print(f"Existing binary version: {existing_version}")
    
    needs_build = True
    
    if existing_version and remote_commit:
        # Extract commit hash from version string (format: "rust-mcp-server 0.1.0.d7c5bac")
        match = re.search(r'\.([a-f0-9]+)$', existing_version)
        if match:
            existing_commit = match.group(1)
            if existing_commit == remote_commit:
                print(f"Binary up-to-date (commit: {remote_commit})")
                needs_build = False
            else:
                print(f"Binary outdated ({existing_commit} → {remote_commit})")
    
    # Clone/update and build if needed
    if needs_build:
        install_folder.mkdir(parents=True, exist_ok=True)

        if temp_base.exists():
            print("Updating existing temp directory...")
            success, _ = run_command("git fetch --all --tags", cwd=temp_base)
            if success:
                run_command("git reset --hard HEAD", cwd=temp_base)
        else:
            print("Cloning to temp directory...")
            success, _ = run_command(f'git clone "{repo_url}" "{temp_base}"')
            if not success:
                print("Failed to clone repository")
                sys.exit(1)
        
        success, _ = run_command(f"git checkout {args.tag}", cwd=temp_base)
        if not success:
            print(f"Failed to checkout tag: {args.tag}")
            sys.exit(1)
        
        print("Building...")
        cargo_cmd = f'cargo build --release --manifest-path "{temp_base / "Cargo.toml"}"'
        success, _ = run_command(cargo_cmd, capture_output=False)
        if not success:
            print("Build failed")
            sys.exit(1)
        
        src_binary = temp_base / "target" / "release" / exe_name
        shutil.copy2(src_binary, binary_path)
        
        # Clean up temp directory unless keeping it
        if not args.keep_temp:
            shutil.rmtree(temp_base, ignore_errors=True)
        
        print(f"Installed to: {binary_path}")
    
    # Start server
    print("Starting server...")
    cmd = [str(binary_path)] + args.server_args
    try:
        subprocess.run(cmd, check=False)
    except KeyboardInterrupt:
        print("\nServer stopped by user")
    except Exception as e:
        print(f"Error starting server: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
