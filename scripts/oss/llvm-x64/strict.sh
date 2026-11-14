#!/bin/bash
# Strict rebuild in the hod tree of the x86-64 cross build (WP-178, from
# scripts/oss/llvm/strict.sh): strict.sh <stage> <target>...
# Removes the targets and everything built for them (`ninja -t clean`),
# then builds them again in wrap.sh's strict mode (HOD_OSS_STRICT=1): a
# module hod declines or fails, or a job kiln does not offer to hod, is a
# compile error, not a fallback to llc. With `-k 0` every TU is tried;
# kiln's statistics log has the backend of each (strict-check.py).
#
# HOD_OSS_UNOFFERED=<file>: objects (paths relative to build/, one per
# line) kiln never offers to hod (the BLAKE3 .S files, its
# instrument-entry trigger), built after the clean and before the strict
# build, outside strict mode (stage <stage>-unoffered), so the libraries
# that hold them and every link can be built. The strict build finds them
# up to date. The lines of kiln's statistics log the strict build wrote
# start after line logs/<stage>-stats-start.txt.
#
# The host tools (llvm-tblgen, llvm-min-tblgen) are the frozen native
# copy and are not rebuilt. Links overwrite the qemu launchers of
# build/bin: run qemu-tools.sh hod again before lit.
set -uo pipefail
source "$(dirname "$0")/env.sh"
stage="${1:?usage: strict.sh <stage> <target>...}"
shift
tree="$HOD_OSS_ROOT/${HOD_X64_PREFIX}hod"
ninja -C "$tree/build" -t clean "$@" > "$tree/logs/clean-$stage.log" 2>&1
if [ -n "${HOD_OSS_UNOFFERED:-}" ]; then
  mapfile -t unoffered < "$HOD_OSS_UNOFFERED"
  "$HOD_OSS_SCRIPTS/build.sh" hod "$stage-unoffered" "${unoffered[@]}"
fi
touch "$tree/logs/stats.jsonl"
wc -l < "$tree/logs/stats.jsonl" > "$tree/logs/$stage-stats-start.txt"
HOD_OSS_STRICT=1 exec "$HOD_OSS_SCRIPTS/build.sh" hod "$stage" "$@"
