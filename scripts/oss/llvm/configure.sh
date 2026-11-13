#!/bin/bash
# Configure one tree of the LLVM build (WP-178): configure.sh hod|llc [hod]
#
#   hod  $HOD_OSS_ROOT/llvm-hod: kiln with hod (fallback allowed, census on)
#   llc  $HOD_OSS_ROOT/llvm-llc: kiln with its LLVM back end, the control
#
# Both from one source tree ($HOD_OSS_SRC, the shallow clone of
# llvmorg-19.1.7; clone.sh), with the same flags: kiln's llvm_clang row
# (kiln/scripts/oss/projects.toml) without its projects, at -O0 (hod
# only gets -O0 modules) and without -g. For hod, the second argument is
# the hod executable to freeze into the tree's bin/ (default: the release
# hod of this checkout's target directory).
set -euo pipefail
source "$(dirname "$0")/env.sh"
flavour="${1:?usage: configure.sh hod|llc [hod]}"
case "$flavour" in hod | llc) ;; *) echo "configure.sh: hod or llc" >&2; exit 2 ;; esac
tree="$HOD_OSS_ROOT/llvm-$flavour"
mkdir -p "$tree/logs" "$tree/bin"
for n in kiln-$flavour-cxx kiln-$flavour-cc; do ln -sf "$HOD_OSS_SCRIPTS/wrap.sh" "$tree/bin/$n"; done
if [ "$flavour" = hod ]; then
  hod="${2:-${CARGO_TARGET_DIR:?source scripts/env.sh or name the hod executable}/release/hod}"
  cp "$hod" "$tree/bin/hod"
  "$tree/bin/hod" --version > "$tree/logs/hod-version.txt"
fi
"$HOD_OSS_KILN/kiln++" --version > "$tree/logs/kiln-version.txt"
export HOD_OSS_PHASE=configure
cd "$tree"
nice -n 10 cmake --fresh -S "$HOD_OSS_SRC/llvm" -B "$tree/build" -G Ninja \
  -DCMAKE_CXX_COMPILER="$tree/bin/kiln-$flavour-cxx" \
  -DCMAKE_C_COMPILER="$tree/bin/kiln-$flavour-cc" \
  -DCMAKE_CXX_STANDARD=17 \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_CXX_FLAGS_RELEASE=-O0 -DCMAKE_C_FLAGS_RELEASE=-O0 \
  -DLLVM_TARGETS_TO_BUILD="AArch64;X86" \
  -DLLVM_ENABLE_PROJECTS="${HOD_OSS_PROJECTS:-}" \
  -DLLVM_ENABLE_ASSERTIONS=ON \
  -DLLVM_INCLUDE_TESTS=ON \
  -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF \
  -DLLVM_ENABLE_ZSTD=OFF -DLLVM_ENABLE_LIBXML2=OFF \
  -DLLVM_USE_LINKER="${HOD_OSS_LINKER:-mold}" \
  -DLLVM_PARALLEL_LINK_JOBS=1 \
  -DLLVM_LIT_ARGS="-sv -j 4" \
  > "$tree/logs/configure.log" 2>&1
echo "configured $tree/build"
