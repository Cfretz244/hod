#!/bin/bash
# Strict rebuild of a stage in the hod tree (WP-178):
#   strict.sh <stage> <target>...
# Removes the targets and everything built for them (`ninja -t clean`),
# then builds them again in wrap.sh's strict mode (HOD_OSS_STRICT=1): a module hod declines or
# fails is a compile error, not a fallback to llc. rc 0 proves that every
# object of these targets was written by hod (or not offered to it at
# all: kiln's statistics log shows those, with their opt trigger).
#
# HOD_OSS_UNOFFERED=<file>: objects (paths relative to build/, one per
# line) kiln never offers to hod (its instrument-entry trigger), built
# after the clean and before the strict build, outside strict mode
# (stage <stage>-unoffered), so the libraries that hold them and every
# link can be built; the strict build finds them up to date. The lines
# of kiln's statistics log the strict build wrote start after line
# logs/<stage>-stats-start.txt.
set -uo pipefail
source "$(dirname "$0")/env.sh"
stage="${1:?usage: strict.sh <stage> <target>...}"
shift
tree="$HOD_OSS_ROOT/llvm-hod"
ninja -C "$tree/build" -t clean "$@" > "$tree/logs/clean-$stage.log" 2>&1
if [ -n "${HOD_OSS_UNOFFERED:-}" ]; then
  mapfile -t unoffered < "$HOD_OSS_UNOFFERED"
  "$HOD_OSS_SCRIPTS/build.sh" hod "$stage-unoffered" "${unoffered[@]}"
fi
touch "$tree/logs/stats.jsonl"
wc -l < "$tree/logs/stats.jsonl" > "$tree/logs/$stage-stats-start.txt"
HOD_OSS_STRICT=1 exec "$HOD_OSS_SCRIPTS/build.sh" hod "$stage" "$@"
