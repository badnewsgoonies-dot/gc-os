#!/usr/bin/env python3
"""Build a TWA APK for a game using Bubblewrap CLI."""
import pexpect
import sys
import os

game = sys.argv[1] if len(sys.argv) > 1 else "diner-dash"
work_dir = f"/home/geni/gc-project/apks/{game}-build"
os.makedirs(work_dir, exist_ok=True)
os.chdir(work_dir)

config_src = f"/home/geni/gc-project/apks/{game}/twa-manifest.json"
if os.path.exists(config_src):
    os.system(f"cp {config_src} .")
    print(f"Using config: {config_src}")

env = os.environ.copy()
env["PATH"] = "/home/geni/.npm-global/bin:" + env.get("PATH", "")

child = pexpect.spawn(
    "bubblewrap", ["build"],
    timeout=300, encoding="utf-8", env=env
)

try:
    idx = child.expect(["Y/n", "Key Store:", pexpect.EOF, pexpect.TIMEOUT], timeout=120)
    if idx == 0:
        child.sendline("n")
        idx2 = child.expect(["Key Store:", pexpect.EOF, pexpect.TIMEOUT], timeout=120)
        if idx2 == 0:
            child.sendline("gctools123")
            child.expect("alias:", timeout=30)
            child.sendline("gctools123")
            child.expect(pexpect.EOF, timeout=300)
    elif idx == 1:
        child.sendline("gctools123")
        child.expect("alias:", timeout=30)
        child.sendline("gctools123")
        child.expect(pexpect.EOF, timeout=300)

    output = child.before or ""
    print(output[-500:] if len(output) > 500 else output)
    print(f"\nBUILD COMPLETE for {game}")

    # Check for APK
    for f in os.listdir(work_dir):
        if f.endswith(".apk") or f.endswith(".aab"):
            size = os.path.getsize(os.path.join(work_dir, f))
            print(f"  Output: {f} ({size} bytes)")

except Exception as e:
    print(f"Error: {e}")
    if child.before:
        print(f"Output: {child.before[-300:]}")

child.close()
