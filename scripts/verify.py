"""Execute real checks; unsupported/missing checks fail closed, never print a fake pass."""
import argparse
import os
import pathlib
import shutil
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]


def run(command, required=None):
    if required and not (ROOT / required).exists():
        raise RuntimeError(f"NOT IMPLEMENTED: {required}")
    executable = shutil.which(command[0])
    if not executable:
        raise RuntimeError(f"MISSING TOOL: {command[0]}")
    if os.name == "nt" and executable.lower().endswith((".cmd", ".bat")):
        command = [os.environ.get("COMSPEC", "cmd.exe"), "/d", "/c", executable, *command[1:]]
    completed = subprocess.run(command, cwd=ROOT)
    if completed.returncode:
        raise subprocess.CalledProcessError(completed.returncode, command)


def js_package(name):
    base = ROOT / "packages" / name
    if not (base / "tsconfig.json").is_file():
        raise RuntimeError(f"NOT IMPLEMENTED: packages/{name}")
    run(["tsc", "-p", f"packages/{name}/tsconfig.json"])
    files = sorted((base / "tests").glob("*.test.mjs"))
    if not files:
        raise RuntimeError(f"NO TESTS: packages/{name}")
    run(["node", "--experimental-strip-types", "--test", *map(str, files)])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--profile", choices=["core", "adapters", "selectors", "sdk-typescript", "sdk", "javascript", "ui", "release", "all"], default="core")
    parser.add_argument("--adapter")
    args = parser.parse_args()
    if args.profile in ("core", "all", "release"):
        run(["cargo", "test", "--workspace", "--all-targets"], "Cargo.toml")
    if args.profile in ("adapters", "all", "release"):
        command = ["cargo", "test", "-p", "lai-adapters"]
        if args.adapter:
            command.append(args.adapter)
        run(command, "crates/lai-adapters/Cargo.toml")
    if args.profile in ("selectors", "javascript", "all", "release"):
        js_package("selector-core")
    if args.profile in ("sdk-typescript", "sdk", "javascript", "all", "release"):
        js_package("sdk-typescript")
    if args.profile in ("sdk", "all", "release"):
        run([sys.executable, "-m", "unittest", "discover", "-s", "packages/sdk-python/tests", "-v"], "packages/sdk-python/pyproject.toml")
    if args.profile in ("ui", "all", "release"):
        run([sys.executable, "scripts/test_demo.py"], "scripts/test_demo.py")
    if args.profile == "release":
        run(["cargo", "fmt", "--all", "--check"])
        run(["cargo", "clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"])
        run([sys.executable, "scripts/check_generated.py"], "scripts/check_generated.py")
        run([sys.executable, "scripts/package_smoke.py"], "scripts/package_smoke.py")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, subprocess.CalledProcessError) as exc:
        print(str(exc), file=sys.stderr)
        raise SystemExit(getattr(exc, "returncode", 2))
