#!/bin/bash
# Regenerate $HOD_OSS_FINDINGS/LLVM-CENSUS.md (WP-178): the stage log of
# both trees, the census of the hod tree (census.py), the saved modules
# (save-modules.py) and the hand-written notes in
# $HOD_OSS_FINDINGS/llvm/notes.md. Also saves up to three modules per new
# refusal name.
set -euo pipefail
source "$(dirname "$0")/env.sh"
out="$HOD_OSS_FINDINGS/LLVM-CENSUS.md"
mkdir -p "$HOD_OSS_FINDINGS/llvm"
hod_tree="$HOD_OSS_ROOT/llvm-hod"
python3 "$HOD_OSS_SCRIPTS/census.py" "$hod_tree" --json "$hod_tree/logs/census.json" > "$hod_tree/logs/census.md"
python3 "$HOD_OSS_SCRIPTS/save-modules.py" "$hod_tree/logs/census.json" "$HOD_OSS_FINDINGS/llvm" \
  --hod "$hod_tree/bin/hod" >&2
{
  echo "# LLVM 19.1.7 built by kiln with hod: census (WP-178)"
  echo
  echo "Updated $(date -u +%Y-%m-%dT%H:%MZ). hod: $(cat "$hod_tree/logs/hod-version.txt" 2>/dev/null)."
  echo "Trees: \`$hod_tree\` (kiln + hod, fallback allowed, HOD_CENSUS=1) and"
  echo "\`$HOD_OSS_ROOT/llvm-llc\` (kiln + llc, the control). -O0, assertions, AArch64;X86, no -g."
  echo
  echo "## Stages (name, start, end, ninja rc, targets)"
  echo
  for f in hod llc; do
    [ -f "$HOD_OSS_ROOT/llvm-$f/logs/stages.txt" ] || continue
    while read -r st s e rc rest; do
      echo "- $f $st: $((e - s)) s, rc $rc: $rest"
    done < "$HOD_OSS_ROOT/llvm-$f/logs/stages.txt"
  done
  echo
  cat "$hod_tree/logs/census.md"
  echo "## Saved modules (\`$HOD_OSS_FINDINGS/llvm/<reason>/\`: .ll, .cmd.txt, .hod.txt)"
  echo
  for d in "$HOD_OSS_FINDINGS"/llvm/*/; do
    [ -d "$d" ] || continue
    echo "- \`$(basename "$d")\`: $(find "$d" -name '*.ll' -printf '%f ' | sed 's/ $//')"
  done
  echo
  [ -f "$HOD_OSS_FINDINGS/llvm/notes.md" ] && cat "$HOD_OSS_FINDINGS/llvm/notes.md"
} > "$out.tmp"
mv "$out.tmp" "$out"
echo "wrote $out"
