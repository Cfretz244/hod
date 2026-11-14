#!/bin/bash
# The source of the LLVM build (WP-178): a shallow clone of the tag kiln's
# llvm_clang row pins, into $HOD_OSS_SRC. Does nothing if it is there.
set -euo pipefail
source "$(dirname "$0")/env.sh"
if [ -d "$HOD_OSS_SRC/.git" ]; then
  git -C "$HOD_OSS_SRC" describe --tags --exact-match
  exit 0
fi
mkdir -p "$(dirname "$HOD_OSS_SRC")"
git clone --depth 1 --branch "$LLVM_TAG" https://github.com/llvm/llvm-project "$HOD_OSS_SRC"
