#!/usr/bin/env python3
"""oss-reduce.py: the smallest reproducer of each refusal the lane keeps
(WP-191, spec/landed/harness/WP-191.md).

  oss-reduce.py <findings dir> [--project P]... [--hod H] [--timeout S]

For every <findings dir>/<project>/<reason>/ holding a kept module
(`1.ll`, from `oss-report.py collect`) and no `min.ll` yet (only the
projects named, with `--project`): the refused
function is cut out of the module (`llvm-extract`, its callees left as
declarations), then `llvm-reduce` shrinks that while hod still refuses
the module with <reason> (hod's census, HOD_CENSUS=1, read from its
coverage log: the reason need not be the first). The result is
`min.ll` beside the module, with a line in the README. A refusal named
with no function (a module-level construct, or `attr:<a>`: the functions
whose attribute group holds `<a>`) starts from the whole module.

hod is $HOD_OSS_HOD, else $CARGO_TARGET_DIR/release/hod; the LLVM tools
are LLVM 22's (kiln writes LLVM 22 text). Scratch files go under
<findings dir>/.reduce-<pid>/ and are removed.
"""

import argparse
import glob
import os
import re
import shutil
import subprocess
import sys

sys.dont_write_bytecode = True
LLVM = os.environ.get("HOD_OSS_LLVM_BIN", "/usr/lib/llvm-22/bin")
MAX_REDUCE_BYTES = 16 * 1024 * 1024
# A kept module larger than this is replaced by its reduction.
KEEP_WHOLE_BYTES = 5 * 1024 * 1024
DIAG_RE = re.compile(r"in function '((?:[^'\\]|\\.)*)'")

# The interestingness test llvm-reduce runs: hod refuses the file with
# the reason (anywhere in its census).
TEST = r'''#!/usr/bin/env python3
import os, subprocess, sys
hod, reason, log = sys.argv[1], sys.argv[2], sys.argv[3] + ".%d" % os.getpid()
env = dict(os.environ, HOD_CENSUS="1", HOD_COVERAGE_LOG=log)
try:
    with open(sys.argv[4], "rb") as f:
        r = subprocess.run([hod, "--strict", "-O0", "-relocation-model=pic", "-filetype=obj", "-o", os.devnull, "-"],
                           stdin=f, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env, timeout=60)
except subprocess.TimeoutExpired:
    sys.exit(1)
names = set()
try:
    with open(log) as f:
        for line in f:
            fs = line.rstrip("\n").split("\t")
            names.add(fs[1])
            if len(fs) >= 11 and fs[10] != "-":
                names.update(fs[10].split(","))
    os.unlink(log)
except OSError:
    pass
sys.exit(0 if r.returncode == 2 and reason in names else 1)
'''


def function_of(d):
    """The refused function named in 1.cmd's diagnostic line, if any."""
    try:
        with open(os.path.join(d, "1.cmd"), encoding="utf-8", errors="replace") as f:
            for line in f:
                if line.startswith("# hod: error:"):
                    m = DIAG_RE.search(line)
                    return m.group(1) if m else None
    except OSError:
        pass
    # A census reason (never the module's first): the README names the
    # functions that hold it (oss-report.py collect); take the first.
    try:
        with open(os.path.join(d, "README.md"), encoding="utf-8", errors="replace") as f:
            m = re.search(r" in `([^`, ]+)", f.readline())
            return m.group(1) if m else None
    except OSError:
        return None


def interesting(test, hod, reason, scratch, path):
    return subprocess.run([sys.executable, test, hod, reason, os.path.join(scratch, "cov"), path]).returncode == 0


def reduce_one(d, reason, hod, scratch, timeout):
    test = os.path.join(scratch, "test.py")
    src = os.path.join(d, "1.ll")
    start = src
    fn = function_of(d)
    if fn:
        cut = os.path.join(scratch, "cut.ll")
        r = subprocess.run([f"{LLVM}/llvm-extract", "-S", f"--func={fn}", src, "-o", cut], capture_output=True, text=True)
        if r.returncode == 0 and interesting(test, hod, reason, scratch, cut):
            start = cut
    if not interesting(test, hod, reason, scratch, start):
        return None, "hod no longer refuses the kept module with this reason"
    # llvm-reduce takes hours over a whole module of tens of MB (highway's
    # x86-64 tests, WP-200): such a module is kept unreduced.
    if os.path.getsize(start) > MAX_REDUCE_BYTES:
        return None, f"not reduced: {os.path.getsize(start)} bytes, no single function to extract"
    out = os.path.join(scratch, "min.ll")
    # llvm-reduce passes the candidate as the test's last argument. Its
    # `attributes` pass is skipped: without a function's target-features
    # hod refuses a vector intrinsic for the missing feature, a different
    # refusal under the same name (WP-200: reductions of AVX2 and AVX-512
    # calls that hod builds in the real module).
    cmd = [f"{LLVM}/llvm-reduce", "-j", "1", "--skip-delta-passes=attributes", f"--test={sys.executable}", f"--test-arg={test}", f"--test-arg={hod}",
           f"--test-arg={reason}", f"--test-arg={os.path.join(scratch, 'cov')}", "-o", out, start]
    try:
        subprocess.run(cmd, capture_output=True, timeout=timeout, cwd=scratch)
    except subprocess.TimeoutExpired:
        pass
    if os.path.exists(out) and interesting(test, hod, reason, scratch, out):
        return out, "llvm-reduce"
    return start, "extracted, not reduced" if start != src else "the whole module"


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("findings")
    ap.add_argument("--hod")
    ap.add_argument("--project", action="append", default=[])
    ap.add_argument("--timeout", type=int, default=600)
    a = ap.parse_args()
    hod = a.hod or os.environ.get("HOD_OSS_HOD") or os.path.join(os.environ.get("CARGO_TARGET_DIR", ""), "release", "hod")
    if not os.access(hod, os.X_OK):
        sys.exit(f"oss-reduce: no hod at {hod} (--hod or HOD_OSS_HOD)")
    hod = os.path.abspath(hod)
    scratch = os.path.join(a.findings, f".reduce-{os.getpid()}")
    os.makedirs(scratch)
    try:
        with open(os.path.join(scratch, "test.py"), "w") as f:
            f.write(TEST)
        dirs = [d for p in (a.project or ["*"]) for d in glob.glob(os.path.join(a.findings, p, "*"))]
        for d in sorted(dirs):
            if not os.path.isfile(os.path.join(d, "1.ll")) or os.path.exists(os.path.join(d, "min.ll")):
                continue
            reason = os.path.basename(d)
            # The directory name is the reason made safe (projects.SAFE_RE);
            # the README's first line holds it exactly.
            try:
                with open(os.path.join(d, "README.md"), encoding="utf-8") as f:
                    m = re.search(r"`([^`]+)`", f.readline())
                    reason = m.group(1) if m else reason
            except OSError:
                pass
            got, how = reduce_one(d, reason, hod, scratch, a.timeout)
            if got:
                shutil.copyfile(got, os.path.join(d, "min.ll"))
                lines = sum(1 for _ in open(got, encoding="utf-8", errors="replace"))
                note = f"min.ll: `{reason}`, {lines} lines ({how}; `hod --strict -O0 -filetype=obj -o /dev/null min.ll` refuses it)"
                # A large module is kept only as its reduction (WP-200); 1.cmd
                # says how to make it again.
                big = os.path.getsize(os.path.join(d, "1.ll"))
                if big > KEEP_WHOLE_BYTES:
                    os.unlink(os.path.join(d, "1.ll"))
                    note += f"; 1.ll ({big} bytes) dropped, 1.cmd rebuilds it"
            else:
                note = f"min.ll: none ({how})"
            with open(os.path.join(d, "README.md"), "a", encoding="utf-8") as f:
                f.write(note + "\n")
            print(f"oss-reduce: {os.path.relpath(d, a.findings)}: {note}")
    finally:
        shutil.rmtree(scratch, ignore_errors=True)


if __name__ == "__main__":
    main()
