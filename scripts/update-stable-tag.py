#!/usr/bin/env python3
import argparse
import subprocess
import sys

def run_cmd(cmd):
    """Run command and exit on failure."""
    result = subprocess.run(cmd, shell=True, check=False)
    if result.returncode != 0:
        sys.exit(result.returncode)

def main():
    parser = argparse.ArgumentParser(description='Update stable tag')
    parser.add_argument('--commit', help='Commit hash (default: latest master)')
    args = parser.parse_args()
    
    # Get target commit
    if args.commit:
        commit = args.commit
    else:
        result = subprocess.run("git rev-parse origin/master", shell=True, 
                              capture_output=True, text=True, check=False)
        if result.returncode != 0:
            print("Error: Could not find origin/master or origin/main")
            sys.exit(1)
        commit = result.stdout.strip()
    
    print(f"Updating stable tag to {commit[:7]}")

    run_cmd("git fetch --tags")
    run_cmd("git push origin :refs/tags/stable || true")  # Delete remote (ignore errors)
    run_cmd("git tag -d stable || true")  # Delete local (ignore errors)
    run_cmd(f"git tag stable {commit}")
    run_cmd("git push origin stable")
    
    print("✅ Done")

if __name__ == "__main__":
    main()
