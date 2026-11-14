#!/bin/bash
# Build, run, compare and delete the gtest unit tests of check-llvm in
# batches (WP-178), so at most one batch of -O0 test binaries is on disk:
#   unit-loop.sh <stage> [batch size, default 8] < list of binaries
# (unit-binaries.txt: `ninja -t query unittests/UnitTests`). The hod
# tree builds in strict mode (HOD_OSS_STRICT=1; gtest-all.cc, which hod
# declines, was built in stage s1 and is not rebuilt), the control as
# usual. Each batch: build.sh in both trees, unittests.sh, delete.
set -uo pipefail
source "$(dirname "$0")/env.sh"
stage="${1:?usage: unit-loop.sh <stage> [batch] < binaries}"
batch="${2:-8}"
mapfile -t all
i=0
while [ $i -lt ${#all[@]} ]; do
  part=("${all[@]:$i:$batch}")
  n=$((i / batch))
  HOD_OSS_STRICT=1 "$HOD_OSS_SCRIPTS/build.sh" hod "$stage-b$n" "${part[@]}"
  "$HOD_OSS_SCRIPTS/build.sh" llc "$stage-b$n" "${part[@]}"
  HOD_OSS_DELETE=1 "$HOD_OSS_SCRIPTS/unittests.sh" "$stage" "${part[@]}"
  i=$((i + batch))
done
