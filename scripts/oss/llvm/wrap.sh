#!/bin/sh
# The compiler wrapper of the LLVM build (WP-178), installed under four
# names (symlinks to this file) that choose the flavour and the language:
#
#   kiln-hod-cxx, kiln-hod-cc   kiln++ / kiln with hod as the back end
#                               (KILN_BACKEND=hod; a decline or a failure
#                               falls back to llc unless KILN_HOD_STRICT=1)
#   kiln-llc-cxx, kiln-llc-cc   kiln++ / kiln with its LLVM back end: the
#                               control
#
# It runs kiln's own logging wrapper (kiln's scripts/oss/kiln-cxx and
# kiln-cc: address-space cap, timeout, one JSON line per invocation in
# $KILN_OSS_LOG) with these settings:
#
#   KILN_OSS_LOG      $HOD_OSS_ROOT/llvm-<flavour>/logs/tu.jsonl
#   HOD_COVERAGE_LOG  $HOD_OSS_ROOT/llvm-hod/logs/coverage.tsv (hod only)
#   HOD_CENSUS        1 (hod only): every refusal name of a module
#   KILN_HOD          $HOD_OSS_ROOT/llvm-hod/bin/hod (a frozen copy)
#
# Environment: HOD_OSS_ROOT (required: the directory holding llvm-src,
# llvm-hod, llvm-llc), HOD_OSS_KILN (required: the directory of kiln++
# and kiln), HOD_OSS_KILN_SCRIPTS (required: kiln's scripts/oss),
# HOD_OSS_PHASE (configure | build | test; recorded). KILN_HOD_STRICT
# passes through.
set -eu
: "${HOD_OSS_ROOT:?set HOD_OSS_ROOT}"
: "${HOD_OSS_KILN:?set HOD_OSS_KILN to the directory of kiln++}"
: "${HOD_OSS_KILN_SCRIPTS:?set HOD_OSS_KILN_SCRIPTS to kiln/scripts/oss}"
name=$(basename "$0")
case "$name" in
  kiln-hod-*) flavour=hod ;;
  kiln-llc-*) flavour=llc ;;
  *) echo "wrap.sh: run me as kiln-{hod,llc}-{cxx,cc}, not $name" >&2; exit 2 ;;
esac
tree="$HOD_OSS_ROOT/llvm-$flavour"
mkdir -p "$tree/logs"
export KILN_OSS_LOG="$tree/logs/tu.jsonl"
export KILN_OSS_PHASE="${HOD_OSS_PHASE:-build}"
export KILN_OSS_CXX="$HOD_OSS_KILN/kiln++" KILN_OSS_CC="$HOD_OSS_KILN/kiln"
if [ "$flavour" = hod ]; then
  export KILN_BACKEND=hod KILN_HOD="$tree/bin/hod"
  export HOD_COVERAGE_LOG="$tree/logs/coverage.tsv" HOD_CENSUS=1
else
  export KILN_BACKEND=llvm
  unset KILN_HOD HOD_COVERAGE_LOG HOD_CENSUS
fi
case "$name" in
  *-cxx) exec "$HOD_OSS_KILN_SCRIPTS/kiln-cxx" "$@" ;;
  *-cc) exec "$HOD_OSS_KILN_SCRIPTS/kiln-cc" "$@" ;;
esac
echo "wrap.sh: unknown language in $name" >&2
exit 2
