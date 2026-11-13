#!/usr/bin/env python3
"""strict-check.py <stats.jsonl>: print one line per front-end job of
kiln's statistics log whose object was not written by hod (backend other
than `hod`), naming the backend, hod's reason and the opt triggers; print
nothing when every object job went to hod (wrap.sh's strict mode)."""
import json
import sys

for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    try:
        r = json.loads(line)
    except json.JSONDecodeError:
        continue
    if r.get("output_kind") != "obj" or r.get("backend") == "hod":
        continue
    trig = ",".join(t for t in r.get("triggers") or [] if t != "alwaysinline") or "-"
    print(f"{r.get('input')}: backend {r.get('backend')}, reason {r.get('hod_reason')}, triggers {trig}")
