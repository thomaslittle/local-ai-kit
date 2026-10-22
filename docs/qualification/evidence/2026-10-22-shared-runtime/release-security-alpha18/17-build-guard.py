"""Task-owned, serialized build guard. No global configuration or cleanup."""
from pathlib import Path
import argparse
import json
import os
import shutil
import subprocess
import sys
import time
import psutil
import msvcrt

ROOT = Path(__file__).resolve().parents[1]
TASK = Path("D:/lai-task-7a4b270a-20261022").resolve()
THREAD = "7a4b270a-3512-4710-ad7e-3b4751818241"
GIB = 1024 ** 3
owner = json.loads((TASK / "owner.json").read_text(encoding="utf-8-sig"))
assert owner["threadId"] == THREAD and Path(owner["workspace"]).resolve() == ROOT
for p in [TASK, TASK / "target", TASK / "tmp", TASK / "guard-receipts"]:
    assert not p.is_symlink()
    p.mkdir(exist_ok=True)

def active_bytes():
    total = 0
    for parent in [TASK / "target", TASK / "tmp"]:
        for directory, dirs, names in os.walk(parent, followlinks=False):
            dirs[:] = [d for d in dirs if not (Path(directory) / d).is_symlink()]
            for name in names:
                try:
                    p = Path(directory) / name
                    if not p.is_symlink():
                        total += p.stat().st_size
                except FileNotFoundError:
                    pass
    return total

def pressure():
    used = active_bytes()
    c_free, d_free = shutil.disk_usage("C:/").free, shutil.disk_usage("D:/").free
    reason = None
    if used >= 11 * GIB:
        reason = "11 GiB stop threshold leaves 1 GiB below the 12 GiB active budget"
    elif c_free < 16 * GIB:
        reason = "C free space below 16 GiB shared reserve"
    elif d_free < 64 * GIB:
        reason = "D free space below 64 GiB shared reserve"
    return used, c_free, d_free, reason

def stop_owned(process):
    # Only this invocation's captured process tree is eligible for cancellation.
    try:
        parent = psutil.Process(process.pid)
        owned = [(p.pid, p.create_time()) for p in parent.children(recursive=True)]
        owned.append((parent.pid, parent.create_time()))
    except psutil.Error:
        return
    survivors = []
    for pid, created in reversed(owned):
        try:
            p = psutil.Process(pid)
            if p.create_time() == created:
                p.terminate()
                survivors.append(p)
        except psutil.Error:
            pass
    _, alive = psutil.wait_procs(survivors, timeout=3)
    for p in alive:
        try:
            if p.create_time() == next(t for pid, t in owned if pid == p.pid):
                p.kill()
        except (psutil.Error, StopIteration):
            pass

parser = argparse.ArgumentParser()
parser.add_argument("--name", required=True)
parser.add_argument("command", nargs=argparse.REMAINDER)
args = parser.parse_args()
assert args.name.replace("-", "").replace("_", "").isalnum()
command = args.command[1:] if args.command[:1] == ["--"] else args.command
assert command
lock = (TASK / "build.lock").open("a+b")
lock.seek(0)
if not lock.read(1):
    lock.write(b"0")
    lock.flush()
lock.seek(0)
try:
    msvcrt.locking(lock.fileno(), msvcrt.LK_NBLCK, 1)
except OSError:
    raise SystemExit("Another owned build holds the task lock; do not start a target variant")

used, c_free, d_free, reason = pressure()
if reason:
    raise SystemExit(reason)
env = {**os.environ, "CARGO_TARGET_DIR": str(TASK / "target"),
       "RUSTC_WRAPPER": "", "RUSTC_WORKSPACE_WRAPPER": "",
       "CARGO_INCREMENTAL": "0", "CARGO_BUILD_JOBS": "2",
       "CARGO_PROFILE_DEV_DEBUG": "0", "CARGO_PROFILE_TEST_DEBUG": "0",
       "LAI_TEST_PYTHON": str(ROOT / ".venv-build/Scripts/python.exe"),
       "LAI_NATIVE_BINARY": str(TASK / "target/debug/local-ai-kit.exe"),
       "TMP": str(TASK / "tmp"), "TEMP": str(TASK / "tmp"),
       "TMPDIR": str(TASK / "tmp"), "PYTHONUTF8": "1"}
executable = shutil.which(command[0]) or command[0]
launch = [executable, *command[1:]]
if executable.lower().endswith((".cmd", ".bat")):
    launch = [os.environ.get("COMSPEC", "cmd.exe"), "/d", "/c", *launch]
started = time.time()
peak = used
process = subprocess.Popen(launch, cwd=ROOT, env=env)
while process.poll() is None:
    used, c_free, d_free, reason = pressure()
    peak = max(peak, used)
    if reason:
        stop_owned(process)
        break
    time.sleep(1)
code = process.wait()
used, c_free, d_free, final_reason = pressure()
peak = max(peak, used)
receipt = {"name": args.name, "threadId": THREAD, "command": command,
           "sourceCommit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
           "exitCode": code, "seconds": round(time.time() - started, 3),
           "target": str(TASK / "target"), "activeBudgetBytes": 12 * GIB,
           "stopThresholdBytes": 11 * GIB, "peakObservedActiveBytes": peak,
           "finalActiveBytes": used, "cFreeBytes": c_free, "dFreeBytes": d_free,
           "pressureStop": reason, "finalPressure": final_reason,
           "jobs": 2, "debugSymbols": False, "incremental": False,
           "rustcWrapper": "disabled per process; shared cache untouched",
           "scope": "Owned target and temporary files only; fixed retained-symbol archive excluded from active budget"}
destination = TASK / "guard-receipts" / (args.name + ".json")
if destination.exists():
    raise SystemExit("Receipt already exists; use a new attempt name")
destination.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
print(json.dumps(receipt), flush=True)
raise SystemExit(125 if reason or final_reason else code)
