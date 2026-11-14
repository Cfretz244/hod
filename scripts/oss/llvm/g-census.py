#!/usr/bin/env python3
"""A compile-only `-g` census of a sample of a built tree's TUs (WP-178).

g-census.py <tree> <out dir> [--every N] [--offset K] [--target T] [-j J]
            [--objects LIST]

<tree> is a hod tree ($HOD_OSS_ROOT/llvm-hod or llvm-x64-hod) whose
build/compile_commands.json names the commands. The sample is every N-th
C and C++ TU (from K; sorted by object path) of the objects of LIST
(default <tree>/logs/strict-objects.txt). Each TU's command is run again
through kiln's logging wrapper (kiln-cxx, kiln-cc) with `-g` added (and
`--target=T` for a cross tree), KILN_BACKEND=hod (strict off: a decline
falls back to llc), HOD_CENSUS=1, and every log in <out dir>: tu.jsonl,
stats.jsonl (kiln's `backend` per job), coverage.tsv (hod's reasons). The
object goes to <out dir>/tmp and is deleted at once (never a `-g` tree on
disk); dependency outputs are dropped. Then it prints the census: TUs,
the count by backend, the refusal names (field 2 and every name of field
11) with example TUs, and writes <out dir>/census.json in census.py's
--json form (reason -> [{output, source, argv, cwd}]) for save-modules.py.
"""

import argparse
import collections
import concurrent.futures
import json
import os
import shlex
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from census import split_reasons  # noqa: E402


def command_for(e, out_obj, target):
    argv = shlex.split(e["command"]) if "command" in e else list(e["arguments"])
    lang = "cc" if argv[0].endswith("-cc") else "cxx"
    rest, skip = [], False
    for a in argv[1:]:
        if skip:
            skip = False
            continue
        if a in ("-o", "-MF", "-MT", "-MQ"):
            skip = True
            continue
        if a in ("-MD", "-MMD"):
            continue
        rest.append(a)
    pre = ["-g"] + ([f"--target={target}"] if target else [])
    return lang, pre + rest + ["-o", out_obj]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("tree")
    ap.add_argument("out")
    ap.add_argument("--every", type=int, default=10)
    ap.add_argument("--offset", type=int, default=3)
    ap.add_argument("--target")
    ap.add_argument("-j", type=int, default=6)
    ap.add_argument("--objects")
    a = ap.parse_args()
    tree, out = os.path.abspath(a.tree), os.path.abspath(a.out)
    os.makedirs(os.path.join(out, "tmp"), exist_ok=True)
    build = os.path.join(tree, "build")
    objs = set(open(a.objects or os.path.join(tree, "logs", "strict-objects.txt")).read().split())
    cc = {}
    for e in json.load(open(os.path.join(build, "compile_commands.json"))):
        rel = os.path.relpath(os.path.normpath(os.path.join(e["directory"], e["output"])), build)
        if rel in objs and e["file"].endswith((".cpp", ".cc", ".c")):
            cc[rel] = e
    sample = sorted(cc)[a.offset::a.every]
    kiln = os.environ["HOD_OSS_KILN"]
    scripts = os.environ["HOD_OSS_KILN_SCRIPTS"]
    env = dict(os.environ, KILN_OSS_LOG=os.path.join(out, "tu.jsonl"), KILN_OSS_PHASE="build",
               KILN_STATS_LOG=os.path.join(out, "stats.jsonl"), KILN_OSS_CXX=f"{kiln}/kiln++",
               KILN_OSS_CC=f"{kiln}/kiln", KILN_BACKEND="hod", KILN_HOD=os.path.join(tree, "bin", "hod"),
               HOD_COVERAGE_LOG=os.path.join(out, "coverage.tsv"), HOD_CENSUS="1")
    for k in ("KILN_HOD_STRICT", "HOD_OSS_STRICT"):
        env.pop(k, None)

    def one(i_rel):
        i, rel = i_rel
        e = cc[rel]
        tmp = os.path.join(out, "tmp", f"{i}.o")
        lang, args = command_for(e, tmp, a.target)
        r = subprocess.run([f"{scripts}/kiln-{lang}"] + args, cwd=e["directory"], env=env,
                           capture_output=True, text=True)
        size = os.path.getsize(tmp) if os.path.exists(tmp) else -1
        if os.path.exists(tmp):
            os.remove(tmp)
        return rel, r.returncode, size, r.stderr[-400:]

    results = {}
    with concurrent.futures.ThreadPoolExecutor(a.j) as ex:
        for rel, rc, size, err in ex.map(one, enumerate(sample)):
            results[rel] = (rc, size)
            if rc != 0:
                print(f"g-census: {rel}: rc {rc}: {err}", file=sys.stderr)
    # The census.
    stats = [json.loads(l) for l in open(os.path.join(out, "stats.jsonl"))]
    objstats = [r for r in stats if r.get("output_kind") == "obj"]
    back = collections.Counter(r.get("backend") for r in objstats)
    trig = collections.Counter(",".join(t for t in r.get("triggers") or [] if t != "alwaysinline") or "-"
                               for r in objstats if r.get("backend") == "llvm")
    tus = {}
    for l in open(os.path.join(out, "tu.jsonl")):
        r = json.loads(l)
        if r.get("output"):
            tus[os.path.normpath(os.path.join(r["cwd"], r["output"]))] = r
    groups, every = collections.defaultdict(list), collections.Counter()
    first = collections.Counter()
    if os.path.exists(os.path.join(out, "coverage.tsv")):
        for l in open(os.path.join(out, "coverage.tsv")):
            f = l.rstrip("\n").split("\t")
            if f[0] == "hod":
                continue
            key = os.path.normpath(f[4]) if os.path.isabs(f[4]) else None
            t = tus.get(key) if key else None
            if t is None:
                t = next((v for k, v in tus.items() if k.endswith("/" + os.path.basename(f[4]))), None)
            first[f[1]] += 1
            names = [f[1]] if f[10] == "-" else split_reasons(f[10])
            for n in names:
                every[n] += 1
                if t:
                    groups[n].append({"output": t["output"], "source": t.get("source"), "argv": t["argv"], "cwd": t["cwd"]})
    json.dump({"census": groups}, open(os.path.join(out, "census.json"), "w"), indent=1)
    print(f"sample: {len(sample)} TUs (every {a.every}th from {a.offset} of {len(cc)} C/C++ objects), -g, "
          f"compile rc 0: {sum(1 for rc, _ in results.values() if rc == 0)}")
    print(f"kiln statistics, object jobs: {len(objstats)}: " + ", ".join(f"{k} {v}" for k, v in back.most_common()))
    if trig:
        print("  routed to llvm by kiln (triggers): " + ", ".join(f"{k} {v}" for k, v in trig.most_common()))
    print("hod refusals, field 2: " + (", ".join(f"{k} {v}" for k, v in first.most_common()) or "none"))
    print("hod refusals, every name (field 11): " + (", ".join(f"{k} {v}" for k, v in every.most_common()) or "none"))
    for n, ents in sorted(groups.items()):
        print(f"  {n}: e.g. {', '.join(e['source'] or e['output'] for e in ents[:3])}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
