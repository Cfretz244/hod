#!/bin/bash
# Strict rebuild of a stage in the hod tree (WP-178):
#   strict.sh <stage> <target>...
# Removes the targets and everything built for them (`ninja -t clean`),
# then builds them again in wrap.sh's strict mode (HOD_OSS_STRICT=1): a module hod declines or
# fails is a compile error, not a fallback to llc. rc 0 proves that every
# object of these targets was written by hod (or not offered to it at
# all: kiln's statistics log shows those, with their opt trigger).
set -uo pipefail
source "$(dirname "$0")/env.sh"
stage="${1:?usage: strict.sh <stage> <target>...}"
shift
tree="$HOD_OSS_ROOT/llvm-hod"
ninja -C "$tree/build" -t clean "$@" > "$tree/logs/clean-$stage.log" 2>&1
HOD_OSS_STRICT=1 exec "$HOD_OSS_SCRIPTS/build.sh" hod "$stage" "$@"
