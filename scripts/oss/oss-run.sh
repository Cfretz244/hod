#!/usr/bin/env bash
# oss-run.sh <project>... [--flavours "O0 O0g"] [--modes "llc hod strict"] [--keep]
#
# The lane's whole procedure for each project (spec/harness.md "Real
# projects"), one build at a time: for each flavour, the hybrid build
# (`--mode hod`, the census), the control (`--mode llc`), and the strict
# build (`--mode strict`) when the hybrid build fell back nowhere; then the
# refused modules are kept as findings (oss-report.py collect), the build
# trees pruned to their logs and reports (unless --keep), and the census
# page rewritten (<findings>/CENSUS.md). `--modes` runs only the modes
# named (strict still only after an existing hybrid report with no
# fallback). Source scripts/env.sh first.
#
# It runs from a copy of scripts/oss taken when it starts
# (<root>/lane-<pid>/, removed at exit), and bash parses this file whole
# before running it (the braces): editing the lane while a run is going
# never changes the running one.
set -uo pipefail
{
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
: "${CARGO_TARGET_DIR:?oss-run: source scripts/env.sh first}"
BROOT="${HOD_BUILD_ROOT:-$(dirname "$CARGO_TARGET_DIR")}"
OSS="${HOD_OSS_ROOT:-$BROOT/oss}"
FINDINGS="${HOD_OSS_FINDINGS:-$BROOT/findings/oss}"
FLAVOURS="O0 O0g" MODES="llc hod strict" KEEP="" PROJECTS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --flavours) FLAVOURS="$2"; shift 2 ;;
    --modes) MODES="$2"; shift 2 ;;
    --keep) KEEP=1; shift ;;
    -*) echo "oss-run: unknown option $1" >&2; exit 2 ;;
    *) PROJECTS+=("$1"); shift ;;
  esac
done
[ ${#PROJECTS[@]} -gt 0 ] || { sed -n '2p' "$0" | sed 's/^# //' >&2; exit 2; }
mkdir -p "$FINDINGS" "$OSS"
SNAP="$OSS/lane-$$"
mkdir "$SNAP" && cp -p "$HERE"/oss-build.sh "$HERE"/oss-report.py "$HERE"/projects.py "$HERE"/projects.toml \
  "$HERE"/kiln-cxx "$HERE"/kiln-cc "$HERE"/hod-shim "$SNAP"/ || exit 2
trap 'find "$SNAP" -mindepth 1 -delete; rmdir "$SNAP"' EXIT
HERE="$SNAP"

fell_back() {  # the hybrid report's fallbacks: declined + failed (not offered is kiln's choice)
  python3 -c "import json,sys; c=json.load(open(sys.argv[1]))['hod'] or {}; b=c.get('backends',{}); print(b.get('hod-declined',0)+b.get('hod-failed',0))" "$1" 2>/dev/null || echo 1
}

worse() {  # tests passing in the control ($1) and not in $2
  python3 -c "
import json,sys
a=json.load(open(sys.argv[1])).get('test_status') or {}; b=json.load(open(sys.argv[2])).get('test_status') or {}
print(' '.join(sorted(t for t,s in a.items() if s=='Passed' and b.get(t)!='Passed')))" "$1" "$2" 2>/dev/null
}

# Build one mode; a hod or strict tree whose tests do worse than the
# control's keeps its objects (the finding), any other is pruned.
one() {
  local p="$1" fl="$2" m="$3"
  echo "oss-run: $p $fl $m ($(date -u +%H:%M:%S))"
  if [ "$m" = llc ]; then
    local prune=(--prune); [ -z "$KEEP" ] || prune=()
    "$HERE/oss-build.sh" "$p" --mode llc --flavour "$fl" "${prune[@]}"
    return
  fi
  "$HERE/oss-build.sh" "$p" --mode "$m" --flavour "$fl"
  local w; w="$(worse "$OSS/build/$p/$fl-llc/report.json" "$OSS/build/$p/$fl-$m/report.json")"
  if [ -n "$w" ]; then
    echo "oss-run: $p $fl $m: tests passing under llc and not here: $w (tree kept)"
    echo "$p $fl $m: $w" >> "$FINDINGS/TEST-REGRESSIONS.txt"
  elif [ -z "$KEEP" ]; then
    "$HERE/oss-build.sh" "$p" --mode "$m" --flavour "$fl" --prune-only
  fi
}

for p in "${PROJECTS[@]}"; do
  for fl in $FLAVOURS; do
    case " $MODES " in *" llc "*) one "$p" "$fl" llc ;; esac
    case " $MODES " in *" hod "*) one "$p" "$fl" hod ;; esac
    n="$(fell_back "$OSS/build/$p/$fl-hod/report.json")"
    if [ "$n" != 0 ]; then
      echo "oss-run: $p $fl: $n objects hod declined or failed in the hybrid build: no strict build"
    elif [ "${MODES#*strict}" != "$MODES" ]; then
      one "$p" "$fl" strict
    fi
    python3 "$HERE/oss-report.py" collect "$p"
    python3 "$HERE/oss-report.py" census --out "$FINDINGS/CENSUS.md" > /dev/null
  done
done
exit 0
}
