#!/usr/bin/env python3
"""Compile time of the two trees (WP-178): compare-time.py <root>

For the TUs compiled in both llvm-hod and llvm-llc (joined by the object
path relative to the build directory; the last compile of each), the sum
of kiln's wall and CPU time per TU (kiln's wrapper records the child's
`wall_s`, `cpu_s` and `max_rss_kb`: front end plus back end) and the peak
RSS, plus kiln's own back-end time from the statistics logs
(`children_ms`: hod, opt, llc), summed. A side note, not a benchmark: the
host is shared and loaded, and the two trees were built at different
times."""
import json
import os
import sys


def tus(tree):
    out = {}
    for line in open(os.path.join(tree, "logs", "tu.jsonl"), encoding="utf-8", errors="replace"):
        r = json.loads(line)
        if r.get("phase") != "build" or r.get("source") in ("link", "none") or not r.get("output"):
            continue
        key = os.path.relpath(os.path.normpath(os.path.join(r["cwd"], r["output"])), os.path.join(tree, "build"))
        out[key] = r
    return out


def children(tree):
    tot = {}
    last = {}
    for line in open(os.path.join(tree, "logs", "stats.jsonl"), encoding="utf-8", errors="replace"):
        r = json.loads(line)
        if r.get("output_kind") == "obj" and r.get("input"):
            last[r["input"]] = r
    for r in last.values():
        for k, v in (r.get("children_ms") or {}).items():
            tot[k] = tot.get(k, 0.0) + v
        tot["frontend"] = tot.get("frontend", 0.0) + (r.get("frontend_ms") or 0.0)
    return len(last), tot


root = sys.argv[1]
h, c = tus(os.path.join(root, "llvm-hod")), tus(os.path.join(root, "llvm-llc"))
both = sorted(set(h) & set(c))
for name, d in (("hod tree", h), ("control", c)):
    wall = sum(d[k]["wall_s"] for k in both)
    cpu = sum(d[k]["cpu_s"] for k in both)
    rss = max(d[k]["max_rss_kb"] for k in both) / 1024
    print(f"{name}: {len(both)} TUs, wall {wall:.0f} s, cpu {cpu:.0f} s, peak RSS {rss:.0f} MiB")
for name in ("llvm-hod", "llvm-llc"):
    n, tot = children(os.path.join(root, name))
    print(f"{name} statistics log ({n} object jobs): " + ", ".join(f"{k} {v / 1000:.0f} s" for k, v in sorted(tot.items())))
