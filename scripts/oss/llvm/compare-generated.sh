#!/bin/bash
# Compare what each tree's llvm-tblgen generated (WP-178): every .inc and
# .def file under build/ of both trees, byte for byte. Prints the counts
# and writes the paths that differ or exist in one tree only to
# logs/generated-<stage>.diff of the hod tree.
#   compare-generated.sh <stage>
set -uo pipefail
source "$(dirname "$0")/env.sh"
stage="${1:?stage}"
h="$HOD_OSS_ROOT/llvm-hod/build" c="$HOD_OSS_ROOT/llvm-llc/build"
out="$HOD_OSS_ROOT/llvm-hod/logs/generated-$stage.diff"
: > "$out"
same=0 differ=0 only=0
while read -r rel; do
  if [ ! -f "$c/$rel" ]; then
    echo "only-hod $rel" >> "$out"; only=$((only + 1))
  elif cmp -s "$h/$rel" "$c/$rel"; then
    same=$((same + 1))
  else
    echo "differ $rel" >> "$out"; differ=$((differ + 1))
  fi
done < <(cd "$h" && find . \( -name '*.inc' -o -name '*.def' -o -name 'Intrinsics*.h' \) -type f -not -path './CMakeFiles/*' | sort)
while read -r rel; do
  [ -f "$h/$rel" ] || { echo "only-control $rel" >> "$out"; only=$((only + 1)); }
done < <(cd "$c" && find . \( -name '*.inc' -o -name '*.def' -o -name 'Intrinsics*.h' \) -type f -not -path './CMakeFiles/*' | sort)
echo "generated files ($stage): same $same, differ $differ, in one tree only $only ($out)"
